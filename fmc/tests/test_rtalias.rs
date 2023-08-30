// Licensed under the Apache-2.0 license
use caliptra_builder::{firmware, ImageOptions};
use caliptra_hw_model::{BootParams, HwModel, InitParams};

const RT_ALIAS_MEASUREMENT_COMPLETE: u32 = 0x400;
const RT_ALIAS_DERIVED_CDI_COMPLETE: u32 = 0x401;
const RT_ALIAS_KEY_PAIR_DERIVATION_COMPLETE: u32 = 0x402;
const RT_ALIAS_SUBJ_ID_SN_GENERATION_COMPLETE: u32 = 0x403;
const RT_ALIAS_SUBJ_KEY_ID_GENERATION_COMPLETE: u32 = 0x404;
const RT_ALIAS_CERT_SIG_GENERATION_COMPLETE: u32 = 0x405;
const RT_ALIAS_DERIVATION_COMPLETE: u32 = 0x406;

#[test]
fn test_boot_status_reporting() {
    let rom = caliptra_builder::build_firmware_rom(&firmware::ROM_WITH_UART).unwrap();

    let image = caliptra_builder::build_and_sign_image(
        &firmware::FMC_WITH_UART,
        &firmware::fmc_tests::MOCK_RT_WITH_UART,
        ImageOptions::default(),
    )
    .unwrap();

    let mut hw = caliptra_hw_model::new(BootParams {
        init_params: InitParams {
            rom: &rom,
            ..Default::default()
        },
        fw_image: Some(&image.to_bytes().unwrap()),
        ..Default::default()
    })
    .unwrap();

    hw.step_until_boot_status(RT_ALIAS_MEASUREMENT_COMPLETE, true);
    hw.step_until_boot_status(RT_ALIAS_DERIVED_CDI_COMPLETE, true);
    hw.step_until_boot_status(RT_ALIAS_KEY_PAIR_DERIVATION_COMPLETE, true);
    hw.step_until_boot_status(RT_ALIAS_SUBJ_ID_SN_GENERATION_COMPLETE, true);
    hw.step_until_boot_status(RT_ALIAS_SUBJ_KEY_ID_GENERATION_COMPLETE, true);
    hw.step_until_boot_status(RT_ALIAS_CERT_SIG_GENERATION_COMPLETE, true);
    hw.step_until_boot_status(RT_ALIAS_DERIVATION_COMPLETE, true);

    let mut output = vec![];
    hw.copy_output_until_exit_success(&mut output).unwrap();
}
