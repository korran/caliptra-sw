// Licensed under the Apache-2.0 license.

use crate::common::run_rt_test;
use caliptra_builder::{
    firmware::{self, APP_WITH_UART, FMC_WITH_UART},
    ImageOptions,
};
use caliptra_common::mailbox_api::{
    CommandId, EcdsaVerifyReq, FipsVersionResp, FwInfoResp, GetIdevCertReq, GetIdevCertResp,
    GetIdevInfoResp, InvokeDpeReq, InvokeDpeResp, MailboxReq, MailboxReqHeader, MailboxRespHeader,
    PopulateIdevCertReq, StashMeasurementReq, StashMeasurementResp,
};
use caliptra_drivers::{CaliptraError, Ecc384PubKey};
use caliptra_hw_model::{DefaultHwModel, HwModel, ModelError, ShaAccMode};
use caliptra_runtime::{
    FipsVersionCmd, InvokeDpeCmd, RtBootStatus, DPE_SUPPORT, VENDOR_ID, VENDOR_SKU,
};
use dpe::{
    commands::{
        CertifyKeyCmd, CertifyKeyFlags, Command, CommandHdr, DeriveChildCmd, DeriveChildFlags,
        GetCertificateChainCmd, RotateCtxCmd, RotateCtxFlags, SignCmd, SignFlags,
    },
    context::ContextHandle,
    response::{
        CertifyKeyResp, DeriveChildResp, GetCertificateChainResp, GetProfileResp, NewHandleResp,
        SignResp,
    },
    DPE_PROFILE,
};
use openssl::{
    asn1::{Asn1Integer, Asn1Time},
    hash::MessageDigest,
    x509::{X509Name, X509NameBuilder},
};
use openssl::{
    bn::BigNum,
    ec::{EcGroup, EcKey},
    ecdsa::EcdsaSig,
    nid::Nid,
    pkey::{PKey, Private},
    stack::Stack,
    x509::{
        store::X509StoreBuilder, verify::X509VerifyFlags, X509Builder, X509StoreContext,
        X509VerifyResult, X509,
    },
};
use zerocopy::{AsBytes, FromBytes, LayoutVerified};

#[test]
fn test_standard() {
    // Test that the normal runtime firmware boots.
    // Ultimately, this will be useful for exercising Caliptra end-to-end
    // via the mailbox.
    let mut model = run_rt_test(None, None, None);

    model
        .step_until_output_contains("Caliptra RT listening for mailbox commands...")
        .unwrap();
}

#[test]
fn test_update() {
    let image_options = ImageOptions {
        app_version: 0xaabbccdd,
        ..Default::default()
    };
    // Make image to update to. On the FPGA this needs to be done before executing the test,
    // otherwise the test will fail because processor is too busy building to be able to respond to
    // the TRNG call during the initial boot.
    let image =
        caliptra_builder::build_and_sign_image(&FMC_WITH_UART, &APP_WITH_UART, image_options)
            .unwrap()
            .to_bytes()
            .unwrap();

    // Test that the normal runtime firmware boots.
    // Ultimately, this will be useful for exercising Caliptra end-to-end
    // via the mailbox.
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| m.soc_mbox().status().read().mbox_fsm_ps().mbox_idle());

    model
        .mailbox_execute(u32::from(CommandId::FIRMWARE_LOAD), &image)
        .unwrap();

    model
        .step_until_output_contains("Caliptra RT listening for mailbox commands...")
        .unwrap();

    let fw_rev = model.soc_ifc().cptra_fw_rev_id().read();
    assert_eq!(fw_rev[0], 0xaaaaaaaa);
    assert_eq!(fw_rev[1], 0xaabbccdd);
}

#[test]
fn test_boot() {
    let mut model = run_rt_test(Some(&firmware::runtime_tests::BOOT), None, None);

    model.step_until_exit_success().unwrap();
}

#[test]
// Check if the owner and vendor cert validity dates are present in RT Alias cert
fn test_rt_cert_with_custom_dates() {
    const VENDOR_CONFIG: (&str, &str) = ("20250101000000Z", "20260101000000Z");
    const OWNER_CONFIG: (&str, &str) = ("20270101000000Z", "20280101000000Z");

    let mut opts = ImageOptions::default();

    opts.vendor_config
        .not_before
        .copy_from_slice(VENDOR_CONFIG.0.as_bytes());

    opts.vendor_config
        .not_after
        .copy_from_slice(VENDOR_CONFIG.1.as_bytes());

    let mut own_config = opts.owner_config.unwrap();

    own_config
        .not_before
        .copy_from_slice(OWNER_CONFIG.0.as_bytes());
    own_config
        .not_after
        .copy_from_slice(OWNER_CONFIG.1.as_bytes());

    opts.owner_config = Some(own_config);

    let mut model = run_rt_test(Some(&firmware::runtime_tests::CERT), Some(opts), None);

    let rt_resp = model.mailbox_execute(0x3000_0000, &[]).unwrap().unwrap();
    let rt: &[u8] = rt_resp.as_bytes();

    let rt_cert: X509 = X509::from_der(rt).unwrap();

    let not_before: Asn1Time = Asn1Time::from_str(OWNER_CONFIG.0).unwrap();
    let not_after: Asn1Time = Asn1Time::from_str(OWNER_CONFIG.1).unwrap();

    assert!(rt_cert.not_before() == not_before);
    assert!(rt_cert.not_after() == not_after);
}

