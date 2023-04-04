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
    pub fn sha512_acc_csr() -> Self {
        unsafe { Self::new(0x30021000 as *mut u32) }
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
    /// SHA lock register for SHA access, reading 0 will set the lock, Write 1 to clear the lock
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`sha512_acc::regs::LockReadVal`]; Write value: [`sha512_acc::regs::LockWriteVal`]
    pub fn lock(&self) -> ureg::RegRef<crate::sha512_acc::meta::Lock, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Stores the user that locked the SHA
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn user(&self) -> ureg::RegRef<crate::sha512_acc::meta::User, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(4 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Stores the requested mode for the SHA to execute.
    /// SHA Supports both SHA384 and SHA512 modes of operation.
    /// SHA Supports streaming mode - SHA is computed on a stream of incoming data to datain register.
    ///             mailbox mode - SHA is computed on LENGTH bytes of data stored in the mailbox from START_ADDRESS.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`sha512_acc::regs::ModeReadVal`]; Write value: [`sha512_acc::regs::ModeWriteVal`]
    pub fn mode(&self) -> ureg::RegRef<crate::sha512_acc::meta::Mode, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(8 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// The start address for FW controlled SHA performed on data stored in the mailbox.
    /// Start Address must be dword aligned.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn start_address(&self) -> ureg::RegRef<crate::sha512_acc::meta::StartAddress, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0xc / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// The length of data to be processed in bytes.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn dlen(&self) -> ureg::RegRef<crate::sha512_acc::meta::Dlen, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x10 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Data in register for SHA Streaming function
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn datain(&self) -> ureg::RegRef<crate::sha512_acc::meta::Datain, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x14 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// For Streaming Function, indicates that the initiator is done streaming.
    /// For the Mailbox SHA Function, indicates that the SHA can begin execution.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`sha512_acc::regs::ExecuteReadVal`]; Write value: [`sha512_acc::regs::ExecuteWriteVal`]
    pub fn execute(&self) -> ureg::RegRef<crate::sha512_acc::meta::Execute, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x18 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Status register indicating when the requested function is complete
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`sha512_acc::regs::StatusReadVal`]; Write value: [`sha512_acc::regs::StatusWriteVal`]
    pub fn status(&self) -> ureg::RegRef<crate::sha512_acc::meta::Status, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x1c / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// 16 32-bit registers storing the 512-bit digest output in
    /// big-endian representation.
    /// [br]Caliptra Access: RO
    /// [br]SOC Access:      RO
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    pub fn digest(&self) -> ureg::Array<16, ureg::RegRef<crate::sha512_acc::meta::Digest, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x20 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// SHA Accelerator control flows.
    /// [br]Zeroize the SHA engine internal registers.
    /// [br]Caliptra Access: RW
    /// [br]SOC Access:      RW
    ///
    /// Read value: [`sha512_acc::regs::ControlReadVal`]; Write value: [`sha512_acc::regs::ControlWriteVal`]
    pub fn control(&self) -> ureg::RegRef<crate::sha512_acc::meta::Control, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0x60 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct ControlReadVal(u32);
    impl ControlReadVal {
        /// Zeroize all internal registers
        #[inline(always)]
        pub fn zeroize(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> ControlWriteVal {
            ControlWriteVal(self.0)
        }
    }
    impl From<u32> for ControlReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ControlReadVal> for u32 {
        fn from(val: ControlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ControlWriteVal(u32);
    impl ControlWriteVal {
        /// Zeroize all internal registers
        #[inline(always)]
        pub fn zeroize(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for ControlWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ControlWriteVal> for u32 {
        fn from(val: ControlWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ExecuteReadVal(u32);
    impl ExecuteReadVal {
        ///
        #[inline(always)]
        pub fn execute(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> ExecuteWriteVal {
            ExecuteWriteVal(self.0)
        }
    }
    impl From<u32> for ExecuteReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ExecuteReadVal> for u32 {
        fn from(val: ExecuteReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ExecuteWriteVal(u32);
    impl ExecuteWriteVal {
        ///
        #[inline(always)]
        pub fn execute(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for ExecuteWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ExecuteWriteVal> for u32 {
        fn from(val: ExecuteWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct LockReadVal(u32);
    impl LockReadVal {
        ///
        #[inline(always)]
        pub fn lock(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> LockWriteVal {
            LockWriteVal(self.0)
        }
    }
    impl From<u32> for LockReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<LockReadVal> for u32 {
        fn from(val: LockReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct LockWriteVal(u32);
    impl LockWriteVal {
        ///
        #[inline(always)]
        pub fn lock(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for LockWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<LockWriteVal> for u32 {
        fn from(val: LockWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ModeReadVal(u32);
    impl ModeReadVal {
        ///
        #[inline(always)]
        pub fn mode(&self) -> super::enums::ShaCmdE {
            super::enums::ShaCmdE::try_from((self.0 >> 0) & 3).unwrap()
        }
        /// Default behavior assumes that data in mailbox is little endian,
        /// When set to 0, data from mailbox will be swizzled from little to big endian at the byte level.
        /// When set to 1, data from the mailbox will be loaded into SHA as-is.
        /// [br]Caliptra Access: RW
        /// [br]SOC Access:      RW
        #[inline(always)]
        pub fn endian_toggle(&self) -> bool {
            ((self.0 >> 2) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        pub fn modify(self) -> ModeWriteVal {
            ModeWriteVal(self.0)
        }
    }
    impl From<u32> for ModeReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ModeReadVal> for u32 {
        fn from(val: ModeReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ModeWriteVal(u32);
    impl ModeWriteVal {
        ///
        #[inline(always)]
        pub fn mode(
            self,
            f: impl FnOnce(super::enums::selector::ShaCmdESelector) -> super::enums::ShaCmdE,
        ) -> Self {
            Self(
                (self.0 & !(3 << 0))
                    | (u32::from(f(super::enums::selector::ShaCmdESelector())) << 0),
            )
        }
        /// Default behavior assumes that data in mailbox is little endian,
        /// When set to 0, data from mailbox will be swizzled from little to big endian at the byte level.
        /// When set to 1, data from the mailbox will be loaded into SHA as-is.
        /// [br]Caliptra Access: RW
        /// [br]SOC Access:      RW
        #[inline(always)]
        pub fn endian_toggle(self, val: bool) -> Self {
            Self((self.0 & !(1 << 2)) | (u32::from(val) << 2))
        }
    }
    impl From<u32> for ModeWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ModeWriteVal> for u32 {
        fn from(val: ModeWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StatusReadVal(u32);
    impl StatusReadVal {
        /// Valid bit, indicating that the digest is complete
        #[inline(always)]
        pub fn valid(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
    }
    impl From<u32> for StatusReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StatusReadVal> for u32 {
        fn from(val: StatusReadVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(u32)]
    pub enum ShaCmdE {
        ShaStream384 = 0,
        ShaStream512 = 1,
        ShaMbox384 = 2,
        ShaMbox512 = 3,
    }
    impl ShaCmdE {
        #[inline(always)]
        pub fn sha_stream_384(&self) -> bool {
            *self == Self::ShaStream384
        }
        #[inline(always)]
        pub fn sha_stream_512(&self) -> bool {
            *self == Self::ShaStream512
        }
        #[inline(always)]
        pub fn sha_mbox_384(&self) -> bool {
            *self == Self::ShaMbox384
        }
        #[inline(always)]
        pub fn sha_mbox_512(&self) -> bool {
            *self == Self::ShaMbox512
        }
    }
    impl TryFrom<u32> for ShaCmdE {
        type Error = ();
        #[inline(always)]
        fn try_from(val: u32) -> Result<ShaCmdE, ()> {
            match val {
                0 => Ok(Self::ShaStream384),
                1 => Ok(Self::ShaStream512),
                2 => Ok(Self::ShaMbox384),
                3 => Ok(Self::ShaMbox512),
                _ => Err(()),
            }
        }
    }
    impl From<ShaCmdE> for u32 {
        fn from(val: ShaCmdE) -> Self {
            val as u32
        }
    }
    pub mod selector {
        pub struct ShaCmdESelector();
        impl ShaCmdESelector {
            #[inline(always)]
            pub fn sha_stream_384(&self) -> super::ShaCmdE {
                super::ShaCmdE::ShaStream384
            }
            #[inline(always)]
            pub fn sha_stream_512(&self) -> super::ShaCmdE {
                super::ShaCmdE::ShaStream512
            }
            #[inline(always)]
            pub fn sha_mbox_384(&self) -> super::ShaCmdE {
                super::ShaCmdE::ShaMbox384
            }
            #[inline(always)]
            pub fn sha_mbox_512(&self) -> super::ShaCmdE {
                super::ShaCmdE::ShaMbox512
            }
        }
    }
}
pub mod meta {
    //! Additional metadata needed by ureg.
    pub type Lock = ureg::ReadWriteReg32<
        0,
        crate::sha512_acc::regs::LockReadVal,
        crate::sha512_acc::regs::LockWriteVal,
    >;
    pub type User = ureg::ReadOnlyReg32<u32>;
    pub type Mode = ureg::ReadWriteReg32<
        0,
        crate::sha512_acc::regs::ModeReadVal,
        crate::sha512_acc::regs::ModeWriteVal,
    >;
    pub type StartAddress = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Dlen = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Datain = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Execute = ureg::ReadWriteReg32<
        0,
        crate::sha512_acc::regs::ExecuteReadVal,
        crate::sha512_acc::regs::ExecuteWriteVal,
    >;
    pub type Status = ureg::ReadOnlyReg32<crate::sha512_acc::regs::StatusReadVal>;
    pub type Digest = ureg::ReadOnlyReg32<u32>;
    pub type Control = ureg::ReadWriteReg32<
        0,
        crate::sha512_acc::regs::ControlReadVal,
        crate::sha512_acc::regs::ControlWriteVal,
    >;
}
