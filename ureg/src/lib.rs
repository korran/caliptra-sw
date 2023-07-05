/*++
Licensed under the Apache-2.0 license.
--*/

//! # ureg register abstraction
//!
//! This crate contains traits and types used by MMIO register-access code
//! generated by [`ureg-codegen`].

#![no_std]

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod opt_riscv;

use core::{default::Default, marker::PhantomData, mem::MaybeUninit};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UintType {
    U8,
    U16,
    U32,
    U64,
}

pub trait Uint: Clone + Copy + private::Sealed {
    const TYPE: UintType;
}

mod private {
    pub trait Sealed {}
}

impl Uint for u8 {
    const TYPE: UintType = UintType::U8;
}
impl private::Sealed for u8 {}

impl Uint for u16 {
    const TYPE: UintType = UintType::U16;
}
impl private::Sealed for u16 {}

impl Uint for u32 {
    const TYPE: UintType = UintType::U32;
}
impl private::Sealed for u32 {}

impl Uint for u64 {
    const TYPE: UintType = UintType::U64;
}
impl private::Sealed for u64 {}

/// The root trait for metadata describing a MMIO register.
///
/// Implementors of this trait should also consider implementing
/// [`ReadableReg`], [`ReadableReg`], and [`ResettableReg`].
pub trait RegType {
    /// The raw type of the register: typically `u8`, `u16`, `u32`, or `u64`.
    type Raw: Uint;
}

/// A trait used to describe an MMIO register with a reset value.
pub trait ResettableReg: RegType {
    /// The reset value of the register
    ///
    /// This is the value the register is expected to have after the peripheral
    /// has been reset. It is used as the initial value of the first closure
    /// parameter to [`RegRef::write`].
    const RESET_VAL: Self::Raw;
}

/// A trait used to describe an MMIO register that can be read from.
pub trait ReadableReg: RegType {
    type ReadVal: Copy + From<Self::Raw>;
}

/// A trait used to describe an MMIO register that can be written to.
pub trait WritableReg: RegType {
    type WriteVal: Copy + From<Self::Raw> + Into<Self::Raw>;
}

/// A convenient RegType implementation intended for read-only 32-bit fields.
///
/// The code-generator may use this type to cut down on the number of RegType
/// implementations created, making it easier for the compiler to deduplicate
/// generic functions that act on registers with the same field layout.
pub struct ReadOnlyReg32<TReadVal: Copy + From<u32>> {
    phantom: PhantomData<TReadVal>,
}
impl<TReadVal: Copy + From<u32>> RegType for ReadOnlyReg32<TReadVal> {
    type Raw = u32;
}
impl<TReadVal: Copy + From<u32>> ReadableReg for ReadOnlyReg32<TReadVal> {
    type ReadVal = TReadVal;
}

/// A convenient RegType implementation intended for write-only 32-bit fields.
///
/// The code-generator may use this type to cut down on the number of RegType
/// implementations created, making it easier for the compiler to deduplicate
/// generic functions that act on registers with the same field layout.
pub struct WriteOnlyReg32<const RESET_VAL: u32, TWriteVal: Copy + From<u32> + Into<u32>> {
    phantom: PhantomData<TWriteVal>,
}
impl<const RESET_VAL: u32, TWriteVal: Copy + From<u32> + Into<u32>> RegType
    for WriteOnlyReg32<RESET_VAL, TWriteVal>
{
    type Raw = u32;
}
impl<const RESET_VAL: u32, TWriteVal: Copy + From<u32> + Into<u32>> ResettableReg
    for WriteOnlyReg32<RESET_VAL, TWriteVal>
{
    const RESET_VAL: Self::Raw = RESET_VAL;
}
impl<const RESET_VAL: u32, TWriteVal: Copy + From<u32> + Into<u32>> WritableReg
    for WriteOnlyReg32<RESET_VAL, TWriteVal>
{
    type WriteVal = TWriteVal;
}

/// A convenient RegType implementation intended for read-write 32-bit fields.
///
/// The code-generator may use this type to cut down on the number of RegType
/// implementations created, making it easier for the compiler to deduplicate
/// generic functions that act on registers with the same field layout.
pub struct ReadWriteReg32<
    const RESET_VAL: u32,
    TReadVal: Copy + From<u32>,
    TWriteVal: Copy + From<u32> + Into<u32>,