#[test]
fn test_certs() {
    let mut model = run_rt_test(Some(&firmware::runtime_tests::CERT), None, None);

    // Get certs over the mailbox
    let ldev_resp = model.mailbox_execute(0x1000_0000, &[]).unwrap().unwrap();
    let ldevid: &[u8] = ldev_resp.as_bytes();

    let fmc_resp = model.mailbox_execute(0x2000_0000, &[]).unwrap().unwrap();
    let fmc: &[u8] = fmc_resp.as_bytes();

    let rt_resp = model.mailbox_execute(0x3000_0000, &[]).unwrap().unwrap();
    let rt: &[u8] = rt_resp.as_bytes();

    // Ensure certs are valid X.509
    let ldev_cert: X509 = X509::from_der(ldevid).unwrap();
    let fmc_cert: X509 = X509::from_der(fmc).unwrap();
    let rt_cert: X509 = X509::from_der(rt).unwrap();

    let idev_resp = model.mailbox_execute(0x4000_0000, &[]).unwrap().unwrap();
    let idev_pub = Ecc384PubKey::read_from(idev_resp.as_bytes()).unwrap();

    // Check the LDevID is signed by IDevID
    let group = EcGroup::from_curve_name(Nid::SECP384R1).unwrap();
    let x_bytes: [u8; 48] = idev_pub.x.into();
    let y_bytes: [u8; 48] = idev_pub.y.into();
    let idev_x = &BigNum::from_slice(&x_bytes).unwrap();
    let idev_y = &BigNum::from_slice(&y_bytes).unwrap();

    let idev_ec_key = EcKey::from_public_key_affine_coordinates(&group, idev_x, idev_y).unwrap();
    assert!(ldev_cert
        .verify(&PKey::from_ec_key(idev_ec_key).unwrap())
        .unwrap());

    // Check the FMC is signed by LDevID and that subject/issuer names match
    assert!(fmc_cert.verify(&ldev_cert.public_key().unwrap()).unwrap());
    assert_eq!(
        fmc_cert
            .issuer_name()
            .try_cmp(ldev_cert.subject_name())
            .unwrap(),
        core::cmp::Ordering::Equal
    );

    // Check that RT Alias is signed by FMC and that subject/issuer names match
    assert!(rt_cert.verify(&fmc_cert.public_key().unwrap()).unwrap());
    assert_eq!(
        rt_cert
            .issuer_name()
            .try_cmp(fmc_cert.subject_name())
            .unwrap(),
        core::cmp::Ordering::Equal
    );

    // Verify full cert chain
    let mut roots_bldr = X509StoreBuilder::new().unwrap();
    roots_bldr.add_cert(ldev_cert).unwrap();
    roots_bldr
        .set_flags(X509VerifyFlags::X509_STRICT | X509VerifyFlags::PARTIAL_CHAIN)
        .unwrap();
    let roots = roots_bldr.build();
    let mut cert_store = X509StoreContext::new().unwrap();
    let mut chain = Stack::new().unwrap();
    chain.push(fmc_cert).unwrap();
    cert_store
        .init(&roots, &rt_cert, &chain, |c| {
            let success = c.verify_cert().unwrap();
            assert_eq!(c.error(), X509VerifyResult::OK);
            assert!(success);

            Ok(())
        })
        .unwrap();
}

#[test]
fn test_fw_info() {
    let mut image_opts = ImageOptions::default();
    image_opts.vendor_config.pl0_pauser = Some(0x1);
    image_opts.fmc_version = 0xaaaaaaaa;
    image_opts.app_version = 0xbbbbbbbb;
    image_opts.fmc_svn = 5;

    let mut image_opts10 = image_opts.clone();
    image_opts10.app_svn = 10;

    let mut model = run_rt_test(None, Some(image_opts10), None);

    let get_fwinfo = |model: &mut DefaultHwModel| {
        let payload = MailboxReqHeader {
            chksum: caliptra_common::checksum::calc_checksum(u32::from(CommandId::FW_INFO), &[]),
        };

        let resp = model
            .mailbox_execute(u32::from(CommandId::FW_INFO), payload.as_bytes())
            .unwrap()
            .unwrap();

        let info = FwInfoResp::read_from(resp.as_slice()).unwrap();

        // Verify checksum and FIPS status
        assert!(caliptra_common::checksum::verify_checksum(
            info.hdr.chksum,
            0x0,
            &info.as_bytes()[core::mem::size_of_val(&info.hdr.chksum)..],
        ));
        assert_eq!(
            info.hdr.fips_status,
            MailboxRespHeader::FIPS_STATUS_APPROVED
        );
        assert_eq!(info.attestation_disabled, 0);
        info
    };

    let update_to = |model: &mut DefaultHwModel, image: &[u8]| {
        model
            .mailbox_execute(u32::from(CommandId::FIRMWARE_LOAD), image)
            .unwrap();

        model
            .step_until_output_contains("Caliptra RT listening for mailbox commands...")
            .unwrap();
    };

    let info = get_fwinfo(&mut model);
    // Verify FW info
    assert_eq!(info.pl0_pauser, 0x1);
    assert_eq!(info.fmc_manifest_svn, 5);
    assert_eq!(info.runtime_svn, 10);
    assert_eq!(info.min_runtime_svn, 10);

    // Make image with newer SVN.
    let mut image_opts20 = image_opts.clone();
    image_opts20.app_svn = 20;

    let image20 =
        caliptra_builder::build_and_sign_image(&FMC_WITH_UART, &APP_WITH_UART, image_opts20)
            .unwrap()
            .to_bytes()
            .unwrap();

    // Trigger an update reset.
    update_to(&mut model, &image20);

    let info = get_fwinfo(&mut model);
    assert_eq!(info.runtime_svn, 20);
    assert_eq!(info.min_runtime_svn, 10);

    // Make image with older SVN.
    let mut image_opts5 = image_opts;
    image_opts5.app_svn = 5;

    let image5 =
        caliptra_builder::build_and_sign_image(&FMC_WITH_UART, &APP_WITH_UART, image_opts5)
            .unwrap()
            .to_bytes()
            .unwrap();

    update_to(&mut model, &image5);
    let info = get_fwinfo(&mut model);
    assert_eq!(info.runtime_svn, 5);
    assert_eq!(info.min_runtime_svn, 5);

    // Go back to SVN 20
    update_to(&mut model, &image20);
    let info = get_fwinfo(&mut model);
    assert_eq!(info.runtime_svn, 20);
    assert_eq!(info.min_runtime_svn, 5);
}

#[test]
fn test_stash_measurement() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    let mut cmd = MailboxReq::StashMeasurement(StashMeasurementReq {
        hdr: MailboxReqHeader { chksum: 0 },
        metadata: [0u8; 4],
        measurement: [0u8; 48],
        context: [0u8; 48],
        svn: 0,
    });
    cmd.populate_chksum().unwrap();

    let resp = model
        .mailbox_execute(
            u32::from(CommandId::STASH_MEASUREMENT),
            cmd.as_bytes().unwrap(),
        )
        .unwrap()
        .expect("We should have received a response");

    let resp_hdr: &StashMeasurementResp =
        LayoutVerified::<&[u8], StashMeasurementResp>::new(resp.as_bytes())
            .unwrap()
            .into_ref();

    assert_eq!(resp_hdr.dpe_result, 0);
}

