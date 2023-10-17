
use crate::bindings;

pub use crate::bindings::caliptra_fpga_sync_sig_in as SigIn;
pub use crate::bindings::caliptra_fpga_sync_sig_out as SigOut;

#[derive(Debug)]
pub enum AxiErr {
    Timeout = 1,
    SlvErr = 2,
    DecErr = 3,
}

pub struct FpgaSyncVerilated{
    v: *mut crate::bindings::caliptra_fpga_sync_verilated,
    pub input: SigIn,
    pub output: SigOut,
}
impl FpgaSyncVerilated {

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut result = Self{
            v: unsafe { bindings::caliptra_fpga_sync_verilated_new() },
            input: Default::default(),
            output: Default::default(),
        };
        result.input.rstn = true;

        result
    }

    pub fn total_cycles(&self) -> u64 {
        todo!();

    }

    pub fn write_rom_image(&mut self, image: &[u8]) {
        todo!();
    }

    pub fn eval(&mut self) {
        unsafe { bindings::caliptra_fpga_sync_verilated_eval(self.v, &self.input, &mut self.output) }
    }

    /// Toggles core_clk until there have been `n_cycles` rising edges.
    pub fn next_cycle_high(&mut self, n_cycles: u32) {
        for _ in 0..n_cycles {
            loop {
                self.input.aclk = !self.input.aclk;
                self.eval();
                if self.input.aclk {
                    break;
                }
            }
        }
    }

    pub fn axi_read(&mut self, addr: u32) -> Result<u64, AxiErr> {
        self.input.arvalid = true;
        self.input.araddr = addr;
        self.input.arprot = 0b010;

        self.input.rready = true;

        let mut timeout_cycles = 10000;

        loop {
            self.next_cycle_high(1);
            if self.output.arready {
                self.input.arvalid = false;
                break;
            }
            timeout_cycles -= 1;
            if timeout_cycles == 0 {
                return Err(AxiErr::Timeout);
            }
        }
        while !self.output.rvalid {
            self.next_cycle_high(1);
            timeout_cycles -= 1;
            if timeout_cycles == 0 {
                return Err(AxiErr::Timeout);
            }
        }
        match self.output.rresp {
            0b10 => return Err(AxiErr::SlvErr),
            0b11 => return Err(AxiErr::DecErr),
            _ => {},
        }
        Ok(self.output.rdata)
    }
    
    pub fn axi_write(&mut self, addr: u32, data: u64) -> Result<(), AxiErr> {
        self.input.awvalid = true;
        self.input.awaddr = addr;
        self.input.awprot = 0b010;

        self.input.wvalid = true;
        self.input.wdata = data;
        self.input.wstrb = 0xff;

        self.input.bready = true;

        let mut timeout_cycles = 10000;
        while self.input.wvalid && self.input.awvalid {
            self.next_cycle_high(1);

            if self.input.wvalid && self.output.wready {
                self.input.wvalid = false;
            }

            if self.input.awvalid && self.output.awready {
                self.input.awvalid = false;
            }

            timeout_cycles -= 1;
            if timeout_cycles == 0 {
                return Err(AxiErr::Timeout);
            }
        }
        while !self.output.bvalid {
            self.next_cycle_high(1);
            timeout_cycles -= 1;
            if timeout_cycles == 0 {
                return Err(AxiErr::Timeout);
            }
        }
        match self.output.bresp {
            0b10 => return Err(AxiErr::SlvErr),
            0b11 => return Err(AxiErr::DecErr),
             _ => {},
        }

        Ok(())
    }
}

impl Drop for FpgaSyncVerilated {
    fn drop(&mut self) {
        unsafe { bindings::caliptra_fpga_sync_verilated_destroy(self.v) }
    }
}
