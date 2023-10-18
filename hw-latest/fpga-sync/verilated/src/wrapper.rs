
use std::error::Error;
use std::ffi::CString;
use std::ffi::NulError;
use std::fmt::Display;
use std::ptr::null;

use crate::bindings;

pub use crate::bindings::caliptra_fpga_sync_sig_in as SigIn;
pub use crate::bindings::caliptra_fpga_sync_sig_out as SigOut;

#[derive(Debug)]
pub enum AxiErr {
    Timeout = 1,
    SlvErr = 2,
    DecErr = 3,
}
impl Error for AxiErr {}
impl Display for AxiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
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

    /// Starts tracing to VCD file `path`, with SystemVerilog module depth
    /// `depth`. If tracing was previously started to another file, that file
    /// will be closed and all new traces will be written to this file.
    pub fn start_tracing(&mut self, path: &str, depth: i32) -> Result<(), NulError> {
        unsafe {
            bindings::caliptra_fpga_sync_verilated_trace(self.v, CString::new(path)?.as_ptr(), depth);
        }
        Ok(())
    }

    /// Stop any tracing that might have been previously started with `start_tracing()`.
    pub fn stop_tracing(&mut self) {
        unsafe {
            bindings::caliptra_fpga_sync_verilated_trace(self.v, null(), 0);
        }
    }

}

impl Drop for FpgaSyncVerilated {
    fn drop(&mut self) {
        unsafe { bindings::caliptra_fpga_sync_verilated_destroy(self.v) }
    }
}