#[test]
fn test_invoke_dpe_get_profile_cmd() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    let mut data = [0u8; InvokeDpeReq::DATA_MAX_SIZE];
    let cmd_hdr = CommandHdr::new_for_test(Command::GET_PROFILE);
    let cmd_hdr_buf = cmd_hdr.as_bytes();
    data[..cmd_hdr_buf.len()].copy_from_slice(cmd_hdr_buf);
    let cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: 0 },
        data,
        data_size: cmd_hdr_buf.len() as u32,
    };

    let checksum = caliptra_common::checksum::calc_checksum(
        u32::from(CommandId::INVOKE_DPE),
        &cmd.as_bytes()[4..],
    );

    let cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: checksum },
        ..cmd
    };

    let resp = model
        .mailbox_execute(u32::from(CommandId::INVOKE_DPE), cmd.as_bytes())
        .unwrap()
        .expect("We should have received a response");

    assert!(resp.len() <= std::mem::size_of::<InvokeDpeResp>());
    let mut resp_hdr = InvokeDpeResp::default();
    resp_hdr.as_bytes_mut()[..resp.len()].copy_from_slice(&resp);

    assert!(caliptra_common::checksum::verify_checksum(
        resp_hdr.hdr.chksum,
        0x0,
        &resp[core::mem::size_of_val(&resp_hdr.hdr.chksum)..],
    ));

    let profile = GetProfileResp::read_from(&resp_hdr.data[..resp_hdr.data_size as usize]).unwrap();
    assert_eq!(profile.resp_hdr.profile, DPE_PROFILE as u32);
    assert_eq!(profile.vendor_id, VENDOR_ID);
    assert_eq!(profile.vendor_sku, VENDOR_SKU);
    assert_eq!(profile.flags, DPE_SUPPORT.bits());

    // Test with data_size too big.
    let mut cmd = MailboxReq::InvokeDpeCommand(InvokeDpeReq {
        data_size: InvokeDpeReq::DATA_MAX_SIZE as u32 + 1,
        ..cmd
    });
    assert_eq!(
        cmd.populate_chksum(),
        Err(caliptra_drivers::CaliptraError::RUNTIME_MAILBOX_API_REQUEST_DATA_LEN_TOO_LARGE)
    );
}

#[test]
fn test_invoke_dpe_get_certificate_chain_cmd() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    let mut data = [0u8; InvokeDpeReq::DATA_MAX_SIZE];
    let get_cert_chain_cmd = GetCertificateChainCmd {
        offset: 0,
        size: 2048,
    };
    let cmd_hdr = CommandHdr::new_for_test(Command::GET_CERTIFICATE_CHAIN);
    let cmd_hdr_buf = cmd_hdr.as_bytes();
    data[..cmd_hdr_buf.len()].copy_from_slice(cmd_hdr_buf);
    let dpe_cmd_buf = get_cert_chain_cmd.as_bytes();
    data[cmd_hdr_buf.len()..cmd_hdr_buf.len() + dpe_cmd_buf.len()].copy_from_slice(dpe_cmd_buf);
    let mut cmd = MailboxReq::InvokeDpeCommand(InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: 0 },
        data,
        data_size: (cmd_hdr_buf.len() + dpe_cmd_buf.len()) as u32,
    });
    cmd.populate_chksum().unwrap();

    let resp = model
        .mailbox_execute(u32::from(CommandId::INVOKE_DPE), cmd.as_bytes().unwrap())
        .unwrap()
        .expect("We should have received a response");

    assert!(resp.len() <= std::mem::size_of::<InvokeDpeResp>());
    let mut resp_hdr = InvokeDpeResp::default();
    resp_hdr.as_bytes_mut()[..resp.len()].copy_from_slice(&resp);

    assert!(caliptra_common::checksum::verify_checksum(
        resp_hdr.hdr.chksum,
        0x0,
        &resp[core::mem::size_of_val(&resp_hdr.hdr.chksum)..],
    ));

    let cert_chain =
        GetCertificateChainResp::read_from(&resp_hdr.data[..resp_hdr.data_size as usize]).unwrap();
    assert_eq!(cert_chain.certificate_size, 2048);
    assert_ne!([0u8; 2048], cert_chain.certificate_chain);
}

fn get_full_cert_chain(model: &mut DefaultHwModel, out: &mut [u8; 4096]) -> usize {
    // first half
    let mut data = [0u8; InvokeDpeReq::DATA_MAX_SIZE];
    let get_cert_chain_cmd = GetCertificateChainCmd {
        offset: 0,
        size: 2048,
    };
    let cmd_hdr = CommandHdr::new_for_test(Command::GET_CERTIFICATE_CHAIN);
    let cmd_hdr_buf = cmd_hdr.as_bytes();
    data[..cmd_hdr_buf.len()].copy_from_slice(cmd_hdr_buf);
    let dpe_cmd_buf = get_cert_chain_cmd.as_bytes();
    data[cmd_hdr_buf.len()..cmd_hdr_buf.len() + dpe_cmd_buf.len()].copy_from_slice(dpe_cmd_buf);
    let mut cmd = MailboxReq::InvokeDpeCommand(InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: 0 },
        data,
        data_size: (cmd_hdr_buf.len() + dpe_cmd_buf.len()) as u32,
    });
    cmd.populate_chksum().unwrap();

    let resp = model
        .mailbox_execute(u32::from(CommandId::INVOKE_DPE), cmd.as_bytes().unwrap())
        .unwrap()
        .expect("We should have received a response");

    let mut resp_hdr = InvokeDpeResp::default();
    resp_hdr.as_bytes_mut()[..resp.len()].copy_from_slice(&resp);

    let cert_chunk_1 =
        GetCertificateChainResp::read_from(&resp_hdr.data[..resp_hdr.data_size as usize]).unwrap();
    out[..cert_chunk_1.certificate_size as usize]
        .copy_from_slice(&cert_chunk_1.certificate_chain[..cert_chunk_1.certificate_size as usize]);

    // second half
    let mut data = [0u8; InvokeDpeReq::DATA_MAX_SIZE];
    let get_cert_chain_cmd = GetCertificateChainCmd {
        offset: cert_chunk_1.certificate_size,
        size: 2048,
    };
    let cmd_hdr = CommandHdr::new_for_test(Command::GET_CERTIFICATE_CHAIN);
    let cmd_hdr_buf = cmd_hdr.as_bytes();
    data[..cmd_hdr_buf.len()].copy_from_slice(cmd_hdr_buf);
    let dpe_cmd_buf = get_cert_chain_cmd.as_bytes();
    data[cmd_hdr_buf.len()..cmd_hdr_buf.len() + dpe_cmd_buf.len()].copy_from_slice(dpe_cmd_buf);
    let mut cmd = MailboxReq::InvokeDpeCommand(InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: 0 },
        data,
        data_size: (cmd_hdr_buf.len() + dpe_cmd_buf.len()) as u32,
    });
    cmd.populate_chksum().unwrap();

    let resp = model
        .mailbox_execute(u32::from(CommandId::INVOKE_DPE), cmd.as_bytes().unwrap())
        .unwrap()
        .expect("We should have received a response");

    let mut resp_hdr = InvokeDpeResp::default();
    resp_hdr.as_bytes_mut()[..resp.len()].copy_from_slice(&resp);

    let cert_chunk_2 =
        GetCertificateChainResp::read_from(&resp_hdr.data[..resp_hdr.data_size as usize]).unwrap();
    out[cert_chunk_1.certificate_size as usize
        ..cert_chunk_1.certificate_size as usize + cert_chunk_2.certificate_size as usize]
        .copy_from_slice(&cert_chunk_2.certificate_chain[..cert_chunk_2.certificate_size as usize]);

    cert_chunk_1.certificate_size as usize + cert_chunk_2.certificate_size as usize
}

