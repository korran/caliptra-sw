// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with caliptra-rtl repo at bdc7673ad4ab6d191f186eb170977c614d170be4
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
#[derive(Clone, Copy)]
pub struct RegisterBlock<TMmio: ureg::Mmio + core::borrow::Borrow<TMmio> = ureg::RealMmio> {
    ptr: *mut u32,
    mmio: TMmio,
}
impl RegisterBlock<ureg::RealMmio> {
    pub fn dv_reg() -> Self {
        unsafe { Self::new(0x1001c000 as *mut u32) }
    }
}
impl<TMmio: ureg::Mmio + core::default::Default> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    pub unsafe fn new(ptr: *mut u32) -> Self {
        Self {
            ptr,
            mmio: core::default::Default::default(),
        }
    }
}
impl<TMmio: ureg::Mmio> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    pub unsafe fn new_with_mmio(ptr: *mut u32, mmio: TMmio) -> Self {
        Self { ptr, mmio }
    }
    /// Controls for the Sticky Data Vault Entries
    ///
    /// Read value: [`dv::regs::StickydatavaultctrlReadVal`]; Write value: [`dv::regs::StickydatavaultctrlWriteVal`]
    pub fn sticky_data_vault_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Stickydatavaultctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn sticky_data_vault_entry(
        &self,
    ) -> ureg::Array<10, ureg::Array<12, ureg::RegRef<crate::dv::meta::StickyDataVaultEntry, &TMmio>>>
    {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x28 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Controls for the Non-Sticky Data Vault Entries
    ///
    /// Read value: [`dv::regs::StickydatavaultctrlReadVal`]; Write value: [`dv::regs::StickydatavaultctrlWriteVal`]
    pub fn non_sticky_data_vault_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Nonstickydatavaultctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x208 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn nonsticky_data_vault_entry(
        &self,
    ) -> ureg::Array<
        10,
        ureg::Array<12, ureg::RegRef<crate::dv::meta::NonstickyDataVaultEntry, &TMmio>>,
    > {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x230 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Non-Sticky Scratch Register Controls
    ///
    /// Read value: [`dv::regs::StickylockablescratchregctrlReadVal`]; Write value: [`dv::regs::StickylockablescratchregctrlWriteVal`]
    pub fn non_sticky_lockable_scratch_reg_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Nonstickylockablescratchregctrl, &TMmio>>
    {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x410 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn non_sticky_lockable_scratch_reg(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Nonstickylockablescratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x438 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn non_sticky_generic_scratch_reg(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Nonstickygenericscratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x460 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Sticky Scratch Register Controls
    ///
    /// Read value: [`dv::regs::StickylockablescratchregctrlReadVal`]; Write value: [`dv::regs::StickylockablescratchregctrlWriteVal`]
    pub fn sticky_lockable_scratch_reg_ctrl(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Stickylockablescratchregctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x480 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn sticky_lockable_scratch_reg(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Stickylockablescratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x4a0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct StickydatavaultctrlReadVal(u32);
    impl StickydatavaultctrlReadVal {
        /// Lock writes to this entry. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> StickydatavaultctrlWriteVal {
            StickydatavaultctrlWriteVal(self.0)
        }
    }
    impl From<u32> for StickydatavaultctrlReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StickydatavaultctrlReadVal> for u32 {
        fn from(val: StickydatavaultctrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StickydatavaultctrlWriteVal(u32);
    impl StickydatavaultctrlWriteVal {
        /// Lock writes to this entry. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for StickydatavaultctrlWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StickydatavaultctrlWriteVal> for u32 {
        fn from(val: StickydatavaultctrlWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StickylockablescratchregctrlReadVal(u32);
    impl StickylockablescratchregctrlReadVal {
        /// Lock writes to the Scratch registers. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> StickylockablescratchregctrlWriteVal {
            StickylockablescratchregctrlWriteVal(self.0)
        }
    }
    impl From<u32> for StickylockablescratchregctrlReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StickylockablescratchregctrlReadVal> for u32 {
        fn from(val: StickylockablescratchregctrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StickylockablescratchregctrlWriteVal(u32);
    impl StickylockablescratchregctrlWriteVal {
        /// Lock writes to the Scratch registers. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for StickylockablescratchregctrlWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StickylockablescratchregctrlWriteVal> for u32 {
        fn from(val: StickylockablescratchregctrlWriteVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    pub mod selector {}
}
pub mod meta {
    //! Additional metadata needed by ureg.
    pub type Stickydatavaultctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::StickydatavaultctrlReadVal,
        crate::dv::regs::StickydatavaultctrlWriteVal,
    >;
    pub type StickyDataVaultEntry = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Nonstickydatavaultctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::StickydatavaultctrlReadVal,
        crate::dv::regs::StickydatavaultctrlWriteVal,
    >;
    pub type NonstickyDataVaultEntry = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Nonstickylockablescratchregctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::StickylockablescratchregctrlReadVal,
        crate::dv::regs::StickylockablescratchregctrlWriteVal,
    >;
    pub type Nonstickylockablescratchreg = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Nonstickygenericscratchreg = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Stickylockablescratchregctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::StickylockablescratchregctrlReadVal,
        crate::dv::regs::StickylockablescratchregctrlWriteVal,
    >;
    pub type Stickylockablescratchreg = ureg::ReadWriteReg32<0, u32, u32>;
}
