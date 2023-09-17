use std::time::Duration;

use crate::{FtdiCtx, UsbPortPath};



pub enum FpgaReset {
    Reset = 0,
    Run = 1,
}

pub struct FpgaJtag {
    ftdi: FtdiCtx,
}
impl FpgaJtag {
    pub fn open(port_path: UsbPortPath) -> anyhow::Result<Self> {
        Ok(Self {
            ftdi: FtdiCtx::open(port_path)?,
        })
    }

    pub fn set_reset(&mut self, reset: FpgaReset) -> anyhow::Result<()> {
        match reset {
            FpgaReset::Reset => {
                // Set PS_POR_B and PS_SRST_B pins low
                self.ftdi.write_all_data(&[0x0d])?;
            },
            FpgaReset::Run => {
                // Set PS_POR_B high, PS_SRST_B low
                self.ftdi.write_all_data(&[0x8d])?;

                // wait a bi
                std::thread::sleep(Duration::from_millis(1));

                // Set PS_POR_B and PS_SRST_B pins high
                self.ftdi.write_all_data(&[0xcd])?;
            }
        }
        Ok(())
    }
}