// Will panic if any of the cert chain chunks is not a valid X.509 cert
fn parse_cert_chain(cert_chain: &[u8], cert_chain_size: usize, expected_num_certs: u32) {
    let mut i = 0;
    let mut cert_count = 0;
    while i < cert_chain_size {
        let curr_cert = X509::from_der(&cert_chain[i..]).unwrap();
        i += curr_cert.to_der().unwrap().len();
        cert_count += 1;
    }
    assert_eq!(expected_num_certs, cert_count);
}

#[test]
fn test_populate_idev_cert_cmd() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    let mut cert_chain_without_idev_cert = [0u8; 4096];
    let cert_chain_len_without_idev_cert =
        get_full_cert_chain(&mut model, &mut cert_chain_without_idev_cert);

    // generate test idev cert
    let ec_group = EcGroup::from_curve_name(Nid::SECP384R1).unwrap();
    let ec_key = PKey::from_ec_key(EcKey::generate(&ec_group).unwrap()).unwrap();

    let cert = generate_test_x509_cert(ec_key);

    // copy der encoded idev cert
    let cert_bytes = cert.to_der().unwrap();
    let mut cert_slice = [0u8; PopulateIdevCertReq::MAX_CERT_SIZE];
    cert_slice[..cert_bytes.len()].copy_from_slice(&cert_bytes);

    let mut pop_idev_cmd = MailboxReq::PopulateIdevCert(PopulateIdevCertReq {
        hdr: MailboxReqHeader { chksum: 0 },
        cert_size: cert_bytes.len() as u32,
        cert: cert_slice,
    });
    pop_idev_cmd.populate_chksum().unwrap();

    // call populate idev cert so that the idev cert is added to the certificate chain
    model
        .mailbox_execute(
            u32::from(CommandId::POPULATE_IDEV_CERT),
            pop_idev_cmd.as_bytes().unwrap(),
        )
        .unwrap()
        .expect("We should have received a response");

    let mut cert_chain_with_idev_cert = [0u8; 4096];
    let cert_chain_len_with_idev_cert =
        get_full_cert_chain(&mut model, &mut cert_chain_with_idev_cert);

    // read idev cert from prefix of cert chain and parse it as X509
    let idev_len = cert_chain_len_with_idev_cert - cert_chain_len_without_idev_cert;
    let idev_cert = X509::from_der(&cert_chain_with_idev_cert[..idev_len]).unwrap();
    assert_eq!(idev_cert, cert);

    // ensure rest of cert chain is not corrupted
    assert_eq!(
        cert_chain_without_idev_cert[..cert_chain_len_without_idev_cert],
        cert_chain_with_idev_cert[idev_len..cert_chain_len_with_idev_cert]
    );
    parse_cert_chain(
        &cert_chain_with_idev_cert[idev_len..],
        cert_chain_len_with_idev_cert - idev_len,
        3,
    );
}

#[test]
fn test_pauser_privilege_level_dpe_context_thresholds() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    let mut cmd_data: [u8; 512] = [0u8; InvokeDpeReq::DATA_MAX_SIZE];
    // First rotate the default context so that we don't run into an error
    // when trying to retain the default context in derive child.
    let rotate_ctx_cmd = RotateCtxCmd {
        handle: ContextHandle::default(),
        flags: RotateCtxFlags::empty(),
    };
    let rotate_ctx_cmd_hdr = CommandHdr::new_for_test(Command::ROTATE_CONTEXT_HANDLE);
    let rotate_ctx_cmd_hdr_buf = rotate_ctx_cmd_hdr.as_bytes();
    cmd_data[..rotate_ctx_cmd_hdr_buf.len()].copy_from_slice(rotate_ctx_cmd_hdr_buf);
    let rotate_ctx_cmd_buf = rotate_ctx_cmd.as_bytes();
    cmd_data[rotate_ctx_cmd_hdr_buf.len()..rotate_ctx_cmd_hdr_buf.len() + rotate_ctx_cmd_buf.len()]
        .copy_from_slice(rotate_ctx_cmd_buf);
    let rotate_ctx_mbox_cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: 0 },
        data: cmd_data,
        data_size: (rotate_ctx_cmd_hdr_buf.len() + rotate_ctx_cmd_buf.len()) as u32,
    };

    let checksum = caliptra_common::checksum::calc_checksum(
        u32::from(CommandId::INVOKE_DPE),
        &rotate_ctx_mbox_cmd.as_bytes()[4..],
    );

    let rotate_ctx_mbox_cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: checksum },
        ..rotate_ctx_mbox_cmd
    };

    let rotate_ctx_resp_buf = model
        .mailbox_execute(
            u32::from(CommandId::INVOKE_DPE),
            rotate_ctx_mbox_cmd.as_bytes(),
        )
        .unwrap()
        .expect("We should have received a response");

    assert!(rotate_ctx_resp_buf.len() <= std::mem::size_of::<InvokeDpeResp>());
    let mut rotate_ctx_resp_hdr = InvokeDpeResp::default();
    rotate_ctx_resp_hdr.as_bytes_mut()[..rotate_ctx_resp_buf.len()]
        .copy_from_slice(&rotate_ctx_resp_buf);

    assert!(caliptra_common::checksum::verify_checksum(
        rotate_ctx_resp_hdr.hdr.chksum,
        0x0,
        &rotate_ctx_resp_buf[core::mem::size_of_val(&rotate_ctx_resp_hdr.hdr.chksum)..],
    ));

    let rotate_ctx_resp = NewHandleResp::read_from(
        &rotate_ctx_resp_hdr.data[..rotate_ctx_resp_hdr.data_size as usize],
    )
    .unwrap();
    let mut handle = rotate_ctx_resp.handle;

    // Call DeriveChild with PL0 enough times to breach the threshold on the last iteration.
    // Note that this loop runs exactly PL0_DPE_ACTIVE_CONTEXT_THRESHOLD times. When we initialize
    // DPE, we measure mailbox valid pausers in pl0_pauser's locality. Thus, we can call derive child
    // from PL0 exactly 7 times, and the last iteration of this loop, is expected to throw a threshold breached error.
    let num_iterations = InvokeDpeCmd::PL0_DPE_ACTIVE_CONTEXT_THRESHOLD;
    for i in 0..num_iterations {
        let mut cmd_data: [u8; 512] = [0u8; InvokeDpeReq::DATA_MAX_SIZE];
        let derive_child_cmd = DeriveChildCmd {
            handle,
            data: [0u8; DPE_PROFILE.get_hash_size()],
            flags: DeriveChildFlags::RETAIN_PARENT,
            tci_type: 0,
            target_locality: 0,
        };
        let derive_child_cmd_hdr = CommandHdr::new_for_test(Command::DERIVE_CHILD);
        let derive_child_cmd_hdr_buf = derive_child_cmd_hdr.as_bytes();
        cmd_data[..derive_child_cmd_hdr_buf.len()].copy_from_slice(derive_child_cmd_hdr_buf);
        let derive_child_cmd_buf = derive_child_cmd.as_bytes();
        cmd_data[derive_child_cmd_hdr_buf.len()
            ..derive_child_cmd_hdr_buf.len() + derive_child_cmd_buf.len()]
            .copy_from_slice(derive_child_cmd_buf);
        let derive_child_mbox_cmd = InvokeDpeReq {
            hdr: MailboxReqHeader { chksum: 0 },
            data: cmd_data,
            data_size: (derive_child_cmd_hdr_buf.len() + derive_child_cmd_buf.len()) as u32,
        };

        let checksum = caliptra_common::checksum::calc_checksum(
            u32::from(CommandId::INVOKE_DPE),
            &derive_child_mbox_cmd.as_bytes()[4..],
        );

        let derive_child_mbox_cmd = InvokeDpeReq {
            hdr: MailboxReqHeader { chksum: checksum },
            ..derive_child_mbox_cmd
        };

        // If we are on the last call to DeriveChild, expect that we get a PL0_USED_DPE_CONTEXT_THRESHOLD_EXCEEDED error.
        if i == num_iterations - 1 {
            let resp = model
                .mailbox_execute(
                    u32::from(CommandId::INVOKE_DPE),
                    derive_child_mbox_cmd.as_bytes(),
                )
                .unwrap_err();
            if let ModelError::MailboxCmdFailed(code) = resp {
                assert_eq!(
                    code,
                    u32::from(caliptra_drivers::CaliptraError::RUNTIME_PL0_USED_DPE_CONTEXT_THRESHOLD_EXCEEDED)
                );
            } else {
                panic!("This DeriveChild call should have failed since it would have breached the PL0 non-inactive dpe context threshold.")
            }
            break;
        }

        let derive_child_resp_buf = model
            .mailbox_execute(
                u32::from(CommandId::INVOKE_DPE),
                derive_child_mbox_cmd.as_bytes(),
            )
            .unwrap()
            .expect("We should have received a response");

        assert!(derive_child_resp_buf.len() <= std::mem::size_of::<InvokeDpeResp>());
        let mut derive_child_resp_hdr = InvokeDpeResp::default();
        derive_child_resp_hdr.as_bytes_mut()[..derive_child_resp_buf.len()]
            .copy_from_slice(&derive_child_resp_buf);

        assert!(caliptra_common::checksum::verify_checksum(
            derive_child_resp_hdr.hdr.chksum,
            0x0,
            &derive_child_resp_buf[core::mem::size_of_val(&derive_child_resp_hdr.hdr.chksum)..],
        ));

        let derive_child_resp = DeriveChildResp::read_from(
            &derive_child_resp_hdr.data[..derive_child_resp_hdr.data_size as usize],
        )
        .unwrap();
        handle = derive_child_resp.handle;
    }
}

