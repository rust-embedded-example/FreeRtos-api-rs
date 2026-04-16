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
    /// `*destination = new_value`. Returns the previous value of `*destination`.
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

    /// Atomic compare-and-swap for pointers. Returns the previous value.
    pub fn freertos_rs_atomic_compare_and_swap_pointers_p32(
        destination: *mut *mut core::ffi::c_void,
        new_value: *mut core::ffi::c_void,
        expected_value: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
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

    /// Returns the current value.
    pub fn load(&self) -> u32 {
        unsafe { core::ptr::read(self.ptr) }
    }

    /// Sets the value directly (non-atomic, use with care).
    pub fn store(&self, value: u32) {
        unsafe { core::ptr::write(self.ptr, value) };
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
    /// Returns the previous value (check if it equals `expected` to know if swap succeeded).
    pub fn compare_and_swap(&self, expected: u32, new: u32) -> u32 {
        unsafe { freertos_rs_atomic_compare_and_swap_u32(self.ptr, new, expected) }
    }

    /// Atomically swaps with `new`, returning the previous value.
    pub fn swap(&self, new: u32) -> u32 {
        // FreeRTOS doesn't have Atomic_Exchange_u32 in V11.1.0,
        // so we use CAS in a loop (for correctness on contention).
        loop {
            let current = self.load();
            let prev = self.compare_and_swap(current, new);
            if prev == current {
                return prev;
            }
        }
    }
}

impl Drop for FreeRtosAtomicU32 {
    fn drop(&mut self) {
        unsafe { crate::portable::freertos_rs_port_free(self.ptr as *mut core::ffi::c_void) };
    }
}

// Safety: FreeRTOS atomic operations are thread-safe (they use ldrex/strex or
// equivalent on ARM, or interrupt masking on single-core systems).
unsafe impl Send for FreeRtosAtomicU32 {}
unsafe impl Sync for FreeRtosAtomicU32 {}

//===========================================================================
// UNIT TESTS
//===========================================================================

#[cfg(test)]
mod tests {
    #[test]
    fn test_atomic_function_signatures() {
        // This test just verifies the function signatures are correct by
        // referencing them. The actual atomic operations require FreeRTOS.
        use super::*;
        let _ = freertos_rs_atomic_add_u32 as unsafe extern "C" fn(*mut u32, u32) -> u32;
        let _ = freertos_rs_atomic_subtract_u32 as unsafe extern "C" fn(*mut u32, u32) -> u32;
        let _ = freertos_rs_atomic_increment_u32 as unsafe extern "C" fn(*mut u32) -> u32;
        let _ = freertos_rs_atomic_decrement_u32 as unsafe extern "C" fn(*mut u32) -> u32;
        let _ = freertos_rs_atomic_or_u32 as unsafe extern "C" fn(*mut u32, u32) -> u32;
        let _ = freertos_rs_atomic_and_u32 as unsafe extern "C" fn(*mut u32, u32) -> u32;
        let _ = freertos_rs_atomic_nand_u32 as unsafe extern "C" fn(*mut u32, u32) -> u32;
        let _ = freertos_rs_atomic_xor_u32 as unsafe extern "C" fn(*mut u32, u32) -> u32;
    }
}
