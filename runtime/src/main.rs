/*++

Licensed under the Apache-2.0 license.

File Name:

    main.rs

Abstract:

    File contains main entry point for Caliptra Test Runtime

--*/
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

use caliptra_common::cprintln;
use caliptra_cpu::TrapRecord;
use caliptra_drivers::{report_fw_error_non_fatal, Mailbox};
use core::{hint::black_box, mem};

#[cfg(feature = "std")]
pub fn main() {}

const BANNER: &str = r#"
  ____      _ _       _               ____ _____
 / ___|__ _| (_)_ __ | |_ _ __ __ _  |  _ \_   _|
| |   / _` | | | '_ \| __| '__/ _` | | |_) || |
| |__| (_| | | | |_) | |_| | | (_| | |  _ < | |
 \____\__,_|_|_| .__/ \__|_|  \__,_| |_| \_\|_|
               |_|
"#;

const MBOX_DOWNLOAD_FIRMWARE_CMD_ID: u32 = 0x46574C44;

#[no_mangle]
pub extern "C" fn entry_point() -> ! {
    cprintln!("{}", BANNER);

    let mbox = Mailbox::default();
    let download_txn = if let Some(txn) = mbox.try_start_recv_txn() {
        // TODO: Consider adding a flag to the FHT that the ROM left a
        // DOWNLOAD_FIRMWARE command in the mailbox for us to notify of success.
        if txn.cmd() == MBOX_DOWNLOAD_FIRMWARE_CMD_ID {
            // This transaction is ManuallyDrop because we don't want the
            // transaction to be completed with failure until after report_error
            // is called.
            Some(mem::ManuallyDrop::new(txn))
        } else {
            // This is some other command; leave it in the mailbox for the event
            // loop to handle later
            mem::forget(txn);
            None
        }
    } else {
        None
    };

    if let Some(fht) = caliptra_common::FirmwareHandoffTable::try_load() {
        cprintln!("[rt] FHT Marker: 0x{:08X}", fht.fht_marker);
        cprintln!("[rt] FHT Major Version: 0x{:04X}", fht.fht_major_ver);
        cprintln!("[rt] FHT Minor Version: 0x{:04X}", fht.fht_minor_ver);
        cprintln!("[rt] FHT Manifest Addr: 0x{:08X}", fht.manifest_load_addr);
        cprintln!("[rt] FHT FMC CDI KV KeyID: {}", fht.fmc_cdi_kv_idx);
        cprintln!("[rt] FHT FMC PrivKey KV KeyID: {}", fht.fmc_priv_key_kv_idx);
        cprintln!(
            "[rt] FHT RT Load Address: 0x{:08x}",
            fht.rt_fw_load_addr_idx
        );
        cprintln!("[rt] FHT RT Entry Point: 0x{:08x}", fht.rt_fw_load_addr_idx);
        if let Some(mut txn) = download_txn {
            // Complete the download-firmware transaction that was started in the ROM
            txn.complete(true).ok();
        }
        caliptra_drivers::ExitCtrl::exit(0)
    } else {
        // TODO: Use report_error()
        if let Some(mut txn) = download_txn {
            txn.complete(false).ok();
        }
        caliptra_drivers::ExitCtrl::exit(0xff)
    }
}

#[no_mangle]
#[inline(never)]
#[allow(clippy::empty_loop)]
extern "C" fn exception_handler(trap_record: &TrapRecord) {
    cprintln!(
        "FMC EXCEPTION mcause=0x{:08X} mscause=0x{:08X} mepc=0x{:08X}",
        trap_record.mcause,
        trap_record.mscause,
        trap_record.mepc
    );

    // Signal non-fatal error to SOC
    report_error(0xdead);
}

#[no_mangle]
#[inline(never)]
#[allow(clippy::empty_loop)]
extern "C" fn nmi_handler(trap_record: &TrapRecord) {
    cprintln!(
        "FMC NMI mcause=0x{:08X} mscause=0x{:08X} mepc=0x{:08X}",
        trap_record.mcause,
        trap_record.mscause,
        trap_record.mepc
    );

    report_error(0xdead);
}

#[panic_handler]
#[inline(never)]
#[cfg(not(feature = "std"))]
#[allow(clippy::empty_loop)]
fn runtime_panic(_: &core::panic::PanicInfo) -> ! {
    cprintln!("RT Panic!!");
    panic_is_possible();

    // TODO: Signal non-fatal error to SOC
    report_error(0xdead);
}

#[allow(clippy::empty_loop)]
fn report_error(code: u32) -> ! {
    cprintln!("RT Error: 0x{:08X}", code);
    report_fw_error_non_fatal(code);
    loop {
        // SoC firmware might be stuck waiting for Caliptra to finish
        // executing this pending mailbox transaction. Notify them that
        // we've failed.
        unsafe { Mailbox::abort_pending_soc_to_uc_transactions() };
    }
}

#[no_mangle]
#[inline(never)]
fn panic_is_possible() {
    black_box(());
    // The existence of this symbol is used to inform test_panic_missing
    // that panics are possible. Do not remove or rename this symbol.
}