> {
    phantom: PhantomData<(TReadVal, TWriteVal)>,
}
impl<const RESET_VAL: u32, TReadVal: Copy + From<u32>, TWriteVal: Copy + From<u32> + Into<u32>>
    RegType for ReadWriteReg32<RESET_VAL, TReadVal, TWriteVal>
{
    type Raw = u32;
}
impl<const RESET_VAL: u32, TReadVal: Copy + From<u32>, TWriteVal: Copy + From<u32> + Into<u32>>
    ReadableReg for ReadWriteReg32<RESET_VAL, TReadVal, TWriteVal>
{
    type ReadVal = TReadVal;
}
impl<const RESET_VAL: u32, TReadVal: Copy + From<u32>, TWriteVal: Copy + From<u32> + Into<u32>>
    ResettableReg for ReadWriteReg32<RESET_VAL, TReadVal, TWriteVal>
{
    const RESET_VAL: Self::Raw = RESET_VAL;
}
impl<const RESET_VAL: u32, TReadVal: Copy + From<u32>, TWriteVal: Copy + From<u32> + Into<u32>>
    WritableReg for ReadWriteReg32<RESET_VAL, TReadVal, TWriteVal>
{
    type WriteVal = TWriteVal;
}
/// A trait for performing volatile reads from a pointer. On real
/// systems, [`RealMmio`] is typically used to implement this trait, but other
/// implementations may be used for testing or simulation.
pub trait Mmio: Sized {
    /// Performs (or simulates) a volatile read from `src` and returns the read value.
    ///
    /// # Safety
    ///
    /// Same as [`core::ptr::read_volatile`].
    unsafe fn read_volatile<T: Uint>(&self, src: *const T) -> T;

    /// # Safety
    ///
    /// Caller must ensure that the safety requirements of
    /// [`core::ptr::read_volatile`] are met for every location between src and
    /// `dst.add(LEN)`, and that src.add(LEN) does not wrap around the address
    /// space.
    ///
    /// Also, the caller must ensure that the safety requirements of
    /// [`core::ptr::write`] are met for every location between dst and
    /// `dst.add(LEN)`, and that dst.add(LEN) does not wrap around the address
    /// space.
    unsafe fn read_volatile_array<const LEN: usize, T: Uint>(&self, dst: *mut T, src: *mut T) {
        read_volatile_slice(self, dst, src, LEN);
    }
}

/// A trait for performing volatile writes (or reads via Mmio supertrait) to/from a
/// pointer. On real systems, [`RealMmioMut`] is typically used to implement
/// this trait, but other implementations may be used for testing or simulation.
pub trait MmioMut: Mmio {
    /// Performs (or simulates) a volatile write of `src` to `dst`.
    ///
    /// # Safety
    ///
    /// Same as [`core::ptr::write_volatile`].
    unsafe fn write_volatile<T: Uint>(&self, dst: *mut T, src: T);

    /// # Safety
    ///
    /// Caller must ensure that the safety requirements of
    /// [`core::ptr::write_volatile`] are met for every location between dst and
    /// `dst.add(LEN)`, and that dst.add(LEN) does not wrap around the address
    /// space.
    unsafe fn write_volatile_array<const LEN: usize, T: Uint>(&self, dst: *mut T, src: *const [T; LEN]) {
        write_volatile_slice(self, dst, &*src);
    }
}

/// A zero-sized type that implements the Mmio trait with real reads from the
/// provided pointer.
#[derive(Clone, Copy, Default)]
pub struct RealMmio<'a>(PhantomData<&'a ()>);
impl Mmio for RealMmio<'_> {
    /// Performs a volatile read from `src` and returns the read value.
    ///
    /// # Safety
    ///
    /// Same as [`core::ptr::read_volatile`].
    unsafe fn read_volatile<T: Clone + Copy>(&self, src: *const T) -> T {
        core::ptr::read_volatile(src)
    }

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe fn read_volatile_array<const LEN: usize, T: Uint>(&self, dst: *mut T, src: *mut T) {
        opt_riscv::read_volatile_array::<LEN, T>(dst, src)
    }
}

/// A zero-sized type that implements the Mmio and MmioMut traits with real
/// reads and writes from/to the provided pointer.
#[derive(Clone, Copy, Default)]
pub struct RealMmioMut<'a>(PhantomData<&'a mut ()>);
impl Mmio for RealMmioMut<'_> {
    /// Performs a volatile read from `src` and returns the read value.
    ///
    /// # Safety
    ///
    /// Same as [`core::ptr::read_volatile`].
    #[inline(always)]
    unsafe fn read_volatile<T: Clone + Copy>(&self, src: *const T) -> T {
        core::ptr::read_volatile(src)
    }
}
impl MmioMut for RealMmioMut<'_> {
    /// Performs a volatile write of `src` to `dst`.
    ///
    /// # Safety
    ///
    /// Same as [`core::ptr::write_volatile`].
    #[inline(always)]
    unsafe fn write_volatile<T: Clone + Copy>(&self, dst: *mut T, src: T) {
        core::ptr::write_volatile(dst, src);
    }

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe fn write_volatile_array<const LEN: usize, T: Uint>(&self, dst: *mut T, src: *const [T; LEN]) {
        opt_riscv::write_volatile_array::<LEN, T>(dst, src)
    }
}
impl<TMmio: Mmio> Mmio for &TMmio {
    #[inline(always)]
    unsafe fn read_volatile<T: Uint>(&self, src: *const T) -> T {
        (*self).read_volatile(src)
    }
    unsafe fn read_volatile_array<const LEN: usize, T: Uint>(&self, dst: *mut T, src: *mut T) {
        (*self).read_volatile_array::<LEN, T>(dst, src)
    }
}
impl<TMmio: MmioMut> MmioMut for &TMmio {
    #[inline(always)]
    unsafe fn write_volatile<T: Uint>(&self, dst: *mut T, src: T) {
        (*self).write_volatile(dst, src)
    }
    #[inline(always)]
    unsafe fn write_volatile_array<const LEN: usize, T: Uint>(&self, dst: *mut T, src: *const [T; LEN]) {
        (*self).write_volatile_array::<LEN, T>(dst, src)
    }
}
pub trait FromMmioPtr {
    /// The raw type of the register (typically `u8`, `u16`, `u32`, or `u64`)
    type TRaw: Clone + Copy;
    /// The Mmio implementation to use; typically RealMmio.
    type TMmio: Mmio;

