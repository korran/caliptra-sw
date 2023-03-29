use std::io::stdout;

use caliptra_builder::{ROM_WITH_UART, FMC_WITH_UART, APP_WITH_UART, ImageOptions};
use caliptra_hw_model::{HwModel, InitParams};



#[test]
fn smoke_test() {
    let rom = caliptra_builder::build_firmware_rom(&ROM_WITH_UART).unwrap();
    let image = caliptra_builder::build_and_sign_image(&FMC_WITH_UART, &APP_WITH_UART, ImageOptions::default()).unwrap();
    let mut hw = caliptra_hw_model::create(InitParams{
        rom: &rom,
        ..Default::default()
    }).unwrap();
    hw.upload_firmware(&image).unwrap();
    //hw.step_until_exit_success().unwrap();
}

