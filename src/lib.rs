/*!
 * FreeRTOS Rust API Wrapper
 *
 * This crate provides Rust bindings for FreeRTOS APIs through a two-layer architecture:
 * 1. C wrapper layer (api.c) - Provides C wrapper functions for FreeRTOS APIs
 * 2. Rust FFI layer - Provides Rust-friendly bindings to the C wrappers
 *
 * The crate is organized into modules following the FreeRTOS source structure:
 * - `base` - Basic types and error definitions
 * - `task` - Task management functions
 * - `portable` - Memory management and portable layer functions
 * - `projdefs` - Project definitions and constants
 * - `queue` - Queue management functions
 * - `semphr` - Semaphore and mutex functions
 * - `event_groups` - Event group functions
 * - `timers` - Software timer functions
 * - `stream_buffer` - Stream buffer functions
 * - `message_buffer` - Message buffer functions
 * - `croutine` - Co-routine functions (deprecated)
 *
 * # Safety
 *
 * All functions in this crate are marked as `unsafe` because they interface with
 * C code and FreeRTOS kernel functions. Callers must ensure:
 * - FreeRTOS is properly configured and initialized
 * - Memory management follows FreeRTOS conventions
 * - Interrupt safety requirements are met
 */

#![no_std]
#[allow(non_snake_case)]

// Module declarations
pub mod base;
pub mod task;
pub mod portable;
pub mod projdefs;
pub mod queue;
pub mod semphr;
pub mod event_groups;
pub mod timers;
pub mod stream_buffer;
pub mod message_buffer;
pub mod croutine;
pub mod atomic;
pub mod list;

// Re-export commonly used types and functions
pub use base::*;

use task::{freertos_rs_task_delay,
    freertos_rs_task_create,
    freertos_rs_task_start_scheduler};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Panic handler - 在 no_std 环境中必需的
extern "C" fn led_task(_params: *mut core::ffi::c_void) {
    loop {
        // Toggle LED
        unsafe {
            freertos_rs_task_delay(500); // 500ms delay
        }
    }
}

// Main function
#[unsafe(no_mangle)]
pub extern "C" fn rust_create_led_task() {
    // Create a task
    let task_handle: *mut *const core::ffi::c_void = core::ptr::null_mut();
    
    unsafe {
        freertos_rs_task_create(
            led_task,
            b"LED_Task\0".as_ptr(),
            128, // Stack size
            core::ptr::null_mut(),
            1, // Priority
            task_handle
        );
        
        // Start the scheduler
        freertos_rs_task_start_scheduler();
    }
}


