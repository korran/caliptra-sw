#![no_std]
#![no_main]

use caliptra_drivers::Mailbox;
use caliptra_registers::mbox::{self};
use core::mem::size_of;
use core::slice;

mod harness;

#[inline(never)]
fn test_iccm_write() {
    unsafe {
        (0x4000_0000 as *mut u8).write_volatile(42);
        //println!("Read 0x4000_0000 as {}", (0x4000_0000  as *mut u8).read());
        //println!("0x30030620={:x}", (0x3003_0620 as *mut u32).read());
    }
}

test_suite! {
    test_iccm_write,
}
