/*!
 * FreeRTOS Atomic Operations Module
 * 
 * This module provides unsafe extern "C" declarations for FreeRTOS atomic operation functions.
 * The actual implementations are in api.c as C wrapper functions.
 * 
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

// No imports needed for atomic operations

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - ATOMIC OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for Atomic_Add_u32()
    /// Atomically adds to a 32-bit unsigned value
    pub fn freertos_rs_atomic_add_u32(
        addend: *mut u32,
        value: u32
    ) -> u32;

    /// Wrapper for Atomic_Subtract_u32()
    /// Atomically subtracts from a 32-bit unsigned value
    pub fn freertos_rs_atomic_subtract_u32(
        minuend: *mut u32,
        value: u32
    ) -> u32;

    /// Wrapper for Atomic_Increment_u32()
    /// Atomically increments a 32-bit unsigned value
    pub fn freertos_rs_atomic_increment_u32(
        addend: *mut u32
    ) -> u32;

    /// Wrapper for Atomic_Decrement_u32()
    /// Atomically decrements a 32-bit unsigned value
    pub fn freertos_rs_atomic_decrement_u32(
        minuend: *mut u32
    ) -> u32;

    /// Wrapper for Atomic_OR_u32()
    /// Atomically performs bitwise OR on a 32-bit unsigned value
    pub fn freertos_rs_atomic_or_u32(
        destination: *mut u32,
        value: u32
    ) -> u32;

    /// Wrapper for Atomic_AND_u32()
    /// Atomically performs bitwise AND on a 32-bit unsigned value
    pub fn freertos_rs_atomic_and_u32(
        destination: *mut u32,
        value: u32
    ) -> u32;

    /// Wrapper for Atomic_NAND_u32()
    /// Atomically performs bitwise NAND on a 32-bit unsigned value
    pub fn freertos_rs_atomic_nand_u32(
        destination: *mut u32,
        value: u32
    ) -> u32;

    /// Wrapper for Atomic_XOR_u32()
    /// Atomically performs bitwise XOR on a 32-bit unsigned value
    pub fn freertos_rs_atomic_xor_u32(
        destination: *mut u32,
        value: u32
    ) -> u32;

    /// Wrapper for Atomic_CompareAndSwap_u32()
    /// Atomically compares and swaps a 32-bit unsigned value
    pub fn freertos_rs_atomic_compare_and_swap_u32(
        destination: *mut u32,
        new_value: u32,
        expected_value: u32
    ) -> u32;

    /// Wrapper for Atomic_SwapPointers_p32()
    /// Atomically swaps two 32-bit pointers
    pub fn freertos_rs_atomic_swap_pointers_p32(
        destination: *mut *mut core::ffi::c_void,
        new_value: *mut core::ffi::c_void
    ) -> *mut core::ffi::c_void;

    /// Wrapper for Atomic_CompareAndSwapPointers_p32()
    /// Atomically compares and swaps two 32-bit pointers
    pub fn freertos_rs_atomic_compare_and_swap_pointers_p32(
        destination: *mut *mut core::ffi::c_void,
        new_value: *mut core::ffi::c_void,
        expected_value: *mut core::ffi::c_void
    ) -> *mut core::ffi::c_void;
}
