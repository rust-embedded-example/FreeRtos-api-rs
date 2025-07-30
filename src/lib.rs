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
 *
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

#![no_std]
#![allow(non_camel_case_types)]

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

// Re-export commonly used types and functions
pub use base::*;

use core::panic::PanicInfo;

/// Panic handler - 在 no_std 环境中必需的
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}