    /// `Self::STRIDE * mem::size_of::<TRaw>()` is this number of bytes between
    /// the addresses of two Self instances adjacent in memory.
    const STRIDE: usize;

    /// Constructs a FromMmioPtr implementation from a raw register pointer and
    /// Mmio implementation
    ///
    /// Using an Mmio implentation other than RealMmio is primarily for
    /// tests or simulations.
    ///
    /// # Safety
    ///
    /// This function is unsafe because `ptr` may be used for loads or stores at any time
    /// during the lifetime of the returned value. The caller to new is responsible for ensuring that:
    ///
    /// * `ptr` is non-null
    /// * `ptr` is valid for loads and stores of `Self::STRIDE * mem::size_of::<Self::Raw>`
    ///    bytes for the entire lifetime of the returned value.
    /// * `ptr` is properly aligned.
    unsafe fn from_ptr(ptr: *mut Self::TRaw, mmio: Self::TMmio) -> Self;
}

/// A reference to a strongly typed MMIO register.
///
/// The `TReg` type parameter describes the properties of the register.
/// In addition to implementing the `RegType` trait to describe the raw pointer
/// type (typically `u8`, `u16, `u32`, or `u64), TReg should consider
/// implementing the following traits:
///
/// * `ReadableReg` for registers that can be read from. If `TReg` implements `ReadableReg`,
///   `RegRef::read()` will be available.
/// * `WritableReg` for registers that can be written to. If `TReg` implements `WritableReg`,
///   `RegRef::modify()` will be available.
/// * `ResettableReg` for registers that have a defined reset value. If `TReg`
///    implements `ResettableReg` and `WritableReg`, `RegRef::write()` will be
///    available.
#[derive(Clone, Copy)]
pub struct RegRef<TReg: RegType, TMmio: Mmio> {
    mmio: TMmio,
    ptr: *mut TReg::Raw,
}
impl<TReg: RegType, TMmio: Mmio> RegRef<TReg, TMmio> {
    /// Creates a new RegRef from a raw register pointer and Mmio implementation.
    ///
    /// Using an Mmio implentation other than RealMmio is primarily for
    /// tests or simulations in the build environment.
    ///
    /// # Safety
    ///
    /// The caller must fulfill the same requirements as `RegRef::new`
    #[inline(always)]
    pub unsafe fn new_with_mmio(ptr: *mut TReg::Raw, mmio: TMmio) -> Self {
        Self { mmio, ptr }
    }
}

impl<TReg: RegType, TMmio: Mmio + Default> RegRef<TReg, TMmio> {
    /// Creates a new RegRef from a raw register pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because later (safe) calls to read() or write()
    /// will load or store from this pointer. The caller to new is responsible for ensuring that:
    ///
    /// * `ptr` is non-null
    /// * `ptr` is valid for loads and stores of `mem::size_of::<TReg::Raw>`
    ///    bytes for the entire lifetime of this RegRef.
    /// * `ptr` is properly aligned.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut TReg::Raw) -> Self {
        Self {
            mmio: Default::default(),
            ptr,
        }
    }

    /// Returns a pointer to the underlying MMIO register.
    ///
    /// # Safety
    ///
    /// This pointer can be used for volatile reads and writes at any time
    /// during the lifetime of Self. Callers are reponsible for ensuring that
    /// their use doesn't conflict with other accesses to this MMIO register.
    #[inline(always)]
    pub fn ptr(&self) -> *mut TReg::Raw {
        self.ptr
    }
}
impl<TReg: RegType, TMmio: Mmio> FromMmioPtr for RegRef<TReg, TMmio> {
    type TRaw = TReg::Raw;
    type TMmio = TMmio;
    const STRIDE: usize = 1;

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut Self::TRaw, mmio: Self::TMmio) -> Self {
        Self { ptr, mmio }
    }
}