#[test]
fn test_invoke_dpe_sign_and_certify_key_cmds() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    let test_label = [
        48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26,
        25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
    ];
    let test_digest = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    ];
    let mut data = [0u8; InvokeDpeReq::DATA_MAX_SIZE];
    let sign_cmd = SignCmd {
        handle: ContextHandle::default(),
        label: test_label,
        flags: SignFlags::empty(),
        digest: test_digest,
    };
    let sign_cmd_hdr = CommandHdr::new_for_test(Command::SIGN);
    let sign_cmd_hdr_buf = sign_cmd_hdr.as_bytes();
    data[..sign_cmd_hdr_buf.len()].copy_from_slice(sign_cmd_hdr_buf);
    let sign_cmd_buf = sign_cmd.as_bytes();
    data[sign_cmd_hdr_buf.len()..sign_cmd_hdr_buf.len() + sign_cmd_buf.len()]
        .copy_from_slice(sign_cmd_buf);
    let sign_mbox_cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: 0 },
        data,
        data_size: (sign_cmd_hdr_buf.len() + sign_cmd_buf.len()) as u32,
    };

    let checksum = caliptra_common::checksum::calc_checksum(
        u32::from(CommandId::INVOKE_DPE),
        &sign_mbox_cmd.as_bytes()[4..],
    );

    let sign_mbox_cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: checksum },
        ..sign_mbox_cmd
    };

    let sign_resp_buf = model
        .mailbox_execute(u32::from(CommandId::INVOKE_DPE), sign_mbox_cmd.as_bytes())
        .unwrap()
        .expect("We should have received a response");

    assert!(sign_resp_buf.len() <= std::mem::size_of::<InvokeDpeResp>());
    let mut sign_resp_hdr = InvokeDpeResp::default();
    sign_resp_hdr.as_bytes_mut()[..sign_resp_buf.len()].copy_from_slice(&sign_resp_buf);

    assert!(caliptra_common::checksum::verify_checksum(
        sign_resp_hdr.hdr.chksum,
        0x0,
        &sign_resp_buf[core::mem::size_of_val(&sign_resp_hdr.hdr.chksum)..],
    ));

    let sign_resp =
        SignResp::read_from(&sign_resp_hdr.data[..sign_resp_hdr.data_size as usize]).unwrap();

    let certify_key_cmd = CertifyKeyCmd {
        handle: ContextHandle::default(),
        label: test_label,
        flags: CertifyKeyFlags::empty(),
        format: CertifyKeyCmd::FORMAT_X509,
    };
    let certify_key_cmd_hdr = CommandHdr::new_for_test(Command::CERTIFY_KEY);
    let certify_key_cmd_hdr_buf = certify_key_cmd_hdr.as_bytes();
    data[..certify_key_cmd_hdr_buf.len()].copy_from_slice(certify_key_cmd_hdr_buf);
    let certify_key_cmd_buf = certify_key_cmd.as_bytes();
    data[certify_key_cmd_hdr_buf.len()..certify_key_cmd_hdr_buf.len() + certify_key_cmd_buf.len()]
        .copy_from_slice(certify_key_cmd_buf);
    let certify_key_mbox_cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: 0 },
        data,
        data_size: (certify_key_cmd_hdr_buf.len() + certify_key_cmd_buf.len()) as u32,
    };

    let checksum = caliptra_common::checksum::calc_checksum(
        u32::from(CommandId::INVOKE_DPE),
        &certify_key_mbox_cmd.as_bytes()[4..],
    );

    let certify_key_mbox_cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: checksum },
        ..certify_key_mbox_cmd
    };

    let certify_key_resp_buf = model
        .mailbox_execute(
            u32::from(CommandId::INVOKE_DPE),
            certify_key_mbox_cmd.as_bytes(),
        )
        .unwrap()
        .expect("We should have received a response");

    assert!(certify_key_resp_buf.len() <= std::mem::size_of::<InvokeDpeResp>());
    let mut certify_key_resp_hdr = InvokeDpeResp::default();
    certify_key_resp_hdr.as_bytes_mut()[..certify_key_resp_buf.len()]
        .copy_from_slice(&certify_key_resp_buf);

    assert!(caliptra_common::checksum::verify_checksum(
        certify_key_resp_hdr.hdr.chksum,
        0x0,
        &certify_key_resp_buf[core::mem::size_of_val(&certify_key_resp_hdr.hdr.chksum)..],
    ));

    let certify_key_resp = CertifyKeyResp::read_from(
        &certify_key_resp_hdr.data[..certify_key_resp_hdr.data_size as usize],
    )
    .unwrap();

    let sig = EcdsaSig::from_private_components(
        BigNum::from_slice(&sign_resp.sig_r_or_hmac).unwrap(),
        BigNum::from_slice(&sign_resp.sig_s).unwrap(),
    )
    .unwrap();

    let ecc_pub_key = EcKey::from_public_key_affine_coordinates(
        &EcGroup::from_curve_name(Nid::SECP384R1).unwrap(),
        &BigNum::from_slice(&certify_key_resp.derived_pubkey_x).unwrap(),
        &BigNum::from_slice(&certify_key_resp.derived_pubkey_y).unwrap(),
    )
    .unwrap();
    assert!(sig.verify(&test_digest, &ecc_pub_key).unwrap());
}

