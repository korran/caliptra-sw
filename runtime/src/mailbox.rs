// Licensed under the Apache-2.0 license

use caliptra_registers::mbox::{enums::MboxStatusE, MboxCsr};

pub struct Mailbox {
    mbox: MboxCsr,
}

impl Mailbox {
    pub fn new(mbox: MboxCsr) -> Self {
        Self {
            mbox,
        }
    }
    /// Check if there is a new command to be executed
    pub fn is_cmd_ready(&mut self) -> bool {
        let mbox = self.mbox.regs();
        mbox.status().read().mbox_fsm_ps().mbox_execute_uc()
    }

    // Get the length of the current mailbox data in bytes
    pub fn dlen(&mut self) -> u32 {
        let mbox = self.mbox.regs();
        mbox.dlen().read()
    }

    // Set the length of the current mailbox data in bytes
    pub fn set_dlen(&mut self, len: u32) {
        let mbox = self.mbox.regs();
        mbox.dlen().write(|_| len);
    }

    // Get the length of the current mailbox data in words
    pub fn dlen_words(&mut self) -> u32 {
        (self.dlen() + 7) / 8
    }

    pub fn cmd(&mut self) -> u32 {
        let mbox = self.mbox.regs();
        mbox.cmd().read()
    }

    pub fn copy_from_mbox(&mut self, buf: &mut [u32]) {
        let mbox = self.mbox.regs();
        for word in buf {
            *word = mbox.dataout().read();
        }
    }

    pub fn copy_to_mbox(&mut self, buf: &[u32]) {
        let mbox = self.mbox.regs();
        for word in buf {
            mbox.datain().write(|_| *word);
        }
    }

    pub fn set_status(&mut self, status: MboxStatusE) {
        let mbox = self.mbox.regs();
        mbox.status().write(|w| w.status(|_| status));
    }
}