impl<TReg: ReadableReg, TMmio: Mmio> RegRef<TReg, TMmio> {
    /// Peforms a volatile load from the underlying MMIO register.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ureg::{ReadableReg, RegRef, RealMmio, RegType};
    /// // Note: these types are typically generated by ureg-codegen.
    /// struct ControlReg();
    /// impl RegType for ControlReg {
    ///     type Raw = u32;
    /// }
    /// impl ReadableReg for ControlReg {
    ///     type ReadVal = ControlRegReadVal;
    /// }
    /// #[derive(Clone, Copy)]
    /// struct ControlRegReadVal(u32);
    /// impl From<u32> for ControlRegReadVal {
    ///     fn from(val: u32) -> Self {
    ///         Self(val)
    ///     }
    /// }
    /// impl ControlRegReadVal {
    ///     pub fn enabled(&self) -> bool {
    ///         (self.0 & 0x1) != 0
    ///     }
    ///     pub fn tx_len(&self) -> u8 {
    ///         ((self.0 >> 1) & 0xff) as u8
    ///     }
    /// }
    /// struct RegisterBlock(*mut u32);
    /// impl RegisterBlock {
    ///     fn control(&self) -> RegRef<ControlReg, RealMmio> { return unsafe { RegRef::new(self.0.add(4)) } }
    /// }
    ///
    /// let regs = RegisterBlock(0x3002_0000 as *mut u32);
    ///
    /// // Loads from address 0x3002_00010 twice
    /// if regs.control().read().enabled() {
    ///     println!("Active transaction of length {}", regs.control().read().tx_len());
    /// }
    ///
    /// // Loads from address 0x3002_00010 once
    /// let control_reg_val = regs.control().read();
    /// if control_reg_val.enabled() {
    ///     println!("Active transaction of length {}", control_reg_val.tx_len());
    /// }
    /// ```
    #[inline(always)]
    pub fn read(&self) -> TReg::ReadVal {
        let raw = unsafe { self.mmio.read_volatile(self.ptr) };
        TReg::ReadVal::from(raw)
    }
}

impl<TReg: ResettableReg + WritableReg, TMmio: MmioMut> RegRef<TReg, TMmio> {
    /// Peforms a volatile write to the underlying MMIO register.
    ///
    /// The `f` closure is used to build the the register value. It is
    /// immediately called with the reset value of the register, and returns
    /// the value that should be written to the register.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ureg::{ResettableReg, WritableReg, RealMmioMut, RegRef, RegType};
    /// // Note: these types are typically generated by ureg-codegen.
    /// struct ControlReg();
    /// impl RegType for ControlReg {
    ///     type Raw = u32;
    /// }
    /// impl WritableReg for ControlReg {
    ///     type WriteVal = ControlRegWriteVal;
    /// }
    /// impl ResettableReg for ControlReg {
    ///     const RESET_VAL: u32 = 0x1fe;
    /// }
    /// #[derive(Clone, Copy)]
    /// struct ControlRegWriteVal(u32);
    /// impl ControlRegWriteVal {
    ///     fn enabled(self, val: bool) -> ControlRegWriteVal {
    ///         ControlRegWriteVal((self.0 & !0x1) | u32::from(val) << 0)
    ///     }
    ///     fn tx_len(self, val: u8) -> ControlRegWriteVal {
    ///         ControlRegWriteVal((self.0 & !(0xff << 1)) | (u32::from(val) & 0xff) << 1)
    ///     }
    /// }
    /// impl From<u32> for ControlRegWriteVal {
    ///     fn from(val: u32) -> Self {
    ///         Self(val)
    ///     }
    /// }
    /// impl From<ControlRegWriteVal> for u32 {
    ///     fn from(val: ControlRegWriteVal) -> Self {
    ///         val.0
    ///     }
    /// }
    /// struct RegisterBlock(*mut u32);
    /// impl RegisterBlock {
    ///     fn control(&self) -> RegRef<ControlReg, RealMmioMut> { return unsafe { RegRef::new(self.0.add(4)) } }
    /// }
    ///
    /// let regs = RegisterBlock(0x3002_0000 as *mut u32);
    ///
    /// // Writes `0x1ff` to address 0x3002_0010
    /// regs.control().write(|w| w.enabled(true));
    ///
    /// // Writes `0x3` to address 0x3002_0010
    /// regs.control().write(|w| w.enabled(true).tx_len(1));
    ///
    /// ```
    #[inline(always)]
    pub fn write(&self, f: impl FnOnce(TReg::WriteVal) -> TReg::WriteVal) {
        let val = TReg::WriteVal::from(TReg::RESET_VAL);
        let val = f(val);
        unsafe { self.mmio.write_volatile(self.ptr, val.into()) }
    }
}

