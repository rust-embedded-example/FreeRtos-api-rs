//! # `FreeRTOS` Rust API Wrapper
//!
//! This crate provides safe Rust bindings for the [FreeRTOS](https://www.freertos.org/)
//! real-time operating system kernel. It enables writing `FreeRTOS` applications in Rust
//! with type-safe, ergonomic wrappers around the full `FreeRTOS` API surface.
//!
//! # Architecture
//!
//! The crate uses a three-layer architecture to interface with the `FreeRTOS` C kernel:
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │        Your Rust Application        │
//! ├─────────────────────────────────────┤
//! │  Safe Rust Wrappers (RAII types)    │  ← Task, Queue<T>, Mutex, Timer, ...
//! ├─────────────────────────────────────┤
//! │  Rust FFI Declarations              │  ← unsafe extern "C" { ... }
//! ├─────────────────────────────────────┤
//! │  C Wrapper Layer (freertos-api-rust.c)│  ← freertos_rs_*() functions
//! ├─────────────────────────────────────┤
//! │  FreeRTOS C Kernel                  │  ← xTaskCreate(), xQueueSend(), ...
//! └─────────────────────────────────────┘
//! ```
//!
//! **Layer 1 — C Wrapper (`freertos-api-rust.c`):**
//! Provides `freertos_rs_*()` wrapper functions that call through to the native
//! `FreeRTOS` APIs. This layer handles conditional compilation (`#if` guards) for
//! optional `FreeRTOS` features like static allocation, MPU support, and SMP.
//!
//! **Layer 2 — Rust FFI Declarations (`src/*.rs`):**
//! Raw `unsafe extern "C"` function declarations matching the C wrappers. These
//! are the lowest-level Rust interface and should generally not be used directly.
//!
//! **Layer 3 — Safe Rust Wrappers:**
//! RAII wrapper types that provide memory safety, type safety, and ergonomic APIs.
//! These manage resource lifetimes through `Drop` implementations and return
//! `Result` types instead of raw error codes.
//!
//! # Quick Start
//!
//! ## Creating Tasks
//!
//! ```rust,no_run
//! use freertos_api_rs::task::Task;
//! use freertos_api_rs::projdefs::ms_to_ticks;
//!
//! // Task function (signature must match FreeRtosTaskFunction)
//! unsafe extern "C" fn my_task(_param: *mut core::ffi::c_void) {
//!     loop {
//!         // Task work here
//!         freertos_api_rs::task::delay(100);
//!     }
//! }
//! ```
//!
//! ## Using a Queue
//!
//! ```rust,no_run
//! use freertos_api_rs::queue::Queue;
//!
//! let queue: Queue<u32> = Queue::new(10).expect("Failed to create queue");
//! queue.send(&42, 100).expect("Send timeout");
//! if let Some(value) = queue.receive(100) {
//!     // Process value
//! }
//! ```
//!
//! ## Using a Mutex
//!
//! ```rust,no_run
//! use freertos_api_rs::semphr::Mutex;
//!
//! let mutex = Mutex::new().expect("Failed to create mutex");
//! if mutex.lock(100) {
//!     // Critical section
//!     mutex.unlock();
//! }
//! ```
//!
//! # Build Process
//!
//! This crate is designed for bare-metal ARM Cortex-M targets and compiles
//! as a `staticlib` (producing a `.a` archive):
//!
//! 1. Build the Rust crate:
//!    ```sh
//!    cargo build --release --target thumbv7em-none-eabihf
//!    ```
//!
//! 2. Copy the generated `.o` files and `freertos-api-rust.c` to your C project
//!    that includes the `FreeRTOS` kernel source.
//!
//! 3. Compile `freertos-api-rust.c` with your C toolchain (which has `FreeRTOS`
//!    headers available).
//!
//! 4. Link everything together.
//!
//! # Module Organization
//!
//! | Module           | `FreeRTOS` Header         | Description                        |
//! |------------------|-------------------------|------------------------------------|
//! | [`base`]         | —                       | Core types, enums, constants       |
//! | [`task`]         | `task.h`                | Task management                    |
//! | [`queue`]        | `queue.h`               | Queue management                   |
//! | [`semphr`]       | `semphr.h`              | Semaphores and mutexes             |
//! | [`timers`]       | `timers.h`              | Software timers                    |
//! | [`event_groups`] | `event_groups.h`        | Event groups                       |
//! | [`stream_buffer`]| `stream_buffer.h`       | Stream buffers                     |
//! | [`message_buffer`]| `message_buffer.h`     | Message buffers                    |
//! | [`portable`]     | `portable.h`            | Memory management, heap allocator  |
//! | [`projdefs`]     | `projdefs.h`            | Constants and utility functions    |
//! | [`atomic`]       | `atomic.h`              | Atomic operations                  |
//! | [`list`]         | `list.h`                | Linked list operations             |
//!
//! # Safety
//!
//! All functions interfacing with the `FreeRTOS` kernel are inherently `unsafe`
//! because they interact with C code and depend on the `FreeRTOS` scheduler state.
//! The safe wrapper types in each module encapsulate this unsafety and ensure:
//!
//! - Resources are properly released when dropped (`Drop` trait)
//! - NULL handles are checked and converted to `Result` types
//! - ISR-safe variants are distinguished by naming convention (`_from_isr`)
//!
//! Callers must ensure:
//! - `FreeRTOS` is properly configured (FreeRTOSConfig.h) and initialized
//! - The scheduler is started before using most APIs
//! - ISR-context functions are only called from interrupt handlers
//! - Task functions never return (use `loop {}` or call `vTaskDelete(NULL)`)
//!
//! # Feature Flags
//!
//! This crate mirrors `FreeRTOS` configuration through conditional compilation
//! in the C wrapper layer. The following `FreeRTOS` `config` options affect
//! which APIs are available:
//!
//! - `configSUPPORT_STATIC_ALLOCATION` — Static allocation variants
//! - `configSUPPORT_DYNAMIC_ALLOCATION` — Dynamic allocation variants
//! - `configUSE_MUTEXES` — Mutex APIs
//! - `configUSE_RECURSIVE_MUTEXES` — Recursive mutex APIs
//! - `configUSE_COUNTING_SEMAPHORES` — Counting semaphore APIs
//! - `configUSE_TIMERS` — Software timer APIs
//! - `configUSE_QUEUE_SETS` — Queue set APIs
//! - `configUSE_TRACE_FACILITY` — Debug/tracing APIs
//! - `configNUMBER_OF_CORES` — SMP/multi-core APIs
//!
//! ## Cargo Features
//!
//! - `panic-handler` (default) — Provides a minimal `#[panic_handler]` that
//!   loops forever. **Disable this** if your application provides its own
//!   panic handler to avoid duplicate-symbol linker errors.

