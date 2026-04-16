//! FreeRTOS portable layer — memory management and heap functions.
//!
//! Provides FFI bindings for FreeRTOS memory management APIs and a
//! [`FreeRtosAllocator`] that implements Rust's `GlobalAlloc` trait,
//! enabling standard Rust allocation on top of the FreeRTOS heap.

use crate::base::{FreeRtosVoidPtr, FreeRtosHeapStats, FreeRtosConstVoidPtr};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MEMORY MANAGEMENT
//===========================================================================

unsafe extern "C" {
    /// Allocates memory from the FreeRTOS heap.
    ///
    /// Wraps `pvPortMalloc()`. Returns `NULL` if the requested size
    /// cannot be allocated.
    pub fn freertos_rs_port_malloc(wanted_size: usize) -> FreeRtosVoidPtr;

    /// Frees memory previously allocated with `pvPortMalloc`.
    pub fn freertos_rs_port_free(ptr: FreeRtosVoidPtr);

    /// Gets the total free heap space in bytes.
    pub fn freertos_rs_port_get_free_heap_size() -> usize;

    /// Gets the minimum free heap space since boot.
    pub fn freertos_rs_port_get_minimum_ever_free_heap_size() -> usize;

    /// Allocates zeroed memory (`pvPortCalloc`).
    pub fn freertos_rs_port_calloc(num: usize, size: usize) -> FreeRtosVoidPtr;

    /// Initializes memory blocks (heap_3 only).
    pub fn freertos_rs_port_initialise_blocks();

    /// Resets the minimum-ever-free-heap-size counter.
    pub fn freertos_rs_port_reset_heap_minimum_ever_free_heap_size();

    /// Gets detailed heap statistics.
    pub fn freertos_rs_port_get_heap_stats(heap_stats: *mut FreeRtosHeapStats);

    /// Defines heap regions (heap_5 only).
    pub fn freertos_rs_port_define_heap_regions(heap_regions: FreeRtosConstVoidPtr);

    /// Requests a context switch (`portYIELD`).
    pub fn freertos_rs_port_yield();
}

//===========================================================================
// SAFE WRAPPER - GLOBAL ALLOCATOR
//===========================================================================

/// A global allocator backed by FreeRTOS `pvPortMalloc` / `vPortFree`.
///
/// Use this to enable standard Rust `alloc` types (`Vec`, `String`, `Box`, etc.)
/// on top of the FreeRTOS heap.
///
/// # Example
///
/// ```rust,no_run
/// #[global_allocator]
/// static ALLOCATOR: freertos_api_rs::portable::FreeRtosAllocator =
///     freertos_api_rs::portable::FreeRtosAllocator;
/// ```
pub struct FreeRtosAllocator;

unsafe impl core::alloc::GlobalAlloc for FreeRtosAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { freertos_rs_port_malloc(layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { freertos_rs_port_free(ptr as FreeRtosVoidPtr) };
    }
}

//===========================================================================
// SAFE WRAPPER - HEAP STATISTICS
//===========================================================================

/// Returns the total free heap space in bytes.
pub fn get_free_heap_size() -> usize {
    unsafe { freertos_rs_port_get_free_heap_size() }
}

/// Returns the minimum free heap space since boot (or since last reset).
pub fn get_minimum_ever_free_heap_size() -> usize {
    unsafe { freertos_rs_port_get_minimum_ever_free_heap_size() }
}

/// Resets the minimum-ever-free-heap-size counter.
pub fn reset_heap_minimum_ever_free_heap_size() {
    unsafe { freertos_rs_port_reset_heap_minimum_ever_free_heap_size() }
}

/// Returns detailed heap statistics.
pub fn get_heap_stats() -> FreeRtosHeapStats {
    let mut stats = FreeRtosHeapStats::default();
    unsafe { freertos_rs_port_get_heap_stats(&mut stats as *mut FreeRtosHeapStats) };
    stats
}

//===========================================================================
// UNIT TESTS
//===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocator_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FreeRtosAllocator>();
    }
}
