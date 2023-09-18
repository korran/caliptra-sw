use std::io::{self, ErrorKind};

use libftdi1_sys::{ftdi_bits_type, ftdi_interface, ftdi_parity_type, ftdi_stopbits_type};

use crate::{
    ftdi::{BitMode, FtdiCtx},
    usb_port_path::UsbPortPath,
};

pub struct FtdiUart {
    ftdi: FtdiCtx,
}

impl FtdiUart {
    pub fn open(port_path: UsbPortPath, iface: ftdi_interface) -> anyhow::Result<Self> {
        let mut ftdi = FtdiCtx::open(port_path, iface)?;
        ftdi.set_bitmode(0, BitMode::Reset)?;
        ftdi.set_baudrate(115200)?;
        ftdi.set_line_property(
            ftdi_bits_type::BITS_8,
            ftdi_stopbits_type::STOP_BIT_1,
            ftdi_parity_type::NONE,
        )?;
        Ok(Self { ftdi })
    }
}
impl io::Read for FtdiUart {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.ftdi
            .read_data(buf)
            .map_err(|e| io::Error::new(ErrorKind::Other, e))
    }
}

impl io::Write for FtdiUart {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.ftdi
            .write_data(buf)
            .map_err(|e| io::Error::new(ErrorKind::Other, e))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
