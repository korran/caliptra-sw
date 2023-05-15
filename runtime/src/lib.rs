// Licensed under the Apache-2.0 license

#![no_std]

mod mailbox;

use mailbox::Mailbox;

use caliptra_common::cprintln;
use caliptra_drivers::{caliptra_err_def, CaliptraResult};
use caliptra_registers::mbox::{enums::MboxStatusE, MboxCsr};

use core::mem::size_of;

caliptra_err_def! {
    Runtime,
    RuntimeErr
    {
        // Internal
        InternalErr = 0x1,
    }
}

fn wait_for_cmd(_mbox: &mut Mailbox) {
    // TODO: Enable interrupts?
    //#[cfg(feature = "riscv")]
    //unsafe {
    //core::arch::asm!("wfi");
    //}
}

fn handle_command(mbox: &mut Mailbox) -> CaliptraResult<()> {
    let cmd_id = mbox.cmd();
    let dlen_words = mbox.dlen_words() as usize;
    let mut buf = [0u32; 1024];
    mbox.copy_from_mbox(buf.get_mut(..dlen_words).ok_or(err_u32!(InternalErr))?);

    // TODO: Actually handle command
    cprintln!("[rt] Received command={}, len={}", cmd_id, mbox.dlen());

    // Write response
    let out_buf = [0xFFFFFFFFu32; 4];
    mbox.set_dlen((out_buf.len() * size_of::<u32>()) as u32);
    mbox.copy_to_mbox(&out_buf);

    Ok(())
}

pub fn handle_mailbox_commands(mbox_regs: MboxCsr) {
    let mut mbox = Mailbox::new(mbox_regs);
    loop {
        wait_for_cmd(&mut mbox);

        if mbox.is_cmd_ready() {
            if handle_command(&mut mbox).is_ok() {
                mbox.set_status(MboxStatusE::DataReady);
            } else {
                mbox.set_status(MboxStatusE::CmdFailure);
            }
        }
    }
}
