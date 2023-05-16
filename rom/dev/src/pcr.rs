/*++

Licensed under the Apache-2.0 license.

File Name:

    pcr.rs

Abstract:

    File contains execution routines for extending PCR0 & PCR1

Environment:

    ROM

Note:

    PCR0 - Journey PCR unlocked and cleared on cold reset
    PCR1 - Current PCR unlocked and cleared on any reset

--*/

use crate::{verifier::RomImageVerificationEnv};
use caliptra_drivers::{Array4x12, CaliptraResult, PcrId, Sha384, PcrBank};

struct PcrExtender<'a> {
    pcr_bank: &'a mut PcrBank,
    sha384: &'a mut Sha384,
}
impl PcrExtender<'_> {
    fn extend(&mut self, data: Array4x12) -> CaliptraResult<()> {
        let bytes: &[u8; 48] = &data.into();
        self.pcr_bank.extend_pcr(PcrId::PcrId0, self.sha384, bytes)
    }
    fn extend_u8(&mut self, data: u8) -> CaliptraResult<()> {
        let bytes = &data.to_le_bytes();
        self.pcr_bank.extend_pcr(PcrId::PcrId0, self.sha384, bytes)
    }
}

/// Extend PCR0
///
/// # Arguments
///
/// * `env` - ROM Environment
pub(crate) fn extend_pcr0(env: &mut RomImageVerificationEnv) -> CaliptraResult<()> {
    // Clear the PCR
    env.pcr_bank.erase_pcr(caliptra_drivers::PcrId::PcrId0)?;

    // Lock the PCR from clear
    env.pcr_bank.set_pcr_lock(caliptra_drivers::PcrId::PcrId0);


    let mut pcr = PcrExtender{
        pcr_bank: env.pcr_bank,
        sha384: env.sha384,
    };

    pcr.extend_u8(env.soc_ifc.lifecycle() as u8)?;
    pcr.extend_u8(env.soc_ifc.debug_locked() as u8)?;
    pcr.extend_u8(env.soc_ifc.fuse_bank().anti_rollback_disable() as u8)?;
    pcr.extend(env.soc_ifc.fuse_bank().vendor_pub_key_hash())?;
    pcr.extend(env.data_vault.owner_pk_hash())?;
    pcr.extend_u8(env.data_vault.vendor_pk_index() as u8)?;
    pcr.extend(env.data_vault.fmc_tci())?;
    pcr.extend_u8(env.data_vault.fmc_svn() as u8)?;

    // TODO: Check PCR0 != 0

    Ok(())
}