#![no_std]
#![allow(non_snake_case)]
// FFI wrapper functions are inherently pure queries or constructors where the
// return value should not be discarded, but annotating 128+ functions with
// #[must_use] would add noise without improving safety — callers already know
// these wrap C APIs. Similarly, many Result-returning wrappers simply propagate
// the underlying FreeRTOS return code; a full # Errors doc section for each
// would be redundant with the function's name and type signature.
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]

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
pub mod atomic;
pub mod list;

// Re-export commonly used types from base
pub use base::{
    FreeRtosBaseType, FreeRtosUBaseType, FreeRtosTickType,
    FreeRtosTaskHandle, FreeRtosQueueHandle, FreeRtosSemaphoreHandle,
    FreeRtosMutexHandle, FreeRtosTimerHandle, FreeRtosEventGroupHandle,
    FreeRtosStreamBufferHandle, FreeRtosMessageBufferHandle,
    FreeRtosVoidPtr, FreeRtosConstVoidPtr,
    FreeRtosError, FreeRtosNotifyAction, FreeRtosTaskState,
    FreeRtosTimeOut, FreeRtosTaskStatusFfi,
    FreeRtosMemoryRegion, FreeRtosTaskParameters,
    FreeRtosTaskFunction, FreeRtosTimerCallback, FreeRtosPendedFunction,
    FreeRtosEventBits, FreeRtosConfigStackDepthType,
    FreeRtosQueueSendPosition,
    PD_TRUE, PD_FALSE, PD_PASS, PD_FAIL, PORT_MAX_DELAY,
    TSK_IDLE_PRIORITY, TSK_NO_AFFINITY, TSK_DEFAULT_INDEX_TO_NOTIFY,
};

/// Default panic handler for `no_std` `FreeRTOS` environments.
///
/// Only compiled when the `panic-handler` feature is enabled. If your
/// application provides its own `#[panic_handler]`, leave this feature
/// disabled to avoid duplicate-symbol errors.
#[cfg(feature = "panic-handler")]
use core::panic::PanicInfo;

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
