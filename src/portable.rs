//! `FreeRTOS` portable layer — memory management and heap functions.
//!
//! Provides FFI bindings for `FreeRTOS` memory management APIs and a
//! [`FreeRtosAllocator`] that implements Rust's `GlobalAlloc` trait,
//! enabling standard Rust allocation on top of the `FreeRTOS` heap.

use crate::base::{FreeRtosVoidPtr, FreeRtosHeapStats, FreeRtosConstVoidPtr};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MEMORY MANAGEMENT
//===========================================================================

unsafe extern "C" {
    /// Allocates memory from the `FreeRTOS` heap.
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

    /// Initializes memory blocks (`heap_3` only).
    pub fn freertos_rs_port_initialise_blocks();

    /// Resets the minimum-ever-free-heap-size counter.
    pub fn freertos_rs_port_reset_heap_minimum_ever_free_heap_size();

    /// Gets detailed heap statistics.
    pub fn freertos_rs_port_get_heap_stats(heap_stats: *mut FreeRtosHeapStats);

    /// Defines heap regions (`heap_5` only).
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

/// A global allocator backed by `FreeRTOS` `pvPortMalloc` / `vPortFree`.
///
/// Use this to enable standard Rust `alloc` types (`Vec`, `String`, `Box`, etc.)
/// on top of the `FreeRTOS` heap.
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
            unsafe { freertos_rs_port_malloc(layout.size()).cast::<u8>() }
        } else {
            // Over-allocate: extra (align - 1) + sizeof(usize) bytes to store the original pointer
            let total = layout.size().checked_add(layout.align() - 1 + core::mem::size_of::<usize>());
            let Some(total) = total else { return core::ptr::null_mut() };
            let raw = unsafe { freertos_rs_port_malloc(total).cast::<u8>() };
            if raw.is_null() {
                return core::ptr::null_mut();
            }
            // Calculate aligned offset, reserving space for the original pointer
            let raw_addr = raw as usize;
            let aligned = (raw_addr + core::mem::size_of::<usize>() + layout.align() - 1) & !(layout.align() - 1);
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

/// `FreeRTOS` `HeapRegion_t` — describes a memory region for `heap_5`.
///
/// Used with [`define_heap_regions`] to initialize the heap when using
/// the `heap_5` memory scheme (`configAPPLICATION_ALLOCATED_HEAP == 1`).
///
/// The array of `FreeRtosHeapRegion` must be terminated by an entry with
/// a null `start_address` and zero `size_in_bytes`.
///
/// # Example
///
/// ```rust,no_run
/// use freertos_api_rs::portable::{FreeRtosHeapRegion, define_heap_regions};
///
/// static mut REGION1: [u8; 8192] = [0u8; 8192];
/// static mut REGION2: [u8; 4096] = [0u8; 4096];
///
/// let regions = [
///     FreeRtosHeapRegion { start_address: unsafe { REGION1.as_mut_ptr() }, size_in_bytes: 8192 },
///     FreeRtosHeapRegion { start_address: unsafe { REGION2.as_mut_ptr() }, size_in_bytes: 4096 },
///     FreeRtosHeapRegion { start_address: core::ptr::null_mut(), size_in_bytes: 0 },
/// ];
/// unsafe { define_heap_regions(&regions) };
/// ```
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FreeRtosHeapRegion {
    /// Start address of the heap region.
    pub start_address: *mut u8,
    /// Size of the heap region in bytes.
    pub size_in_bytes: usize,
}

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
    unsafe { freertos_rs_port_get_heap_stats(core::ptr::from_mut(&mut stats)) };
    stats
}

/// Defines heap regions for `heap_5` memory allocation scheme.
///
/// The `regions` array must be terminated by an entry with a null
/// `start_address` and zero `size_in_bytes`.
///
/// # Safety
/// The regions array must remain valid for the lifetime of the application.
/// Must only be called once, before any heap allocation. Only valid when
/// using `heap_5` scheme (`configAPPLICATION_ALLOCATED_HEAP == 1`).
pub unsafe fn define_heap_regions(regions: &[FreeRtosHeapRegion]) {
    unsafe {
        freertos_rs_port_define_heap_regions(regions.as_ptr() as FreeRtosConstVoidPtr);
    }
}

/// Triggers a context switch (yield).
///
/// Causes the scheduler to run and potentially switch to a higher-priority task.
pub fn yield_task() {
    unsafe { freertos_rs_port_yield() };
}

/// Allocates zero-initialized memory from the `FreeRTOS` heap.
///
/// Returns a pointer to `num * size` bytes of zeroed memory, or null on failure.
///
/// # Safety
/// The returned pointer must be properly aligned for the intended type.
pub unsafe fn calloc(num: usize, size: usize) -> FreeRtosVoidPtr {
    unsafe { freertos_rs_port_calloc(num, size) }
}

/// Initializes memory block tracking (for `heap_1`/`heap_2` schemes).
pub fn initialise_blocks() {
    unsafe { freertos_rs_port_initialise_blocks() };
}

/// Allocates a task stack from the `FreeRTOS` heap.
///
/// Returns a pointer to the allocated stack, or null on failure.
///
/// # Safety
/// The `size` must be appropriate for the target architecture's stack requirements.
pub unsafe fn malloc_stack(size: usize) -> FreeRtosVoidPtr {
    unsafe { freertos_rs_port_malloc_stack(size) }
}

/// Frees a previously allocated task stack.
///
/// # Safety
/// `ptr` must be a valid pointer returned by `malloc_stack`.
pub unsafe fn free_stack(ptr: FreeRtosVoidPtr) {
    unsafe { freertos_rs_port_free_stack(ptr) };
}

/// Resets the heap state to initial conditions.
///
/// Typically used before restarting the scheduler.
pub fn heap_reset_state() {
    unsafe { freertos_rs_port_heap_reset_state() };
}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FreeRtosAllocator>();
};

// FreeRtosHeapRegion must match C HeapRegion_t layout: pointer + usize
const _: () = assert!(core::mem::size_of::<FreeRtosHeapRegion>() == core::mem::size_of::<*mut u8>() + core::mem::size_of::<usize>());
const _: () = assert!(core::mem::align_of::<FreeRtosHeapRegion>() >= core::mem::align_of::<*mut u8>());
