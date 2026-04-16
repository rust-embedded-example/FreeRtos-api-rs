//! FreeRTOS atomic operations module.
//!
//! Provides FFI bindings for FreeRTOS atomic operations on 32-bit values and
//! pointers. These are thin wrappers around the FreeRTOS `Atomic_*` functions
//! defined in `atomic.h`.
//!
//! All operations return the **previous** value before the atomic modification.
//!
//! # Safety
//!
//! These functions are inherently unsafe as they accept raw pointers.
//! Ensure the pointers are valid and properly aligned.

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - ATOMIC ARITHMETIC
//===========================================================================

unsafe extern "C" {
    /// Atomically adds `value` to `*addend`. Returns the previous value.
    pub fn freertos_rs_atomic_add_u32(addend: *mut u32, value: u32) -> u32;

    /// Atomically subtracts `value` from `*minuend`. Returns the previous value.
    pub fn freertos_rs_atomic_subtract_u32(minuend: *mut u32, value: u32) -> u32;

    /// Atomically increments `*addend`. Returns the previous value.
    pub fn freertos_rs_atomic_increment_u32(addend: *mut u32) -> u32;

    /// Atomically decrements `*minuend`. Returns the previous value.
    pub fn freertos_rs_atomic_decrement_u32(minuend: *mut u32) -> u32;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - ATOMIC BITWISE
//===========================================================================

unsafe extern "C" {
    /// Atomically performs `*destination |= value`. Returns the previous value.
    pub fn freertos_rs_atomic_or_u32(destination: *mut u32, value: u32) -> u32;

    /// Atomically performs `*destination &= value`. Returns the previous value.
    pub fn freertos_rs_atomic_and_u32(destination: *mut u32, value: u32) -> u32;

    /// Atomically performs `~(*destination & value)`. Returns the previous value.
    pub fn freertos_rs_atomic_nand_u32(destination: *mut u32, value: u32) -> u32;

    /// Atomically performs `*destination ^= value`. Returns the previous value.
    pub fn freertos_rs_atomic_xor_u32(destination: *mut u32, value: u32) -> u32;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - ATOMIC COMPARE-AND-SWAP
//===========================================================================

unsafe extern "C" {
    /// Atomically compares `*destination == expected_value`, and if so, sets
    /// `*destination = new_value`.
    ///
    /// **Returns `1` on success, `0` on failure** (ATOMIC_COMPARE_AND_SWAP_SUCCESS/FAILURE).
    /// This is NOT the previous value — it is a boolean status code.
    pub fn freertos_rs_atomic_compare_and_swap_u32(
        destination: *mut u32,
        new_value: u32,
        expected_value: u32,
    ) -> u32;

    /// Atomically swaps `*destination = new_value`. Returns the previous value.
    pub fn freertos_rs_atomic_swap_pointers_p32(
        destination: *mut *mut core::ffi::c_void,
        new_value: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;

    /// Atomic compare-and-swap for pointers.
    ///
    /// **Returns `1` on success, `0` on failure** (ATOMIC_COMPARE_AND_SWAP_SUCCESS/FAILURE).
    pub fn freertos_rs_atomic_compare_and_swap_pointers_p32(
        destination: *mut *mut core::ffi::c_void,
        new_value: *mut core::ffi::c_void,
        expected_value: *mut core::ffi::c_void,
    ) -> u32;
}

//===========================================================================
// SAFE WRAPPER - FreeRtosAtomicU32
//===========================================================================

/// A FreeRTOS-backed atomic `u32` value.
///
/// Provides safe, RAII-managed access to FreeRTOS atomic operations on a
/// `u32` value. The internal value is heap-allocated via `pvPortMalloc` to
/// ensure a stable address (required for atomic operations).
///
/// # Example
///
/// ```rust,no_run
/// use freertos_api_rs::atomic::FreeRtosAtomicU32;
///
/// let counter = FreeRtosAtomicU32::new(0).expect("alloc failed");
/// let prev = counter.fetch_add(1);
/// assert_eq!(prev, 0);
/// assert_eq!(counter.load(), 1);
/// ```
pub struct FreeRtosAtomicU32 {
    ptr: *mut u32,
}

impl FreeRtosAtomicU32 {
    /// Creates a new atomic `u32` with the given initial value.
    ///
    /// Returns `None` if memory allocation fails.
    pub fn new(value: u32) -> Option<Self> {
        let ptr = unsafe { crate::portable::freertos_rs_port_malloc(core::mem::size_of::<u32>()) };
        if ptr.is_null() {
            return None;
        }
        let ptr = ptr as *mut u32;
        unsafe { core::ptr::write(ptr, value) };
        Some(Self { ptr })
    }

    /// Returns the current value atomically.
    ///
    /// Uses a volatile read. On ARM Cortex-M7, aligned 32-bit reads are
    /// naturally atomic for single-core systems (FreeRTOS atomics use
    /// interrupt masking or ldrex/strex).
    pub fn load(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.ptr) }
    }

    /// Atomically sets the value, returning the previous value.
    pub fn store(&self, value: u32) -> u32 {
        self.swap(value)
    }

    /// Atomically adds `value`, returning the previous value.
    pub fn fetch_add(&self, value: u32) -> u32 {
        unsafe { freertos_rs_atomic_add_u32(self.ptr, value) }
    }

    /// Atomically subtracts `value`, returning the previous value.
    pub fn fetch_sub(&self, value: u32) -> u32 {
        unsafe { freertos_rs_atomic_subtract_u32(self.ptr, value) }
    }

    /// Atomically increments by 1, returning the previous value.
    pub fn fetch_inc(&self) -> u32 {
        unsafe { freertos_rs_atomic_increment_u32(self.ptr) }
    }

    /// Atomically decrements by 1, returning the previous value.
    pub fn fetch_dec(&self) -> u32 {
        unsafe { freertos_rs_atomic_decrement_u32(self.ptr) }
    }

    /// Atomically performs `|= value`, returning the previous value.
    pub fn fetch_or(&self, value: u32) -> u32 {
        unsafe { freertos_rs_atomic_or_u32(self.ptr, value) }
    }

    /// Atomically performs `&= value`, returning the previous value.
    pub fn fetch_and(&self, value: u32) -> u32 {
        unsafe { freertos_rs_atomic_and_u32(self.ptr, value) }
    }

    /// Atomically performs `^= value`, returning the previous value.
    pub fn fetch_xor(&self, value: u32) -> u32 {
        unsafe { freertos_rs_atomic_xor_u32(self.ptr, value) }
    }

    /// Atomically performs NAND (`~(&value)`), returning the previous value.
    pub fn fetch_nand(&self, value: u32) -> u32 {
        unsafe { freertos_rs_atomic_nand_u32(self.ptr, value) }
    }

    /// Atomically compares and swaps.
    ///
    /// If the current value equals `expected`, sets to `new`.
    /// Returns `true` if the swap succeeded, `false` otherwise.
    ///
    /// Note: FreeRTOS's `Atomic_CompareAndSwap_u32` returns success/failure (0/1),
    /// NOT the previous value.
    pub fn compare_and_swap(&self, expected: u32, new: u32) -> bool {
        unsafe { freertos_rs_atomic_compare_and_swap_u32(self.ptr, new, expected) != 0 }
    }

    /// Atomically swaps with `new`, returning the previous value.
    ///
    /// Uses a CAS loop: reads the current value, then attempts CAS.
    /// FreeRTOS CAS returns success/failure, so we read the current value
    /// with a volatile read and retry until CAS succeeds.
    pub fn swap(&self, new: u32) -> u32 {
        let mut current = unsafe { core::ptr::read_volatile(self.ptr) };
        loop {
            if unsafe { freertos_rs_atomic_compare_and_swap_u32(self.ptr, new, current) != 0 } {
                return current;
            }
            // CAS failed — another writer changed the value. Re-read and retry.
            current = unsafe { core::ptr::read_volatile(self.ptr) };
        }
    }
}

impl Drop for FreeRtosAtomicU32 {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { crate::portable::freertos_rs_port_free(self.ptr as *mut core::ffi::c_void) };
        }
    }
}

