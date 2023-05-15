/*++

Licensed under the Apache-2.0 license.

File Name:

    rom_env.rs

Abstract:

    File implements a context holding all the services utilized by ROM.
    The primary need for this abstraction is to hide the hardware details
    from the ROM flows. The natural side benefit of this abstraction is it
    makes authoring mocks and unit tests easy.

--*/

use core::ops::Range;

use caliptra_drivers::{
    DataVault, DeobfuscationEngine, Ecc384, Hmac384, KeyVault, Mailbox, PcrBank, Sha1, Sha256,
    Sha384, Sha384Acc, SocIfc,
};
use caliptra_registers::{sha256::Sha256Reg, sha512::Sha512Reg, sha512_acc::Sha512AccCsr, hmac::HmacReg, ecc::EccReg, kv::KvReg, dv::DvReg, soc_ifc::SocIfcReg, mbox::MboxCsr, pv::PvReg, doe::DoeReg};

const ICCM_START: u32 = 0x40000000;
const ICCM_SIZE: u32 = 128 << 10;

/// Rom Context
pub struct RomEnv {
    /// Deobfuscation engine
    pub doe: DeobfuscationEngine,

    // SHA1 Engine
    pub sha1: Sha1,

    // SHA2-256 Engine
    pub sha256: Sha256,

    // SHA2-384 Engine
    pub sha384: Sha384,

    // SHA2-384 Accelerator
    pub sha384_acc: Sha384Acc,

    /// Hmac384 Engine
    pub hmac384: Hmac384,

    /// Ecc384 Engine
    pub ecc384: Ecc384,

    /// Key Vault
    pub key_vault: KeyVault,

    /// Data Vault
    pub data_vault: DataVault,

    /// SoC interface
    pub soc_ifc: SocIfc,

    /// Mailbox
    pub mbox: Mailbox,

    /// PCR Bank
    pub pcr_bank: PcrBank,
}

impl RomEnv {
    pub unsafe fn new_from_registers() -> Self {
        Self {
            doe: DeobfuscationEngine::new(DoeReg::new()),
            sha1: Sha1::default(),
            sha256: Sha256::new(Sha256Reg::new()),
            sha384: Sha384::new(Sha512Reg::new()),
            sha384_acc: Sha384Acc::new(Sha512AccCsr::new()),
            hmac384: Hmac384::new(HmacReg::new()),
            ecc384: Ecc384::new(EccReg::new()),
            key_vault: KeyVault::new(KvReg::new()),
            data_vault: DataVault::new(DvReg::new()),
            soc_ifc: SocIfc::new(SocIfcReg::new()),
            mbox: Mailbox::new(MboxCsr::new()),
            pcr_bank: PcrBank::new(PvReg::new()),
        }
    }

    /// Get ICCM Range
    pub fn iccm_range(&self) -> Range<u32> {
        Range {
            start: ICCM_START,
            end: ICCM_START + ICCM_SIZE,
        }
    }
}