impl<TReg: ReadableReg + WritableReg, TMmio: MmioMut> RegRef<TReg, TMmio> {
    /// Peforms a load-modify-store with the underlying MMIO register.
    ///
    /// The `f` closure is used to build the the register value. It is
    /// immediately called with the loaded value of the register converted to
    /// `TReg::WriteVal`, and the closure must return the value that should be
    /// written to the register.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ureg::{ReadableReg, ResettableReg, WritableReg, RealMmioMut, RegRef, RegType};
    ///
    /// // Note: these types are typically generated by ureg-codegen.
    /// struct ControlReg();
    /// impl RegType for ControlReg {
    ///     type Raw = u32;
    /// }
    /// impl ReadableReg for ControlReg {
    ///     type ReadVal = ControlRegReadVal;
    /// }
    /// impl WritableReg for ControlReg {
    ///     type WriteVal = ControlRegWriteVal;
    /// }
    /// impl ResettableReg for ControlReg {
    ///     const RESET_VAL: u32 = 0x1fe;
    /// }
    /// #[derive(Clone, Copy)]
    /// struct ControlRegReadVal(u32);
    /// impl From<u32> for ControlRegReadVal {
    ///     fn from(val: u32) -> Self {
    ///         Self(val)
    ///     }
    /// }
    /// #[derive(Clone, Copy)]
    /// struct ControlRegWriteVal(u32);
    /// impl ControlRegWriteVal {
    ///     fn enabled(self, val: bool) -> ControlRegWriteVal {
    ///         ControlRegWriteVal((self.0 & !0x1) | u32::from(val) << 0)
    ///     }
    ///     fn tx_len(self, val: u8) -> ControlRegWriteVal {
    ///         ControlRegWriteVal((self.0 & !(0xff << 1)) | (u32::from(val) & 0xff) << 1)
    ///     }
    /// }
    /// impl From<u32> for ControlRegWriteVal {
    ///     fn from(val: u32) -> Self {
    ///         Self(val)
    ///     }
    /// }
    /// impl From<ControlRegWriteVal> for u32 {
    ///     fn from(val: ControlRegWriteVal) -> Self {
    ///         val.0
    ///     }
    /// }
    /// struct RegisterBlock(*mut u32);
    /// impl RegisterBlock {
    ///     fn control(&self) -> RegRef<ControlReg, RealMmioMut> { return unsafe { RegRef::new(self.0.add(4)) } }
    /// }
    ///
    /// let regs = RegisterBlock(0x3002_0000 as *mut u32);
    ///
    /// // Write `0x000` to 0x3002_0010
    /// regs.control().write(|w| w.enabled(false).tx_len(0));
    ///
    /// // Reads `0x000` from 0x3002_0010 and replaces it with `0x001`
    /// regs.control().modify(|w| w.enabled(true));
    ///
    /// // Reads `0x001` from 0x3002_0010 and replaces it with `0x003`
    /// regs.control().modify(|w| w.tx_len(1));
    ///
    /// // Reads `0x003` from 0x3002_0010 and replaces it with `0x002`
    /// regs.control().modify(|w| w.enabled(false));
    ///
    /// ```
    #[inline(always)]
    pub fn modify(&self, f: impl FnOnce(TReg::WriteVal) -> TReg::WriteVal) {
        let val = unsafe { self.mmio.read_volatile(self.ptr) };
        let wval = TReg::WriteVal::from(val);
        let val = f(wval);
        unsafe { self.mmio.write_volatile(self.ptr, val.into()) }
    }

    /// Peforms a load-modify-store with the underlying MMIO register.
    ///
    /// Same as [`RegRef::modify`], but the closure is also passed the read
    /// value as a parameter.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ureg::{ReadableReg, ResettableReg, WritableReg, RealMmioMut, RegRef, RegType};
    ///
    /// // Note: these types are typically generated by ureg-codegen.
    /// struct ControlReg();
    /// impl RegType for ControlReg {
    ///     type Raw = u32;
    /// }
    /// impl ReadableReg for ControlReg {
    ///     type ReadVal = ControlRegReadVal;
    /// }
    /// impl WritableReg for ControlReg {
    ///     type WriteVal = ControlRegWriteVal;
    /// }
    /// impl ResettableReg for ControlReg {
    ///     const RESET_VAL: u32 = 0x1fe;
    /// }
    /// #[derive(Clone, Copy)]
    /// struct ControlRegReadVal(u32);
    /// impl From<u32> for ControlRegReadVal {
    ///     fn from(val: u32) -> Self {
    ///         Self(val)
    ///     }
    /// }
    /// impl ControlRegReadVal {
    ///     pub fn enabled(&self) -> bool {
    ///         (self.0 & 0x1) != 0
    ///     }
    ///     pub fn tx_len(&self) -> u8 {
    ///         ((self.0 >> 1) & 0xff) as u8
    ///     }
    /// }
    /// #[derive(Clone, Copy)]
    /// struct ControlRegWriteVal(u32);
    /// impl ControlRegWriteVal {
    ///     pub fn enabled(self, val: bool) -> ControlRegWriteVal {
    ///         ControlRegWriteVal((self.0 & !0x1) | u32::from(val) << 0)
    ///     }
    ///     pub fn tx_len(self, val: u8) -> ControlRegWriteVal {
    ///         ControlRegWriteVal((self.0 & !(0xff << 1)) | (u32::from(val) & 0xff) << 1)
    ///     }
    /// }
    /// impl From<u32> for ControlRegWriteVal {
    ///     fn from(val: u32) -> Self {
    ///         Self(val)
    ///     }
    /// }
    /// impl From<ControlRegWriteVal> for u32 {
    ///     fn from(val: ControlRegWriteVal) -> Self {
    ///         val.0
    ///     }
    /// }
    /// struct RegisterBlock(*mut u32);
    /// impl RegisterBlock {
    ///     fn control(&self) -> RegRef<ControlReg, RealMmioMut> { return unsafe { RegRef::new(self.0.add(4)) } }
    /// }
    ///
    /// let regs = RegisterBlock(0x3002_0000 as *mut u32);
    ///
    /// // Toggles the least significant bit of 0x3002_0000, and leaves the others the same.
    /// regs.control().read_and_modify(|r, w| w.enabled(!r.enabled()));
    /// ```
    #[inline(always)]
    pub fn read_and_modify(&self, f: impl FnOnce(TReg::ReadVal, TReg::WriteVal) -> TReg::WriteVal) {
        let val = unsafe { self.mmio.read_volatile(self.ptr) };
        let rval = TReg::ReadVal::from(val);
        let wval = TReg::WriteVal::from(val);
        let val = f(rval, wval);
        unsafe { self.mmio.write_volatile(self.ptr, val.into()) }
    }
}