#[test]
fn test_invoke_dpe_symmetric_sign() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    let test_label = [
        48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26,
        25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
    ];
    let test_digest = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    ];
    let mut data = [0u8; InvokeDpeReq::DATA_MAX_SIZE];
    let sign_cmd = SignCmd {
        handle: ContextHandle::default(),
        label: test_label,
        flags: SignFlags::IS_SYMMETRIC,
        digest: test_digest,
    };
    let sign_cmd_hdr = CommandHdr::new_for_test(Command::SIGN);
    let sign_cmd_hdr_buf = sign_cmd_hdr.as_bytes();
    data[..sign_cmd_hdr_buf.len()].copy_from_slice(sign_cmd_hdr_buf);
    let sign_cmd_buf = sign_cmd.as_bytes();
    data[sign_cmd_hdr_buf.len()..sign_cmd_hdr_buf.len() + sign_cmd_buf.len()]
        .copy_from_slice(sign_cmd_buf);
    let sign_mbox_cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: 0 },
        data,
        data_size: (sign_cmd_hdr_buf.len() + sign_cmd_buf.len()) as u32,
    };

    let checksum = caliptra_common::checksum::calc_checksum(
        u32::from(CommandId::INVOKE_DPE),
        &sign_mbox_cmd.as_bytes()[4..],
    );

    let sign_mbox_cmd = InvokeDpeReq {
        hdr: MailboxReqHeader { chksum: checksum },
        ..sign_mbox_cmd
    };

    let sign_resp_buf = model
        .mailbox_execute(u32::from(CommandId::INVOKE_DPE), sign_mbox_cmd.as_bytes())
        .unwrap()
        .expect("We should have received a response");

    assert!(sign_resp_buf.len() <= std::mem::size_of::<InvokeDpeResp>());
    let mut sign_resp_hdr = InvokeDpeResp::default();
    sign_resp_hdr.as_bytes_mut()[..sign_resp_buf.len()].copy_from_slice(&sign_resp_buf);

    assert!(caliptra_common::checksum::verify_checksum(
        sign_resp_hdr.hdr.chksum,
        0x0,
        &sign_resp_buf[core::mem::size_of_val(&sign_resp_hdr.hdr.chksum)..],
    ));

    let sign_resp =
        SignResp::read_from(&sign_resp_hdr.data[..sign_resp_hdr.data_size as usize]).unwrap();
    // r contains the hmac so it should not be all 0s
    assert_ne!(sign_resp.sig_r_or_hmac, [0u8; 48]);
    // s must be all 0s for hmac sign
    assert_eq!(sign_resp.sig_s, [0u8; 48]);
}

#[test]
fn test_disable_attestation_cmd() {
    let mut model = run_rt_test(None, None, None);

    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(
            u32::from(CommandId::DISABLE_ATTESTATION),
            &[],
        ),
    };
    // once DPE APIs are enabled, ensure that the RT alias key in the cert is different from the key that signs DPE certs
    let resp = model
        .mailbox_execute(
            u32::from(CommandId::DISABLE_ATTESTATION),
            payload.as_bytes(),
        )
        .unwrap()
        .unwrap();

    let resp_hdr = MailboxRespHeader::read_from(resp.as_bytes()).unwrap();
    assert_eq!(
        resp_hdr.fips_status,
        MailboxRespHeader::FIPS_STATUS_APPROVED
    );
}

