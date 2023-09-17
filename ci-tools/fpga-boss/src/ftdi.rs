use std::{ptr, ffi::CStr};

use rusb::{Device, GlobalContext};

use crate::UsbPortPath;


fn device_from_path(port_path: &UsbPortPath) -> anyhow::Result<Device<GlobalContext>> {
    for dev in rusb::devices()?.iter() {
        if dev.bus_number() == port_path.bus && dev.port_numbers()? == port_path.ports {
            return Ok(dev)
        }
    }
    anyhow::bail!("USB device not found: {}", port_path);
}

#[repr(u8)]
pub enum BitMode {
    BitBang = 0x01,
    CBus = 0x20,
}
pub struct FtdiCtx {
    ctx: *mut libftdi1_sys::ftdi_context,
    port_path: UsbPortPath,
}
impl FtdiCtx {
    pub fn open(port_path: UsbPortPath) -> anyhow::Result<Self> {
        let dev = device_from_path(&port_path)?;
        unsafe {
            let ctx = libftdi1_sys::ftdi_new();
            if ctx == ptr::null_mut() {
                anyhow::bail!("ftdi_new failed");
            }
            let rv = libftdi1_sys::ftdi_usb_open_dev(ctx, dev.as_raw());
            if rv < 0 {
                libftdi1_sys::ftdi_free(ctx);
                anyhow::bail!(
                    "ftdi_usb_open failed for device {port_path}: {:?}",
                    CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ctx))
                );
            }
            Ok(Self { 
                ctx,
                port_path,
            })
        }
    }

    pub fn set_interface(&mut self, iface: libftdi1_sys::ftdi_interface) -> anyhow::Result<()> {
        unsafe {
            let rv = libftdi1_sys::ftdi_set_interface(self.ctx, iface);
            if rv < 0 {
                panic!(
                    "{} ftdi_set_interface failed: {:?}",
                    self.port_path,
                    CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(self.ctx))
                );
            }
            Ok(())
        }

    }

    pub fn set_bitmode(&mut self, pin_state: u8, mode: BitMode) -> anyhow::Result<()> {
        unsafe {
            let rv = libftdi1_sys::ftdi_set_bitmode(self.ctx, pin_state, mode as u8);
            if rv < 0 {
                anyhow::bail!(
                    "{} ftdi_set_bitmode failed: {:?}",
                    self.port_path,
                    CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(self.ctx))
                );
            }
            Ok(())
        }
    }

    #[allow(unused)]
    pub fn read_pins(&mut self, pin_state: u8, mode: BitMode) -> anyhow::Result<u8> {
        unsafe {
            let mut pins: u8 = 0;
            let rv = libftdi1_sys::ftdi_read_pins(self.ctx, &mut pins as *mut _);
            if rv < 0 {
                anyhow::bail!(
                    "{} ftdi_read_pins failed: {:?}",
                    self.port_path,
                    CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(self.ctx))
                );
            }   
            Ok(pins)
        }
    }

    pub fn write_all_data(&mut self, data: &[u8]) -> anyhow::Result<()> {
        let bytes_written = self.write_data(data)?;
        if bytes_written != data.len() {
            anyhow::bail!("{} ftdi_write data returned {} bytes, expected {}",
            self.port_path, bytes_written, data.len());
        }
        Ok(())
    }

    pub fn write_data(&mut self, data: &[u8]) -> anyhow::Result<usize> {
        unsafe {
            let rv = libftdi1_sys::ftdi_write_data(self.ctx, data.as_ptr(), data.len().try_into()?);
            if rv < 0 {
                anyhow::bail!(
                    "ftdi_write_data failed: {:?}",
                    CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(self.ctx))
                );
            }
            Ok(rv.try_into()?)
        }
    }
}

impl Drop for FtdiCtx {
    fn drop(&mut self) {
        unsafe { libftdi1_sys::ftdi_usb_close(self.ctx) };
        unsafe { libftdi1_sys::ftdi_free(self.ctx) };
    }
}


