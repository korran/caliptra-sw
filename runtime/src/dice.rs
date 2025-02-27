// Licensed under the Apache-2.0 license

#[cfg(feature = "test_only_commands")]
use caliptra_common::mailbox_api::{
    GetLdevCertResp, MailboxResp, MailboxRespHeader, TestGetFmcAliasCertResp,
};

#[cfg(feature = "test_only_commands")]
use crate::Drivers;

use caliptra_drivers::{
    hand_off::DataStore, CaliptraError, CaliptraResult, DataVault, Ecc384Scalar, Ecc384Signature,
    PersistentData,
};
use caliptra_x509::{Ecdsa384CertBuilder, Ecdsa384Signature};

pub struct GetLdevCertCmd;
impl GetLdevCertCmd {
    #[cfg(feature = "test_only_commands")]
    pub(crate) fn execute(drivers: &mut Drivers) -> CaliptraResult<MailboxResp> {
        let mut resp = GetLdevCertResp::default();

        resp.data_size = copy_ldevid_cert(
            &drivers.data_vault,
            drivers.persistent_data.get(),
            &mut resp.data,
        )? as u32;

        Ok(MailboxResp::GetLdevCert(resp))
    }
}

pub struct TestGetFmcAliasCertCmd;
impl TestGetFmcAliasCertCmd {
    #[cfg(feature = "test_only_commands")]
    pub(crate) fn execute(drivers: &mut Drivers) -> CaliptraResult<MailboxResp> {
        let mut resp = TestGetFmcAliasCertResp::default();

        resp.data_size = copy_fmc_alias_cert(
            &drivers.data_vault,
            drivers.persistent_data.get(),
            &mut resp.data,
        )? as u32;

        Ok(MailboxResp::TestGetFmcAliasCert(resp))
    }
}

// Retrieve the r portion of the LDevId cert signature
fn ldevid_dice_sign_r(
    persistent_data: &PersistentData,
    dv: &DataVault,
) -> CaliptraResult<Ecc384Scalar> {
    let ds: DataStore = persistent_data
        .fht
        .ldevid_cert_sig_r_dv_hdl
        .try_into()
        .map_err(|_| CaliptraError::RUNTIME_FMC_CERT_HANDOFF_FAILED)?;

    // The data store is either a warm reset entry or a cold reset entry.
    match ds {
        DataStore::DataVaultNonSticky48(dv_entry) => Ok(dv.read_warm_reset_entry48(dv_entry)),
        DataStore::DataVaultSticky48(dv_entry) => Ok(dv.read_cold_reset_entry48(dv_entry)),
        _ => Err(CaliptraError::RUNTIME_FMC_CERT_HANDOFF_FAILED),
    }
}

// Retrieve the s portion of the LDevId cert signature
fn ldevid_dice_sign_s(
    persistent_data: &PersistentData,
    dv: &DataVault,
) -> CaliptraResult<Ecc384Scalar> {
    let ds: DataStore = persistent_data
        .fht
        .ldevid_cert_sig_s_dv_hdl
        .try_into()
        .map_err(|_| CaliptraError::RUNTIME_FMC_CERT_HANDOFF_FAILED)?;

    // The data store is either a warm reset entry or a cold reset entry.
    match ds {
        DataStore::DataVaultNonSticky48(dv_entry) => Ok(dv.read_warm_reset_entry48(dv_entry)),
        DataStore::DataVaultSticky48(dv_entry) => Ok(dv.read_cold_reset_entry48(dv_entry)),
        _ => Err(CaliptraError::RUNTIME_FMC_CERT_HANDOFF_FAILED),
    }
}

// Piece together the r and s portions of the LDevId cert signature
pub fn ldevid_dice_sign(
    persistent_data: &PersistentData,
    dv: &DataVault,
) -> CaliptraResult<Ecc384Signature> {
    Ok(Ecc384Signature {
        r: ldevid_dice_sign_r(persistent_data, dv)?,
        s: ldevid_dice_sign_s(persistent_data, dv)?,
    })
}

/// Copy LDevID certificate produced by ROM to `cert` buffer
///
/// Returns the number of bytes written to `cert`
#[inline(never)]
pub fn copy_ldevid_cert(
    dv: &DataVault,
    persistent_data: &PersistentData,
    cert: &mut [u8],
) -> CaliptraResult<usize> {
    let tbs = persistent_data
        .ldevid_tbs
        .get(..persistent_data.fht.ldevid_tbs_size.into());
    let sig = ldevid_dice_sign(persistent_data, dv)?;
    cert_from_tbs_and_sig(tbs, &sig, cert)
}

