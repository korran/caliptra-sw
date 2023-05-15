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

use crate::rom_env::RomEnv;
use caliptra_drivers::{Array4x12, CaliptraResult, PcrId};

/// Extend PCR0
///
/// # Arguments
///
/// * `env` - ROM Environment
pub fn extend_pcr0(env: &mut RomEnv) -> CaliptraResult<()> {
    let sha384 = &mut env.sha384;

    // Clear the PCR
    env.pcr_bank.erase_pcr(caliptra_drivers::PcrId::PcrId0)?;

    // Lock the PCR from clear
    env.pcr_bank.set_pcr_lock(caliptra_drivers::PcrId::PcrId0);

    let extend = |data: Array4x12| {
        let bytes: &[u8; 48] = &data.into();
        env.pcr_bank.extend_pcr(PcrId::PcrId0, sha384, bytes)
    };

    let extend_u8 = |data: u8| {
        let bytes = &data.to_le_bytes();
        env.pcr_bank.extend_pcr(PcrId::PcrId0, sha384, bytes)
    };

    extend_u8(env.soc_ifc.lifecycle() as u8)?;
    extend_u8(env.soc_ifc.debug_locked() as u8)?;
    extend_u8(env.soc_ifc.fuse_bank().anti_rollback_disable() as u8)?;
    extend(env.soc_ifc.fuse_bank().vendor_pub_key_hash())?;
    extend(env.data_vault.owner_pk_hash())?;
    extend_u8(env.data_vault.vendor_pk_index() as u8)?;
    extend(env.data_vault.fmc_tci())?;
    extend_u8(env.data_vault.fmc_svn() as u8)?;

    // TODO: Check PCR0 != 0

    Ok(())
}