#[test]
fn test_ecdsa_verify_cmd() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    // Message to hash
    let msg: &[u8] = &[
        0x9d, 0xd7, 0x89, 0xea, 0x25, 0xc0, 0x47, 0x45, 0xd5, 0x7a, 0x38, 0x1f, 0x22, 0xde, 0x01,
        0xfb, 0x0a, 0xbd, 0x3c, 0x72, 0xdb, 0xde, 0xfd, 0x44, 0xe4, 0x32, 0x13, 0xc1, 0x89, 0x58,
        0x3e, 0xef, 0x85, 0xba, 0x66, 0x20, 0x44, 0xda, 0x3d, 0xe2, 0xdd, 0x86, 0x70, 0xe6, 0x32,
        0x51, 0x54, 0x48, 0x01, 0x55, 0xbb, 0xee, 0xbb, 0x70, 0x2c, 0x75, 0x78, 0x1a, 0xc3, 0x2e,
        0x13, 0x94, 0x18, 0x60, 0xcb, 0x57, 0x6f, 0xe3, 0x7a, 0x05, 0xb7, 0x57, 0xda, 0x5b, 0x5b,
        0x41, 0x8f, 0x6d, 0xd7, 0xc3, 0x0b, 0x04, 0x2e, 0x40, 0xf4, 0x39, 0x5a, 0x34, 0x2a, 0xe4,
        0xdc, 0xe0, 0x56, 0x34, 0xc3, 0x36, 0x25, 0xe2, 0xbc, 0x52, 0x43, 0x45, 0x48, 0x1f, 0x7e,
        0x25, 0x3d, 0x95, 0x51, 0x26, 0x68, 0x23, 0x77, 0x1b, 0x25, 0x17, 0x05, 0xb4, 0xa8, 0x51,
        0x66, 0x02, 0x2a, 0x37, 0xac, 0x28, 0xf1, 0xbd,
    ];

    // Stream to SHA ACC
    model
        .compute_sha512_acc_digest(msg, ShaAccMode::Sha384Stream)
        .unwrap();

    // ECDSAVS NIST test vector
    let cmd = EcdsaVerifyReq {
        hdr: MailboxReqHeader { chksum: 0 },
        pub_key_x: [
            0xcb, 0x90, 0x8b, 0x1f, 0xd5, 0x16, 0xa5, 0x7b, 0x8e, 0xe1, 0xe1, 0x43, 0x83, 0x57,
            0x9b, 0x33, 0xcb, 0x15, 0x4f, 0xec, 0xe2, 0x0c, 0x50, 0x35, 0xe2, 0xb3, 0x76, 0x51,
            0x95, 0xd1, 0x95, 0x1d, 0x75, 0xbd, 0x78, 0xfb, 0x23, 0xe0, 0x0f, 0xef, 0x37, 0xd7,
            0xd0, 0x64, 0xfd, 0x9a, 0xf1, 0x44,
        ],
        pub_key_y: [
            0xcd, 0x99, 0xc4, 0x6b, 0x58, 0x57, 0x40, 0x1d, 0xdc, 0xff, 0x2c, 0xf7, 0xcf, 0x82,
            0x21, 0x21, 0xfa, 0xf1, 0xcb, 0xad, 0x9a, 0x01, 0x1b, 0xed, 0x8c, 0x55, 0x1f, 0x6f,
            0x59, 0xb2, 0xc3, 0x60, 0xf7, 0x9b, 0xfb, 0xe3, 0x2a, 0xdb, 0xca, 0xa0, 0x95, 0x83,
            0xbd, 0xfd, 0xf7, 0xc3, 0x74, 0xbb,
        ],
        signature_r: [
            0x33, 0xf6, 0x4f, 0xb6, 0x5c, 0xd6, 0xa8, 0x91, 0x85, 0x23, 0xf2, 0x3a, 0xea, 0x0b,
            0xbc, 0xf5, 0x6b, 0xba, 0x1d, 0xac, 0xa7, 0xaf, 0xf8, 0x17, 0xc8, 0x79, 0x1d, 0xc9,
            0x24, 0x28, 0xd6, 0x05, 0xac, 0x62, 0x9d, 0xe2, 0xe8, 0x47, 0xd4, 0x3c, 0xee, 0x55,
            0xba, 0x9e, 0x4a, 0x0e, 0x83, 0xba,
        ],
        signature_s: [
            0x44, 0x28, 0xbb, 0x47, 0x8a, 0x43, 0xac, 0x73, 0xec, 0xd6, 0xde, 0x51, 0xdd, 0xf7,
            0xc2, 0x8f, 0xf3, 0xc2, 0x44, 0x16, 0x25, 0xa0, 0x81, 0x71, 0x43, 0x37, 0xdd, 0x44,
            0xfe, 0xa8, 0x01, 0x1b, 0xae, 0x71, 0x95, 0x9a, 0x10, 0x94, 0x7b, 0x6e, 0xa3, 0x3f,
            0x77, 0xe1, 0x28, 0xd3, 0xc6, 0xae,
        ],
    };

    let checksum = caliptra_common::checksum::calc_checksum(
        u32::from(CommandId::ECDSA384_VERIFY),
        &cmd.as_bytes()[4..],
    );

    let cmd = EcdsaVerifyReq {
        hdr: MailboxReqHeader { chksum: checksum },
        ..cmd
    };

    let resp = model
        .mailbox_execute(u32::from(CommandId::ECDSA384_VERIFY), cmd.as_bytes())
        .unwrap()
        .expect("We should have received a response");

    let resp_hdr: &MailboxRespHeader =
        LayoutVerified::<&[u8], MailboxRespHeader>::new(resp.as_bytes())
            .unwrap()
            .into_ref();

    assert_eq!(
        resp_hdr.fips_status,
        MailboxRespHeader::FIPS_STATUS_APPROVED
    );
    // Checksum is just going to be 0 because FIPS_STATUS_APPROVED is 0
    assert_eq!(resp_hdr.chksum, 0);
    assert_eq!(model.soc_ifc().cptra_fw_error_non_fatal().read(), 0);

    // Test with a bad reqest chksum
    let cmd = EcdsaVerifyReq {
        hdr: MailboxReqHeader { chksum: 0 },
        ..cmd
    };

    // Make sure the command execution fails.
    let resp = model
        .mailbox_execute(u32::from(CommandId::ECDSA384_VERIFY), cmd.as_bytes())
        .unwrap_err();
    if let ModelError::MailboxCmdFailed(code) = resp {
        assert_eq!(
            code,
            u32::from(caliptra_drivers::CaliptraError::RUNTIME_INVALID_CHECKSUM)
        );
    }
    assert_eq!(
        model.soc_ifc().cptra_fw_error_non_fatal().read(),
        u32::from(caliptra_drivers::CaliptraError::RUNTIME_INVALID_CHECKSUM)
    );
}

#[test]
fn test_fips_cmd_api() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| m.soc_mbox().status().read().mbox_fsm_ps().mbox_idle());

    // VERSION
    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(u32::from(CommandId::VERSION), &[]),
    };

    let fips_version_resp = model
        .mailbox_execute(u32::from(CommandId::VERSION), payload.as_bytes())
        .unwrap()
        .unwrap();

    // Check command size
    let fips_version_bytes: &[u8] = fips_version_resp.as_bytes();

    // Check values against expected.
    let fips_version = FipsVersionResp::read_from(fips_version_bytes).unwrap();
    assert!(caliptra_common::checksum::verify_checksum(
        fips_version.hdr.chksum,
        0x0,
        &fips_version.as_bytes()[core::mem::size_of_val(&fips_version.hdr.chksum)..],
    ));
    assert_eq!(
        fips_version.hdr.fips_status,
        MailboxRespHeader::FIPS_STATUS_APPROVED
    );
    assert_eq!(fips_version.mode, FipsVersionCmd::MODE);
    assert_eq!(fips_version.fips_rev, [0x01, 0xaaaaaaaa, 0xbbbbbbbb]);
    let name = &fips_version.name[..];
    assert_eq!(name, FipsVersionCmd::NAME.as_bytes());

    // SHUTDOWN
    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(u32::from(CommandId::SHUTDOWN), &[]),
    };

    let resp = model.mailbox_execute(u32::from(CommandId::SHUTDOWN), payload.as_bytes());
    assert!(resp.is_ok());

    // Check we are rejecting additional commands with the shutdown error code.
    let expected_err = Err(ModelError::MailboxCmdFailed(0x000E0008));
    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(u32::from(CommandId::VERSION), &[]),
    };
    let resp = model.mailbox_execute(u32::from(CommandId::VERSION), payload.as_bytes());
    assert_eq!(resp, expected_err);
}

/// When a successful command runs after a failed command, ensure the error
/// register is cleared.
#[test]
fn test_error_cleared() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| m.soc_mbox().status().read().mbox_fsm_ps().mbox_idle());

    // Send invalid command to cause failure
    let resp = model.mailbox_execute(0xffffffff, &[]);
    assert_eq!(
        resp,
        Err(ModelError::MailboxCmdFailed(
            CaliptraError::RUNTIME_MAILBOX_INVALID_PARAMS.into()
        ))
    );

    // Succeed a command to make sure error gets cleared
    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(u32::from(CommandId::VERSION), &[]),
    };
    let _ = model
        .mailbox_execute(u32::from(CommandId::VERSION), payload.as_bytes())
        .unwrap()
        .unwrap();

    assert_eq!(model.soc_ifc().cptra_fw_error_non_fatal().read(), 0);
}