/// Represents an array of memory-mapped registers with a fixed-size stride.
///
/// Can be used for arrays of [`RegRef`], [`Array`], or register blocks.
pub struct Array<const LEN: usize, TItem: FromMmioPtr> {
    mmio: TItem::TMmio,
    ptr: *mut TItem::TRaw,
}
impl<const LEN: usize, TItem: FromMmioPtr<TMmio = TMmio>, TMmio: Mmio + Copy> Array<LEN, TItem> {
    /// Returns the item at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[inline(always)]
    pub fn at(&self, index: usize) -> TItem {
        if index >= LEN {
            panic!("register index out of bounds");
        }
        unsafe { TItem::from_ptr(self.ptr.add(index * TItem::STRIDE), self.mmio) }
    }

    /// Returns the item at `index`, or None if `index` is out of bounds.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<TItem> {
        if index >= LEN {
            None
        } else {
            unsafe {
                Some(TItem::from_ptr(
                    self.ptr.add(index * TItem::STRIDE),
                    self.mmio,
                ))
            }
        }
    }

    #[inline(always)]
    pub fn truncate<const NEW_LEN: usize>(&self) -> Array<NEW_LEN, TItem> {
        assert!(NEW_LEN <= LEN);
        Array {
            mmio: self.mmio,
            ptr: self.ptr,
        }
    }
}
impl<const LEN: usize, TMmio: Mmio + Default, TItem: FromMmioPtr<TMmio = TMmio>> Array<LEN, TItem> {
    /// Creates a new Array from a raw register pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because `ptr` may be used for loads or stores at any time
    /// during the lifetime of the array. The caller is responsible for ensuring that:
    ///
    /// * `ptr` is non-null
    /// * `ptr` is valid for loads and stores of `LEN * TItem::STRIDE * mem::size_of::<TItem::Raw>`
    ///    bytes for the entire lifetime of this array.
    /// * `ptr` is properly aligned.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut TItem::TRaw) -> Self {
        Self {
            mmio: Default::default(),
            ptr,
        }
    }
}
impl<const LEN: usize, TItem: FromMmioPtr> Array<LEN, TItem> {
    /// Creates a new Array from a raw register pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because `ptr` may be used for loads or stores at any time
    /// during the lifetime of the array. The caller is responsible for ensuring that:
    ///
    /// * `ptr` is non-null
    /// * `ptr` is valid for loads and stores of `LEN * TItem::STRIDE * mem::size_of::<TItem::Raw>`
    ///    bytes for the entire lifetime of this array.
    /// * `ptr` is properly aligned.
    #[inline(always)]
    pub unsafe fn new_with_mmio(ptr: *mut TItem::TRaw, mmio: TItem::TMmio) -> Self {
        Self { mmio, ptr }
    }
}

impl<const LEN: usize, TRaw: Uint, TReg: ReadableReg<ReadVal = TRaw, Raw = TRaw>, TMmio: Mmio>
    Array<LEN, RegRef<TReg, TMmio>>
{
    /// Reads the entire contents of the array from the underlying registers
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ureg::{Array, ReadableReg, RealMmio, RegRef, RegType};
    ///
    /// // Note: these types are typically generated by ureg-codegen.
    /// struct ValueReg();
    /// impl RegType for ValueReg{
    ///     type Raw = u32;
    /// }
    /// impl ReadableReg for ValueReg {
    ///     type ReadVal = u32;
    /// }
    /// struct RegisterBlock(*mut u32);
    /// impl RegisterBlock {
    ///     fn value(&self) -> Array<4, RegRef<ValueReg, RealMmio>> { return unsafe { Array::new(self.0.add(16)) } }
    /// }
    /// let regs = RegisterBlock(0x3002_0000 as *mut u32);
    ///
    /// // Reads words from 0x3002_0040, 0x3002_0044, 0x3002_0048, then 0x3002_004c
    /// let value: [u32; 4] = regs.value().read();
    /// ```
    #[inline(always)]
    pub fn read(&self) -> [TReg::ReadVal; LEN] {
        let mut result: MaybeUninit<[TReg::ReadVal; LEN]> = MaybeUninit::uninit();
        unsafe {
            self.mmio
                .read_volatile_array::<LEN, _>(result.as_mut_ptr() as *mut TReg::ReadVal, self.ptr);
            result.assume_init()
        }
    }

    #[inline(always)]
    pub fn ptr(self) -> *mut TReg::Raw {
        self.ptr
    }
}

