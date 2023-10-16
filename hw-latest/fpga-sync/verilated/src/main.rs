use wrapper::FpgaSyncVerilated;


#[allow(non_camel_case_types)]
mod bindings;

mod wrapper;

fn main() {
    let mut v = FpgaSyncVerilated::new();
    v.next_cycle_high(10);
    v.axi_write(0, 0xffff_fffa_ffff_ffff).unwrap();
    println!("{:016x}", v.axi_read(0).unwrap());

    v.axi_write(8, 0xffff_ffff_ffff_ffff).unwrap();
    println!("{:016x}", v.axi_read(8).unwrap());

}