// Safety: FreeRTOS atomic operations are thread-safe (they use ldrex/strex or
// equivalent on ARM, or interrupt masking on single-core systems).
unsafe impl Send for FreeRtosAtomicU32 {}
unsafe impl Sync for FreeRtosAtomicU32 {}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

// Verify FFI function signatures match expected types
const _: unsafe extern "C" fn(*mut u32, u32) -> u32 = freertos_rs_atomic_add_u32;
const _: unsafe extern "C" fn(*mut u32, u32) -> u32 = freertos_rs_atomic_subtract_u32;
const _: unsafe extern "C" fn(*mut u32) -> u32 = freertos_rs_atomic_increment_u32;
const _: unsafe extern "C" fn(*mut u32) -> u32 = freertos_rs_atomic_decrement_u32;
const _: unsafe extern "C" fn(*mut u32, u32) -> u32 = freertos_rs_atomic_or_u32;
const _: unsafe extern "C" fn(*mut u32, u32) -> u32 = freertos_rs_atomic_and_u32;
const _: unsafe extern "C" fn(*mut u32, u32) -> u32 = freertos_rs_atomic_nand_u32;
const _: unsafe extern "C" fn(*mut u32, u32) -> u32 = freertos_rs_atomic_xor_u32;

// CAS returns success/failure (0/1), not the previous value
const _: unsafe extern "C" fn(*mut u32, u32, u32) -> u32 = freertos_rs_atomic_compare_and_swap_u32;

// Pointer CAS also returns u32 success/failure code
const _: unsafe extern "C" fn(*mut *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> u32 = freertos_rs_atomic_compare_and_swap_pointers_p32;

// FreeRtosAtomicU32 is Send + Sync (atomic operations are inherently thread-safe)
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FreeRtosAtomicU32>();
};