#[inline(never)]
unsafe fn read_volatile_slice<T: Uint, TMmio: Mmio>(
    mmio: &TMmio,
    dst: *mut T,
    src: *mut T,
    len: usize,
) {
    for i in 0..len {
        dst.add(i).write(mmio.read_volatile(src.add(i)));
    }
}

#[inline(never)]
unsafe fn write_volatile_slice<T: Uint, TMmio: MmioMut>(mmio: &TMmio, dest: *mut T, val: &[T]) {
    #[allow(clippy::needless_range_loop)]
    for i in 0..val.len() {
        mmio.write_volatile(dest.add(i), val[i]);
    }
}

impl<
        const LEN: usize,
        TRaw: Uint,
        TReg: WritableReg<WriteVal = TRaw, Raw = TRaw>,
        TMmio: MmioMut,
    > Array<LEN, RegRef<TReg, TMmio>>
{
    /// Reads the entire contents of the array from the underlying registers
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ureg::{Array, WritableReg, RegRef, RealMmioMut, RegType};
    ///
    /// // Note: these types are typically generated by ureg-codegen.
    /// struct ValueReg();
    /// impl RegType for ValueReg{
    ///     type Raw = u32;
    /// }
    /// impl WritableReg for ValueReg {
    ///     type WriteVal = u32;
    /// }
    /// struct RegisterBlock(*mut u32);
    /// impl RegisterBlock {
    ///     fn value(&self) -> Array<4, RegRef<ValueReg, RealMmioMut>> { return unsafe { Array::new(self.0.add(16)) } }
    /// }
    /// let regs = RegisterBlock(0x3002_0000 as *mut u32);
    ///
    /// // Writes `0x10` to address `0x3002_0040`, `0x20` to `0x3002_0044`,
    /// // `0x30` to `0x3002_0048`, then `0x40` to `0x3002_004c`
    /// regs.value().write(&[0x10, 0x20, 0x30, 0x40]);
    /// ```
    #[inline(always)]
    pub fn write(&self, val: &[TRaw; LEN]) {
        unsafe {
            self.mmio.write_volatile_array(self.ptr, val);
        }
    }

    pub unsafe fn write_ptr(&self, val: *const [TRaw; LEN]) {
        self.mmio.write_volatile_array(self.ptr, val);
    }
}

