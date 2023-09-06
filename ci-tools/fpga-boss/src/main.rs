use libftdi1_sys::{self, ftdi_interface};
use std::ffi::{CStr, c_uchar, c_char};
use std::ptr;
use std::time::Duration;

const BITMODE_BITBANG: c_uchar = 0x01;
const BITMODE_CBUS: c_uchar = 0x20;

enum SdMuxTarget {
    Host,
    Dut,
}

fn set_sd_mux(target: SdMuxTarget) {
    unsafe {
        let ftdi = libftdi1_sys::ftdi_new();
        if ftdi == ptr::null_mut() {
            panic!("ftdi_new failed");
        }
        let mut rv = libftdi1_sys::ftdi_usb_open(ftdi, 0x04e8, 0x6001);
        if rv < 0 {
            panic!(
                "ftdi_usb_open failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }
        let pin_state = match target {
            SdMuxTarget::Dut => 0xf0,
            SdMuxTarget::Host => 0xf1,
        };
        rv = libftdi1_sys::ftdi_set_bitmode(ftdi, pin_state, BITMODE_CBUS);
        if (rv < 0) {
            panic!(
                "ftdi_set_bitmode failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }
        let mut pins: c_uchar = 0;
        rv = libftdi1_sys::ftdi_read_pins(ftdi, &mut pins as *mut _);
        if (rv < 0) {
            panic!(
                "ftdi_read_pins failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }
        println!("Pins are {}", pins);

        println!("Done");
        libftdi1_sys::ftdi_usb_close(ftdi);
        libftdi1_sys::ftdi_free(ftdi);
    }
}

enum FpgaReset {
    Reset = 0,
    Run = 1,
}
fn fpga_set_reset(r: FpgaReset) {
    unsafe { 
        let ftdi = libftdi1_sys::ftdi_new();
        if ftdi == ptr::null_mut() {
            panic!("ftdi_new failed");
        }
        let mut rv = libftdi1_sys::ftdi_usb_open(ftdi, 0x0403, 0x6011);
        if rv < 0 {
            panic!(
                "ftdi_usb_open failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }

        rv = libftdi1_sys::ftdi_set_interface(ftdi, ftdi_interface::INTERFACE_A);
        if rv < 0 {
            panic!(
                "ftdi_set_interface failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }

        rv = libftdi1_sys::ftdi_set_bitmode(ftdi, 0xc0, BITMODE_BITBANG);
        if (rv < 0) {
            panic!(
                "ftdi_set_bitmode failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }
        match r {
            FpgaReset::Reset => {
                let data: u8 = 0x0d;
                rv = libftdi1_sys::ftdi_write_data(ftdi, &data, 1);
                if (rv < 0) {
                    panic!(
                        "ftdi_write_data failed: {:?}",
                        CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
                    );
                }
            },
            FpgaReset::Run => {
                let data: u8 = 0x8d;
                rv = libftdi1_sys::ftdi_write_data(ftdi, &data, 1);
                if (rv < 0) {
                    panic!(
                        "ftdi_write_data failed: {:?}",
                        CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
                    );
                }
                std::thread::sleep(Duration::from_millis(1));
                let data = 0xcd;
                rv = libftdi1_sys::ftdi_write_data(ftdi, &data, 1);
                if (rv < 0) {
                    panic!(
                        "ftdi_write_data failed: {:?}",
                        CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
                    );
                }
            }
        }
        libftdi1_sys::ftdi_usb_close(ftdi);
        libftdi1_sys::ftdi_free(ftdi);
    }
}

fn main() {
    match std::env::args().skip(1).next().as_ref().map(|s| s.as_str()) {
        Some("dut") => {
            fpga_set_reset(FpgaReset::Reset);
            set_sd_mux(SdMuxTarget::Dut);
            std::thread::sleep(Duration::from_millis(1));
            fpga_set_reset(FpgaReset::Run);
        },
        Some("host") => {
            fpga_set_reset(FpgaReset::Reset);
            set_sd_mux(SdMuxTarget::Host);
        }
        other => {
            println!("{:?}", other);
            println!("Usage: fpga-boss host|dut");
            std::process::exit(1);
        }
    }
}
