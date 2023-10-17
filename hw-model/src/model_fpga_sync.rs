// Licensed under the Apache-2.0 license

use crate::bus_logger::{BusLogger, LogFile, NullBus};
use crate::EtrngResponse;
use crate::{HwModel, TrngMode};
use caliptra_emu_bus::Bus;
use caliptra_emu_types::{RvAddr, RvData, RvSize};
use caliptra_fpga_sync_verilated::FpgaSyncVerilated;
use caliptra_hw_model_types::ErrorInjectionMode;
use std::cell::{Cell, RefCell};
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use crate::Output;
use std::env;

// How many clock cycles before emitting a TRNG nibble
const TRNG_DELAY: u32 = 4;

pub struct ApbBus<'a> {
    model: &'a mut ModelFpgaSync,
}
impl<'a> Bus for ApbBus<'a> {
    fn read(&mut self, size: RvSize, addr: RvAddr) -> Result<RvData, caliptra_emu_bus::BusError> {
        if addr & 0x3 != 0 {
            return Err(caliptra_emu_bus::BusError::LoadAddrMisaligned);
        }
        let result = self.model.apb_read_u32(self.model.soc_apb_pauser, addr).map_err(|_| caliptra_emu_bus::BusError::LoadAccessFault);
        self.model
            .log
            .borrow_mut()
            .log_read("SoC", size, addr, result);
        result
    }

    fn write(
        &mut self,
        size: RvSize,
        addr: RvAddr,
        val: RvData,
    ) -> Result<(), caliptra_emu_bus::BusError> {
        if addr & 0x3 != 0 {
            return Err(caliptra_emu_bus::BusError::StoreAddrMisaligned);
        }
        if size != RvSize::Word {
            return Err(caliptra_emu_bus::BusError::StoreAccessFault);
        }
        let result = self.model
            .apb_write_u32(self.model.soc_apb_pauser, addr, val).map_err(|_| caliptra_emu_bus::BusError::StoreAccessFault);
        self.model
            .log
            .borrow_mut()
            .log_write("SoC", size, addr, val, result);
        Ok(())
    }
}

// Like EtrngResponse, but with an absolute time
struct AbsoluteEtrngResponse {
    time: u64,
    data: [u32; 12],
}

pub struct ModelFpgaSync {
    v: FpgaSyncVerilated,

    output: Output,
    trace_enabled: bool,

    trng_mode: TrngMode,

    itrng_nibbles: Box<dyn Iterator<Item = u8>>,
    itrng_delay_remaining: u32,

    etrng_responses: Box<dyn Iterator<Item = EtrngResponse>>,
    etrng_response: Option<AbsoluteEtrngResponse>,
    etrng_waiting_for_req_to_clear: bool,

    log: Rc<RefCell<BusLogger<NullBus>>>,

    soc_apb_pauser: u32,
}

impl ModelFpgaSync {
    fn apb_read_u32(&mut self, pauser: u32, addr: u32) -> Result<u32, ()> {
        self.process_trng();
        todo!();
    }

    fn apb_write_u32(&mut self, pauser: u32, addr: u32, value: u32) -> Result<(), ()> {
        self.process_trng();
        todo!();
    }
}

impl crate::HwModel for ModelFpgaSync {
    type TBus<'a> = ApbBus<'a>;

    fn new_unbooted(params: crate::InitParams) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let output = Output::new(params.log_writer);

        let output_sink = output.sink().clone();

        let generic_output_wires_changed_cb = {
            let prev_uout = Cell::new(None);
            Box::new(move |v: &FpgaSyncVerilated, out_wires: u64| {
                if Some(out_wires & 0x1ff) != prev_uout.get() {
                    // bit #8 toggles whenever the Uart driver writes a byte, so
                    // by including it in the comparison we can tell when the
                    // same character has been written a second time
                    if prev_uout.get().is_some() {
                        // Don't print out a character for the initial state
                        output_sink.set_now(v.total_cycles());
                        output_sink.push_uart_char((out_wires & 0xff) as u8);
                    }
                    prev_uout.set(Some(out_wires & 0x1ff));
                }
            })
        };

        let log = Rc::new(RefCell::new(BusLogger::new(NullBus())));
        let bus_log = log.clone();

        let compiled_trng_mode = if cfg!(feature = "itrng") {
            TrngMode::Internal
        } else {
            TrngMode::External
        };
        let desired_trng_mode = TrngMode::resolve(params.trng_mode);
        if desired_trng_mode != compiled_trng_mode {
            let msg_suffix = match desired_trng_mode {
                TrngMode::Internal => "try compiling with --features itrng",
                TrngMode::External => "try compiling without --features itrng",
            };
            return Err(format!(
                "HwModel InitParams asked for trng_mode={desired_trng_mode:?}, \
                    but verilog was compiled with trng_mode={compiled_trng_mode:?}; {msg_suffix}"
            )
            .into());
        }
        let mut v = FpgaSyncVerilated::new();

        v.write_rom_image(params.rom);

