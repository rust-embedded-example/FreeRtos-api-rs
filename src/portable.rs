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

    /// Allocates memory for task stack from a separate heap.
    ///
    /// Wraps `pvPortMallocStack()`. Only available when
    /// `configSTACK_ALLOCATION_FROM_SEPARATE_HEAP == 1`. When that config
    /// option is 0, this resolves to `pvPortMalloc` via a C macro.
    pub fn freertos_rs_port_malloc_stack(size: usize) -> FreeRtosVoidPtr;

    /// Frees memory allocated with `pvPortMallocStack`.
    ///
    /// Wraps `vPortFreeStack()`. See `freertos_rs_port_malloc_stack`.
    pub fn freertos_rs_port_free_stack(ptr: FreeRtosVoidPtr);

    /// Resets the heap state.
    ///
    /// Wraps `vPortHeapResetState()`. Used to reset the heap to its initial
    /// state, typically before restarting the scheduler.
    pub fn freertos_rs_port_heap_reset_state();
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
        // pvPortMalloc typically guarantees portBYTE_ALIGNMENT alignment (8 bytes).
        // For larger alignment requirements, over-allocate and align manually.
        const MIN_ALIGN: usize = 8;
        if layout.align() <= MIN_ALIGN {
            unsafe { freertos_rs_port_malloc(layout.size()) as *mut u8 }
        } else {
            // Over-allocate: extra (align - 1) + sizeof(usize) bytes to store the original pointer
            let total = layout.size().checked_add(layout.align() - 1 + core::mem::size_of::<usize>());
            let total = match total {
                Some(t) => t,
                None => return core::ptr::null_mut(),
            };
            let raw = unsafe { freertos_rs_port_malloc(total) as *mut u8 };
            if raw.is_null() {
                return core::ptr::null_mut();
            }
            // Calculate aligned offset, reserving space for the original pointer
            let raw_addr = raw as usize;
            let aligned = (raw_addr + core::mem::size_of::<usize>() + layout.align() - 1) & !(layout.align() - 1);
            let _offset = aligned - raw_addr;
            // Store the original pointer just before the aligned address
            unsafe { core::ptr::write((aligned - core::mem::size_of::<usize>()) as *mut *mut u8, raw) };
            aligned as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        const MIN_ALIGN: usize = 8;
        if layout.align() <= MIN_ALIGN {
            unsafe { freertos_rs_port_free(ptr as FreeRtosVoidPtr) };
        } else {
            // Retrieve the original pointer stored before the aligned address
            let raw = unsafe { core::ptr::read((ptr as usize - core::mem::size_of::<usize>()) as *mut *mut u8) };
            unsafe { freertos_rs_port_free(raw as FreeRtosVoidPtr) };
        }
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
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FreeRtosAllocator>();
};
