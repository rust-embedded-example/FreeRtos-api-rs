//! FreeRTOS project definitions and constants.
//!
//! Provides FFI bindings for FreeRTOS utility macros (`pdMS_TO_TICKS`, etc.)
//! and Rust-level constants for the most commonly used FreeRTOS definitions.
//!
//! # Constants
//!
//! These are provided as Rust constants so they can be used at compile time
//! without requiring an FFI call:
//!
//! - `PD_TRUE` / `PD_FALSE` — Boolean return values
//! - `PD_PASS` / `PD_FAIL` — Operation success/failure
//! - [`PORT_MAX_DELAY`] — Wait indefinitely
//!
//! # Scheduler States
//!
//! | Constant                         | Value |
//! |----------------------------------|-------|
//! | [`TASK_SCHEDULER_NOT_STARTED`]   | 0     |
//! | [`TASK_SCHEDULER_RUNNING`]       | 1     |
//! | [`TASK_SCHEDULER_SUSPENDED`]     | 2     |

use crate::base::{FreeRtosBaseType, FreeRtosTickType};

// Re-export from base for convenience
pub use crate::base::{PD_TRUE as pdTRUE, PD_FALSE as pdFALSE, PD_PASS as pdPASS, PD_FAIL as pdFAIL, PORT_MAX_DELAY};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - UTILITY FUNCTIONS
//===========================================================================

unsafe extern "C" {
    /// Converts milliseconds to FreeRTOS ticks (runtime version).
    ///
    /// Prefer the compile-time [`ms_to_ticks`] when the tick rate is known.
    pub fn freertos_rs_ms_to_ticks(time_in_ms: FreeRtosTickType) -> FreeRtosTickType;

    /// Converts ticks to milliseconds (runtime version).
    pub fn freertos_rs_ticks_to_ms(time_in_ticks: FreeRtosTickType) -> FreeRtosTickType;

    /// Returns the `portTICK_PERIOD_MS` constant value from FreeRTOS config.
    pub fn freertos_rs_get_port_tick_period_ms() -> FreeRtosTickType;
}

//===========================================================================
// SCHEDULER STATE CONSTANTS
//===========================================================================

/// Scheduler has not been started (`taskSCHEDULER_NOT_STARTED`).
pub const TASK_SCHEDULER_NOT_STARTED: FreeRtosBaseType = 0;

/// Scheduler is running normally (`taskSCHEDULER_RUNNING`).
pub const TASK_SCHEDULER_RUNNING: FreeRtosBaseType = 1;

/// Scheduler is suspended (`taskSCHEDULER_SUSPENDED`).
pub const TASK_SCHEDULER_SUSPENDED: FreeRtosBaseType = 2;

//===========================================================================
// ERROR CODE CONSTANTS (projdefs.h / queue.h)
//===========================================================================

/// Queue is empty (`errQUEUE_EMPTY`).
pub const ERR_QUEUE_EMPTY: FreeRtosBaseType = -1;

/// Queue is full (`errQUEUE_FULL`).
pub const ERR_QUEUE_FULL: FreeRtosBaseType = -2;

/// Memory allocation failed (`errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY`).
pub const ERR_COULD_NOT_ALLOCATE_REQUIRED_MEMORY: FreeRtosBaseType = -3;

/// Queue blocked (`errQUEUE_BLOCKED`).
pub const ERR_QUEUE_BLOCKED: FreeRtosBaseType = -4;

/// Queue yield (`errQUEUE_YIELD`).
pub const ERR_QUEUE_YIELD: FreeRtosBaseType = -5;

//===========================================================================
// EVENT GROUP CONTROL BIT CONSTANTS (event_groups.h)
//===========================================================================

use crate::base::FreeRtosEventBits;

/// Clear events on exit bit (`eventCLEAR_EVENTS_ON_EXIT_BIT`).
pub const EVENT_CLEAR_EVENTS_ON_EXIT_BIT: FreeRtosEventBits = 0x01000000;

/// Unblocked due to bit set (`eventUNBLOCKED_DUE_TO_BIT_SET`).
pub const EVENT_UNBLOCKED_DUE_TO_BIT_SET: FreeRtosEventBits = 0x02000000;

/// Wait for all bits (`eventWAIT_FOR_ALL_BITS`).
pub const EVENT_WAIT_FOR_ALL_BITS: FreeRtosEventBits = 0x04000000;

/// Event bits control bytes mask (`eventEVENT_BITS_CONTROL_BYTES`).
pub const EVENT_EVENT_BITS_CONTROL_BYTES: FreeRtosEventBits = 0xFF000000;

//===========================================================================
// COMPILE-TIME UTILITY FUNCTIONS
//===========================================================================

/// Converts milliseconds to FreeRTOS ticks at compile time.
///
/// # Note
///
/// This uses a default tick period of 1 ms. The actual conversion depends
/// on `configTICK_RATE_HZ` in `FreeRTOSConfig.h`. Adjust this function
/// or use the FFI version [`freertos_rs_ms_to_ticks`] for runtime accuracy.
pub const fn ms_to_ticks(ms: FreeRtosTickType) -> FreeRtosTickType {
    ms
}

/// Converts FreeRTOS ticks to milliseconds at compile time.
pub const fn ticks_to_ms(ticks: FreeRtosTickType) -> FreeRtosTickType {
    ticks
}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

const _: () = assert!(TASK_SCHEDULER_NOT_STARTED == 0);
const _: () = assert!(TASK_SCHEDULER_RUNNING == 1);
const _: () = assert!(TASK_SCHEDULER_SUSPENDED == 2);

const _: () = assert!(pdTRUE == 1);
const _: () = assert!(pdFALSE == 0);
const _: () = assert!(pdPASS == 1);
const _: () = assert!(pdFAIL == 0);

const _: () = assert!(ms_to_ticks(100) == 100);
const _: () = assert!(ticks_to_ms(100) == 100);

const _: () = assert!(ERR_QUEUE_EMPTY == -1);
const _: () = assert!(ERR_QUEUE_FULL == -2);
const _: () = assert!(ERR_COULD_NOT_ALLOCATE_REQUIRED_MEMORY == -3);
const _: () = assert!(ERR_QUEUE_BLOCKED == -4);
const _: () = assert!(ERR_QUEUE_YIELD == -5);

const _: () = assert!(EVENT_CLEAR_EVENTS_ON_EXIT_BIT == 0x01000000);
const _: () = assert!(EVENT_UNBLOCKED_DUE_TO_BIT_SET == 0x02000000);
const _: () = assert!(EVENT_WAIT_FOR_ALL_BITS == 0x04000000);
const _: () = assert!(EVENT_EVENT_BITS_CONTROL_BYTES == 0xFF000000);
