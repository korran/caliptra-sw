#![no_std]
#![no_main]

use caliptra_drivers::{ColdResetEntry4, DataVault, Mailbox};
use caliptra_registers::mbox::{self};
use core::mem::size_of;
use core::slice;

mod harness;

#[inline(never)]
fn test_datavault() {
    let mut dv = DataVault::default();
    dv.write_cold_reset_entry4(ColdResetEntry4::FmcEntryPoint, 0xba5e_ba11);
    assert_eq!(dv.fmc_entry_point(), 0xba5e_ba11);
}

test_suite! {
    test_datavault,
}