#[test]
fn test_fw_version() {
    let mut model = run_rt_test(None, None, None);
    model.step_until(|m| {
        m.soc_ifc().cptra_boot_status().read() == u32::from(RtBootStatus::RtReadyForCommands)
    });

    let fw_rev = model.soc_ifc().cptra_fw_rev_id().read();
    assert_eq!(fw_rev[0], 0xaaaaaaaa);
    assert_eq!(fw_rev[1], 0xbbbbbbbb);
}

#[test]
fn test_unimplemented_cmds() {
    let mut model = run_rt_test(None, None, None);

    model.step_until(|m| m.soc_mbox().status().read().mbox_fsm_ps().mbox_idle());

    let expected_err = Err(ModelError::MailboxCmdFailed(0xe0002));

    // GET_IDEV_CSR
    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(u32::from(CommandId::GET_IDEV_CSR), &[]),
    };

    let mut resp = model.mailbox_execute(u32::from(CommandId::GET_IDEV_CSR), payload.as_bytes());
    assert_eq!(resp, expected_err);

    // GET_LDEV_CERT
    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(u32::from(CommandId::GET_LDEV_CERT), &[]),
    };

    resp = model.mailbox_execute(u32::from(CommandId::GET_LDEV_CERT), payload.as_bytes());
    assert_eq!(resp, expected_err);

    // Send something that is not a valid RT command.
    let expected_err = Err(ModelError::MailboxCmdFailed(0xe0002));
    const INVALID_CMD: u32 = 0xAABBCCDD;
    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(INVALID_CMD, &[]),
    };

    let resp = model.mailbox_execute(INVALID_CMD, payload.as_bytes());
    assert_eq!(resp, expected_err);
}

#[test]
fn test_idev_id_info() {
    let mut model = run_rt_test(None, None, None);

    let payload = MailboxReqHeader {
        chksum: caliptra_common::checksum::calc_checksum(u32::from(CommandId::GET_IDEV_INFO), &[]),
    };

    let resp = model
        .mailbox_execute(u32::from(CommandId::GET_IDEV_INFO), payload.as_bytes())
        .unwrap()
        .unwrap();

    GetIdevInfoResp::read_from(resp.as_slice()).unwrap();
}

fn generate_test_x509_cert(ec_key: PKey<Private>) -> X509 {
    let mut cert_builder = X509Builder::new().unwrap();
    cert_builder.set_version(2).unwrap();
    cert_builder
        .set_serial_number(&Asn1Integer::from_bn(&BigNum::from_u32(1).unwrap()).unwrap())
        .unwrap();
    let mut subj_name_builder = X509Name::builder().unwrap();
    subj_name_builder
        .append_entry_by_text("CN", "example.com")
        .unwrap();
    let subject_name = X509NameBuilder::build(subj_name_builder);
    cert_builder.set_subject_name(&subject_name).unwrap();
    cert_builder.set_issuer_name(&subject_name).unwrap();
    cert_builder.set_pubkey(&ec_key).unwrap();
    cert_builder
        .set_not_before(&Asn1Time::days_from_now(0).unwrap())
        .unwrap();
    cert_builder
        .set_not_after(&Asn1Time::days_from_now(365).unwrap())
        .unwrap();
    cert_builder.sign(&ec_key, MessageDigest::sha384()).unwrap();
    cert_builder.build()
}

#[test]
fn test_idev_id_cert() {
    let mut model = run_rt_test(None, None, None);

    // generate 48 byte ECDSA key pair
    let ec_group = EcGroup::from_curve_name(Nid::SECP384R1).unwrap();
    let ec_key = PKey::from_ec_key(EcKey::generate(&ec_group).unwrap()).unwrap();

    let cert = generate_test_x509_cert(ec_key.clone());
    assert!(cert.verify(&ec_key).unwrap());

    // Extract the r and s values of the signature
    let sig_bytes = cert.signature().as_slice();
    let signature = EcdsaSig::from_der(sig_bytes).unwrap();
    let signature_r: [u8; 48] = signature.r().to_vec_padded(48).unwrap().try_into().unwrap();
    let signature_s: [u8; 48] = signature.s().to_vec_padded(48).unwrap().try_into().unwrap();

    // Extract tbs from cert
    let mut tbs = [0u8; GetIdevCertReq::DATA_MAX_SIZE];
    let cert_der_vec = cert.to_der().unwrap();
    let cert_der = cert_der_vec.as_bytes();
    // skip first 4 outer sequence bytes
    let tbs_offset = 4;
    // this value is hard-coded and will need to be changed if the above x509 encoding is ever changed
    // you can change it by calling asn1parse on the byte dump of the x509 cert, and finding the size of the TbsCertificate portion
    let tbs_size = 223;
    tbs[..tbs_size].copy_from_slice(&cert_der[tbs_offset..tbs_offset + tbs_size]);

    let mut cmd = MailboxReq::GetIdevCert(GetIdevCertReq {
        hdr: MailboxReqHeader { chksum: 0 },
        tbs,
        signature_r,
        signature_s,
        tbs_size: tbs_size as u32,
    });
    cmd.populate_chksum().unwrap();

    let resp = model
        .mailbox_execute(u32::from(CommandId::GET_IDEV_CERT), cmd.as_bytes().unwrap())
        .unwrap()
        .expect("We expected a response");

    assert!(resp.len() <= std::mem::size_of::<GetIdevCertResp>());
    let mut cert = GetIdevCertResp::default();
    cert.as_bytes_mut()[..resp.len()].copy_from_slice(&resp);

    assert!(caliptra_common::checksum::verify_checksum(
        cert.hdr.chksum,
        0x0,
        &resp[core::mem::size_of_val(&cert.hdr.chksum)..],
    ));

    assert!(tbs_size < cert.cert_size as usize);
    let idev_cert = X509::from_der(&cert.cert[..cert.cert_size as usize]).unwrap();
    assert!(idev_cert.verify(&ec_key).unwrap());

    // Test with tbs_size too big.
    let mut cmd = MailboxReq::GetIdevCert(GetIdevCertReq {
        hdr: MailboxReqHeader { chksum: 0 },
        tbs,
        signature_r,
        signature_s,
        tbs_size: GetIdevCertReq::DATA_MAX_SIZE as u32 + 1,
    });
    assert_eq!(
        cmd.populate_chksum(),
        Err(caliptra_drivers::CaliptraError::RUNTIME_MAILBOX_API_REQUEST_DATA_LEN_TOO_LARGE)
    );
}