// Retrieve the r portion of the FMC cert signature
fn fmc_dice_sign_r(
    persistent_data: &PersistentData,
    dv: &DataVault,
) -> CaliptraResult<Ecc384Scalar> {
    let ds: DataStore = persistent_data
        .fht
        .fmc_cert_sig_r_dv_hdl
        .try_into()
        .map_err(|_| CaliptraError::RUNTIME_FMC_CERT_HANDOFF_FAILED)?;

    // The data store is either a warm reset entry or a cold reset entry.
    match ds {
        DataStore::DataVaultNonSticky48(dv_entry) => Ok(dv.read_warm_reset_entry48(dv_entry)),
        DataStore::DataVaultSticky48(dv_entry) => Ok(dv.read_cold_reset_entry48(dv_entry)),
        _ => Err(CaliptraError::RUNTIME_FMC_CERT_HANDOFF_FAILED),
    }
}

// Retrieve the s portion of the FMC cert signature
fn fmc_dice_sign_s(
    persistent_data: &PersistentData,
    dv: &DataVault,
) -> CaliptraResult<Ecc384Scalar> {
    let ds: DataStore = persistent_data
        .fht
        .fmc_cert_sig_s_dv_hdl
        .try_into()
        .map_err(|_| CaliptraError::RUNTIME_FMC_CERT_HANDOFF_FAILED)?;

    // The data store is either a warm reset entry or a cold reset entry.
    match ds {
        DataStore::DataVaultNonSticky48(dv_entry) => Ok(dv.read_warm_reset_entry48(dv_entry)),
        DataStore::DataVaultSticky48(dv_entry) => Ok(dv.read_cold_reset_entry48(dv_entry)),
        _ => Err(CaliptraError::RUNTIME_FMC_CERT_HANDOFF_FAILED),
    }
}

// Piece together the r and s portions of the FMC cert signature
pub fn fmc_dice_sign(
    persistent_data: &PersistentData,
    dv: &DataVault,
) -> CaliptraResult<Ecc384Signature> {
    Ok(Ecc384Signature {
        r: fmc_dice_sign_r(persistent_data, dv)?,
        s: fmc_dice_sign_s(persistent_data, dv)?,
    })
}

/// Copy FMC Alias certificate produced by ROM to `cert` buffer
///
/// Returns the number of bytes written to `cert`
#[inline(never)]
pub fn copy_fmc_alias_cert(
    dv: &DataVault,
    persistent_data: &PersistentData,
    cert: &mut [u8],
) -> CaliptraResult<usize> {
    let tbs = persistent_data
        .fmcalias_tbs
        .get(..persistent_data.fht.fmcalias_tbs_size.into());
    let sig = fmc_dice_sign(persistent_data, dv)?;
    cert_from_tbs_and_sig(tbs, &sig, cert)
}

/// Copy RT Alias certificate produced by ROM to `cert` buffer
///
/// Returns the number of bytes written to `cert`
#[inline(never)]
pub fn copy_rt_alias_cert(
    persistent_data: &PersistentData,
    cert: &mut [u8],
) -> CaliptraResult<usize> {
    let tbs = persistent_data
        .rtalias_tbs
        .get(..persistent_data.fht.rtalias_tbs_size.into());
    cert_from_tbs_and_sig(tbs, &persistent_data.fht.rt_dice_sign, cert)
}

/// Create a certificate from a tbs and a signature and write the output to `cert`
fn cert_from_tbs_and_sig(
    tbs: Option<&[u8]>,
    sig: &Ecc384Signature,
    cert: &mut [u8],
) -> CaliptraResult<usize> {
    let Some(tbs) = tbs else {
        return Err(CaliptraError::RUNTIME_INSUFFICIENT_MEMORY);
    };

    // Convert from Ecc384Signature to Ecdsa384Signature
    let bldr_sig = Ecdsa384Signature {
        r: sig.r.into(),
        s: sig.s.into(),
    };
    let Some(builder) = Ecdsa384CertBuilder::new(tbs, &bldr_sig) else {
        return Err(CaliptraError::RUNTIME_INSUFFICIENT_MEMORY);
    };

    let Some(size) = builder.build(cert) else {
        return Err(CaliptraError::RUNTIME_INSUFFICIENT_MEMORY);
    };

    Ok(size)
}
