/*++

Licensed under the Apache-2.0 license.

File Name:

    boot_tests.rs

Abstract:

    File contains test cases for booting runtime firmware

--*/

#![no_std]
#![no_main]

use caliptra_test_harness::test_suite;
use caliptra_registers::mbox::MboxCsr;

fn test_mbox_cmd() {
    let mut mbox = unsafe { MboxCsr::new() };
    caliptra_runtime::handle_mailbox_commands(mbox);
}

test_suite! {
    test_mbox_cmd,
}