impl<const LEN: usize, TItem: FromMmioPtr> FromMmioPtr for Array<LEN, TItem> {
    type TRaw = TItem::TRaw;
    type TMmio = TItem::TMmio;
    const STRIDE: usize = LEN * TItem::STRIDE;

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut Self::TRaw, mmio: Self::TMmio) -> Self {
        Self { ptr, mmio }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // To run inside the MIRI interpreter to detect undefined behavior in the
    // unsafe code, run with:
    // cargo +nightly miri test -p ureg --lib

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Pull {
        Down = 0,
        Up = 1,
        HiZ = 2,
        Default3 = 3,
    }
    impl Pull {
        fn down(&self) -> bool {
            *self == Self::Down
        }
        fn up(&self) -> bool {
            *self == Self::Up
        }
        fn hi_z(&self) -> bool {
            *self == Self::HiZ
        }
    }
    impl TryFrom<u32> for Pull {
        type Error = ();

        #[inline(always)]
        fn try_from(val: u32) -> Result<Self, ()> {
            match val {
                0 => Ok(Pull::Down),
                1 => Ok(Pull::Up),
                2 => Ok(Pull::HiZ),
                3 => Ok(Pull::Default3),
                _ => Err(()),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct PullSelector();
    impl PullSelector {
        pub fn down(self) -> Pull {
            Pull::Down
        }
        pub fn up(self) -> Pull {
            Pull::Up
        }
        pub fn hi_z(self) -> Pull {
            Pull::HiZ
        }
    }

    pub struct FifoReg();
    impl RegType for FifoReg {
        type Raw = u32;
    }
    impl ReadableReg for FifoReg {
        type ReadVal = u32;
    }
    impl WritableReg for FifoReg {
        type WriteVal = u32;
    }
    impl ResettableReg for FifoReg {
        const RESET_VAL: u32 = 0;
    }

    pub struct ControlReg();
    impl RegType for ControlReg {
        type Raw = u32;
    }
    impl ReadableReg for ControlReg {
        type ReadVal = ControlRegReadVal;
    }
    impl WritableReg for ControlReg {
        type WriteVal = ControlRegWriteVal;
    }
    impl ResettableReg for ControlReg {
        const RESET_VAL: u32 = 0;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ControlRegReadVal(u32);
    impl ControlRegReadVal {
        pub fn enabled(&self) -> bool {
            (self.0 & 0x1) != 0
        }
        pub fn pull(&self) -> Pull {
            Pull::try_from((self.0 >> 1) & 0x3).unwrap()
        }
    }
    impl From<u32> for ControlRegReadVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ControlRegWriteVal(u32);
    impl ControlRegWriteVal {
        pub fn enabled(self, val: bool) -> ControlRegWriteVal {
            ControlRegWriteVal((self.0 & !0x1) | u32::from(val))
        }
        pub fn pull(self, f: impl FnOnce(PullSelector) -> Pull) -> ControlRegWriteVal {
            ControlRegWriteVal((self.0 & !(0x3 << 1)) | (f(PullSelector()) as u32) << 1)
        }
    }
    impl From<u32> for ControlRegWriteVal {
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ControlRegWriteVal> for u32 {
        fn from(val: ControlRegWriteVal) -> Self {
            val.0
        }
    }

    pub struct MyRegisterBlock(*mut u32);
    impl MyRegisterBlock {
        pub unsafe fn new(ptr: *mut u32) -> Self {
            Self(ptr)
        }
        pub fn fifo(&self) -> RegRef<FifoReg, RealMmioMut> {
            unsafe { RegRef::new(self.0.wrapping_offset(0)) }
        }
        pub fn control(&self) -> RegRef<ControlReg, RealMmioMut> {
            unsafe { RegRef::new(self.0.wrapping_offset(1)) }
        }
    }

    #[test]
    pub fn test() {
        let mut fake_mem = [0u32; 32];
        let block = unsafe { MyRegisterBlock::new(fake_mem.as_mut_ptr()) };

        block.fifo().write(|_| 0xdeadbeef);
        assert_eq!(0xdeadbeef, block.fifo().read());

        block
            .control()
            .modify(|w| w.enabled(true).pull(|w| w.hi_z()));
        assert_eq!(0b101, block.control().read().0);
        assert!(block.control().read().enabled());
        assert_eq!(Pull::HiZ, block.control().read().pull());
        assert!(block.control().read().pull().hi_z());
        assert!(!block.control().read().pull().down());
        assert!(!block.control().read().pull().up());

        block.fifo().read();
    }

    #[test]
    pub fn test_reg_array() {
        let mut fake_mem = [0, 1, 2, 3, 4, 5, 6];
        let reg_array = unsafe {
            Array::<4, RegRef<ReadWriteReg32<0, u32, u32>, RealMmioMut>>::new(fake_mem.as_mut_ptr())
        };
        assert_eq!(reg_array.read(), [0, 1, 2, 3]);

        reg_array.write(&[10, 11, 12, 13]);
        assert_eq!(&fake_mem, &[10, 11, 12, 13, 4, 5, 6]);

        assert_eq!(&fake_mem[0] as *const _, reg_array.at(0).ptr);

        assert_eq!(&fake_mem[3] as *const _, reg_array.at(3).ptr);
    }

    #[test]
    pub fn test_reg_array_truncate() {
        let mut fake_mem = [0, 1, 2, 3, 4, 5, 6];
        let reg_array = unsafe {
            Array::<4, RegRef<ReadWriteReg32<0, u32, u32>, RealMmioMut>>::new(fake_mem.as_mut_ptr())
        };
        let truncated = reg_array.truncate::<3>();
        assert_eq!(truncated.read(), [0, 1, 2]);
    }

    #[test]
    #[should_panic]
    pub fn test_reg_array_oob_panic() {
        let mut fake_mem = [0, 1, 2, 3, 4, 5, 6];
        let reg_array =
            unsafe { Array::<4, RegRef<ControlReg, RealMmioMut>>::new(fake_mem.as_mut_ptr()) };
        reg_array.at(4);
    }
    #[test]
    #[should_panic]
    pub fn test_reg_array_truncate_panic() {
        let mut fake_mem = [0, 1, 2, 3, 4, 5, 6];
        let reg_array =
            unsafe { Array::<4, RegRef<ControlReg, RealMmioMut>>::new(fake_mem.as_mut_ptr()) };
        reg_array.truncate::<5>();
    }

    #[test]
    pub fn test_reg_array_of_arrays() {
        let mut fake_mem = [0, 1, 2, 3, 4, 5, 6];
        let reg_array = unsafe {
            Array::<3, Array<2, RegRef<FifoReg, RealMmioMut>>>::new(fake_mem.as_mut_ptr())
        };
        assert_eq!(reg_array.at(0).read(), [0, 1]);
        assert_eq!(reg_array.at(1).read(), [2, 3]);
        assert_eq!(reg_array.at(2).read(), [4, 5]);

        reg_array.at(0).write(&[10, 11]);
        reg_array.at(1).write(&[12, 13]);
        assert_eq!(&fake_mem, &[10, 11, 12, 13, 4, 5, 6]);

        assert_eq!(&fake_mem[0] as *const _, reg_array.at(0).ptr);

        assert_eq!(&fake_mem[2] as *const _, reg_array.at(1).ptr);
    }

    #[test]
    #[should_panic]
    pub fn test_reg_array_of_arrays_oob_panic() {
        let mut fake_mem = [0, 1, 2, 3, 4, 5, 6];
        let reg_array =
            unsafe { Array::<4, RegRef<ControlReg, RealMmioMut>>::new(fake_mem.as_mut_ptr()) };
        reg_array.at(4);
    }
}
