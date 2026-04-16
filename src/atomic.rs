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