        let mut m = ModelFpgaSync {
            v,
            output,
            trace_enabled: false,

            trng_mode: desired_trng_mode,

            itrng_nibbles: params.itrng_nibbles,
            itrng_delay_remaining: TRNG_DELAY,

            etrng_responses: params.etrng_responses,
            etrng_response: None,
            etrng_waiting_for_req_to_clear: false,

            log,

            soc_apb_pauser: params.soc_apb_pauser,
        };

        m.tracing_hint(true);

        todo!();
        //m.v.input.cptra_pwrgood = true;
        //m.v.next_cycle_high(1);

        //m.v.input.cptra_rst_b = true;
        //m.v.next_cycle_high(1);

        //while !m.v.output.ready_for_fuses {
        //    m.v.next_cycle_high(1);
        //}
        writeln!(m.output().logger(), "ready_for_fuses is high")?;
        Ok(m)
    }

    fn apb_bus(&mut self) -> Self::TBus<'_> {
        ApbBus { model: self }
    }

    fn step(&mut self) {
        self.process_trng_start();
        self.v.next_cycle_high(1);
        self.process_trng_end();
    }

    fn output(&mut self) -> &mut crate::Output {
        self.output.sink().set_now(self.v.total_cycles());
        &mut self.output
    }

    fn warm_reset(&mut self) {
        todo!();
        // Toggle reset pin
        //self.v.input.cptra_rst_b = false;
        //self.v.next_cycle_high(1);

        //self.v.input.cptra_rst_b = true;
        //self.v.next_cycle_high(1);

        //// Wait for ready_for_fuses
        //while !self.v.output.ready_for_fuses {
        //    self.v.next_cycle_high(1);
        //}
    }

    fn ready_for_fw(&self) -> bool {
        todo!();
        //self.v.output.ready_for_fw_push
    }

    fn tracing_hint(&mut self, enable: bool) {
    }

    fn ecc_error_injection(&mut self, mode: ErrorInjectionMode) {
        // todo!()
        //match mode {
        //    ErrorInjectionMode::None => {
        //        self.v.input.sram_error_injection_mode = 0x0;
        //    }
        //    ErrorInjectionMode::IccmDoubleBitEcc => {
        //        self.v.input.sram_error_injection_mode = 0x2;
        //    }
        //    ErrorInjectionMode::DccmDoubleBitEcc => {
        //        self.v.input.sram_error_injection_mode = 0x8;
        //    }
        //}
    }
}
impl ModelFpgaSync {
    fn process_trng(&mut self) {
        if self.process_trng_start() {
            self.v.next_cycle_high(1);
            self.process_trng_end();
        }
    }
    fn process_trng_start(&mut self) -> bool {
        match self.trng_mode {
            TrngMode::Internal => self.process_itrng_start(),
            TrngMode::External => self.process_etrng_start(),
        }
    }

    fn process_trng_end(&mut self) {
        match self.trng_mode {
            TrngMode::Internal => self.process_itrng_end(),
            TrngMode::External => {}
        }
    }

    // Returns true if process_trng_end must be called after a clock cycle
    fn process_etrng_start(&mut self) -> bool {
        todo!();
        //if self.etrng_waiting_for_req_to_clear && !self.v.output.etrng_req {
        //    self.etrng_waiting_for_req_to_clear = false;
        //}
        //if self.v.output.etrng_req && !self.etrng_waiting_for_req_to_clear {
        //    if self.etrng_response.is_none() {
        //        if let Some(response) = self.etrng_responses.next() {
        //            self.etrng_response = Some(AbsoluteEtrngResponse {
        //                time: self.v.total_cycles() + u64::from(response.delay),
        //                data: response.data,
        //            });
        //        }
        //    }
        //    if let Some(etrng_response) = &mut self.etrng_response {
        //        if self.v.total_cycles().wrapping_sub(etrng_response.time) < 0x8000_0000_0000_0000 {
        //            self.etrng_waiting_for_req_to_clear = true;
        //            let etrng_response = self.etrng_response.take().unwrap();
        //            self.soc_ifc_trng()
        //                .cptra_trng_data()
        //                .write(&etrng_response.data);
        //            self.soc_ifc_trng()
        //                .cptra_trng_status()
        //                .write(|w| w.data_wr_done(true));
        //        }
        //    }
        //}
        false
    }
    // Returns true if process_trng_end must be called after a clock cycle
    fn process_itrng_start(&mut self) -> bool {
        todo!();
        //if self.v.output.etrng_req {
        //    if self.itrng_delay_remaining == 0 {
        //        if let Some(val) = self.itrng_nibbles.next() {
        //            self.v.input.itrng_valid = true;
        //            self.v.input.itrng_data = val & 0xf;
        //        }
        //        self.itrng_delay_remaining = TRNG_DELAY;
        //    } else {
        //        self.itrng_delay_remaining -= 1;
        //    }
        //    self.v.input.itrng_valid
        //} else {
        //    false
        //}
    }
    fn process_itrng_end(&mut self) {
        todo!();
        //if self.v.input.itrng_valid {
        //    self.v.input.itrng_valid = false;
        //}
    }
}
