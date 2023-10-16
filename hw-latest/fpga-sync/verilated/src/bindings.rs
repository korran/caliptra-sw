/*++
Licensed under the Apache-2.0 license.
--*/

// generated by hw-latest/fpga-sync/verilated/generate_rust_bindings.sh

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caliptra_fpga_sync_verilated {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct caliptra_fpga_sync_sig_in {
    pub aclk: bool,
    pub rstn: bool,
    pub arvalid: bool,
    pub araddr: u32,
    pub arprot: u8,
    pub rready: bool,
    pub awvalid: bool,
    pub awaddr: u32,
    pub awprot: u8,
    pub wvalid: bool,
    pub wdata: u64,
    pub wstrb: u8,
    pub bready: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct caliptra_fpga_sync_sig_out {
    pub arready: bool,
    pub rvalid: bool,
    pub rdata: u64,
    pub rresp: u8,
    pub awready: bool,
    pub wready: bool,
    pub bvalid: bool,
    pub bresp: u8,
}
extern "C" {
    pub fn caliptra_fpga_sync_verilated_new() -> *mut caliptra_fpga_sync_verilated;
}
extern "C" {
    pub fn caliptra_fpga_sync_verilated_destroy(model: *mut caliptra_fpga_sync_verilated);
}
extern "C" {
    pub fn caliptra_fpga_sync_verilated_trace(
        model: *mut caliptra_fpga_sync_verilated,
        vcd_out_path: *const ::std::os::raw::c_char,
        depth: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn caliptra_fpga_sync_verilated_eval(
        model: *mut caliptra_fpga_sync_verilated,
        in_: *const caliptra_fpga_sync_sig_in,
        out: *mut caliptra_fpga_sync_sig_out,
    );
}
