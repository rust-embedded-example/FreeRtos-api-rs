//! `FreeRTOS` project definitions and constants.
//!
//! Provides FFI bindings for `FreeRTOS` utility macros (`pdMS_TO_TICKS`, etc.)
//! and Rust-level constants for the most commonly used `FreeRTOS` definitions.
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
//! | [`TASK_SCHEDULER_SUSPENDED`]     | 0     |
//! | [`TASK_SCHEDULER_NOT_STARTED`]   | 1     |
//! | [`TASK_SCHEDULER_RUNNING`]       | 2     |

use crate::base::{FreeRtosBaseType, FreeRtosTickType};

// Re-export from base for convenience
pub use crate::base::{PD_TRUE as pdTRUE, PD_FALSE as pdFALSE, PD_PASS as pdPASS, PD_FAIL as pdFAIL, PORT_MAX_DELAY};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - UTILITY FUNCTIONS
//===========================================================================

unsafe extern "C" {
    /// Converts milliseconds to `FreeRTOS` ticks (runtime version).
    ///
    /// Prefer the compile-time [`ms_to_ticks`] when the tick rate is known.
    pub fn freertos_rs_ms_to_ticks(time_in_ms: FreeRtosTickType) -> FreeRtosTickType;

    /// Converts ticks to milliseconds (runtime version).
    pub fn freertos_rs_ticks_to_ms(time_in_ticks: FreeRtosTickType) -> FreeRtosTickType;

    /// Returns the `portTICK_PERIOD_MS` constant value from `FreeRTOS` config.
    pub fn freertos_rs_get_port_tick_period_ms() -> FreeRtosTickType;
}

//===========================================================================
// SCHEDULER STATE CONSTANTS
//===========================================================================

/// Scheduler is suspended (`taskSCHEDULER_SUSPENDED`). Value: 0.
pub const TASK_SCHEDULER_SUSPENDED: FreeRtosBaseType = 0;

/// Scheduler has not been started (`taskSCHEDULER_NOT_STARTED`). Value: 1.
pub const TASK_SCHEDULER_NOT_STARTED: FreeRtosBaseType = 1;

/// Scheduler is running normally (`taskSCHEDULER_RUNNING`). Value: 2.
pub const TASK_SCHEDULER_RUNNING: FreeRtosBaseType = 2;

//===========================================================================
// ERROR CODE CONSTANTS (projdefs.h / queue.h)
//===========================================================================

/// Queue is empty (`errQUEUE_EMPTY`). Value is 0 (same as `pdFAIL`).
pub const ERR_QUEUE_EMPTY: FreeRtosBaseType = 0;

/// Queue is full (`errQUEUE_FULL`). Value is 0 (same as `pdFAIL`).
pub const ERR_QUEUE_FULL: FreeRtosBaseType = 0;

/// Memory allocation failed (`errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY`).
pub const ERR_COULD_NOT_ALLOCATE_REQUIRED_MEMORY: FreeRtosBaseType = -1;

/// Queue blocked (`errQUEUE_BLOCKED`).
pub const ERR_QUEUE_BLOCKED: FreeRtosBaseType = -4;

/// Queue yield (`errQUEUE_YIELD`).
pub const ERR_QUEUE_YIELD: FreeRtosBaseType = -5;

//===========================================================================
// SIGNED/UNSIGNED BOOLEAN VARIANTS (projdefs.h)
//===========================================================================

use crate::base::FreeRtosUBaseType;

/// `pdFALSE` as signed `BaseType_t` (`pdFALSE_SIGNED`).
pub const PD_FALSE_SIGNED: FreeRtosBaseType = 0;

/// `pdTRUE` as signed `BaseType_t` (`pdTRUE_SIGNED`).
pub const PD_TRUE_SIGNED: FreeRtosBaseType = 1;

/// `pdFALSE` as unsigned `UBaseType_t` (`pdFALSE_UNSIGNED`).
pub const PD_FALSE_UNSIGNED: FreeRtosUBaseType = 0;

/// `pdTRUE` as unsigned `UBaseType_t` (`pdTRUE_UNSIGNED`).
pub const PD_TRUE_UNSIGNED: FreeRtosUBaseType = 1;

//===========================================================================
// ENDIANNESS CONSTANTS (projdefs.h — FreeRTOS+ components)
//===========================================================================

/// Little-endian flag (`pdFREERTOS_LITTLE_ENDIAN`). Value: 0.
pub const PD_FREERTOS_LITTLE_ENDIAN: u32 = 0;

/// Big-endian flag (`pdFREERTOS_BIG_ENDIAN`). Value: 1.
pub const PD_FREERTOS_BIG_ENDIAN: u32 = 1;

/// Little-endian alias (`pdLITTLE_ENDIAN`).
pub const PD_LITTLE_ENDIAN: u32 = PD_FREERTOS_LITTLE_ENDIAN;

/// Big-endian alias (`pdBIG_ENDIAN`).
pub const PD_BIG_ENDIAN: u32 = PD_FREERTOS_BIG_ENDIAN;

//===========================================================================
// FREERTOS ERRNO CONSTANTS (projdefs.h — FreeRTOS+ components)
//===========================================================================

/// No error (`pdFREERTOS_ERRNO_NONE`). Value: 0.
pub const ERRNO_NONE: FreeRtosBaseType = 0;
/// No such file or directory (`pdFREERTOS_ERRNO_ENOENT`). Value: 2.
pub const ERRNO_ENOENT: FreeRtosBaseType = 2;
/// Interrupted system call (`pdFREERTOS_ERRNO_EINTR`). Value: 4.
pub const ERRNO_EINTR: FreeRtosBaseType = 4;
/// I/O error (`pdFREERTOS_ERRNO_EIO`). Value: 5.
pub const ERRNO_EIO: FreeRtosBaseType = 5;
/// No such device or address (`pdFREERTOS_ERRNO_ENXIO`). Value: 6.
pub const ERRNO_ENXIO: FreeRtosBaseType = 6;
/// Bad file number (`pdFREERTOS_ERRNO_EBADF`). Value: 9.
pub const ERRNO_EBADF: FreeRtosBaseType = 9;
/// No more processes / operation would block (`pdFREERTOS_ERRNO_EAGAIN`). Value: 11.
pub const ERRNO_EAGAIN: FreeRtosBaseType = 11;
/// Operation would block (`pdFREERTOS_ERRNO_EWOULDBLOCK`). Value: 11.
pub const ERRNO_EWOULDBLOCK: FreeRtosBaseType = 11;
/// Not enough memory (`pdFREERTOS_ERRNO_ENOMEM`). Value: 12.
pub const ERRNO_ENOMEM: FreeRtosBaseType = 12;
/// Permission denied (`pdFREERTOS_ERRNO_EACCES`). Value: 13.
pub const ERRNO_EACCES: FreeRtosBaseType = 13;
/// Bad address (`pdFREERTOS_ERRNO_EFAULT`). Value: 14.
pub const ERRNO_EFAULT: FreeRtosBaseType = 14;
/// Mount device busy (`pdFREERTOS_ERRNO_EBUSY`). Value: 16.
pub const ERRNO_EBUSY: FreeRtosBaseType = 16;
/// File exists (`pdFREERTOS_ERRNO_EEXIST`). Value: 17.
pub const ERRNO_EEXIST: FreeRtosBaseType = 17;
/// Cross-device link (`pdFREERTOS_ERRNO_EXDEV`). Value: 18.
pub const ERRNO_EXDEV: FreeRtosBaseType = 18;
/// No such device (`pdFREERTOS_ERRNO_ENODEV`). Value: 19.
pub const ERRNO_ENODEV: FreeRtosBaseType = 19;
/// Not a directory (`pdFREERTOS_ERRNO_ENOTDIR`). Value: 20.
pub const ERRNO_ENOTDIR: FreeRtosBaseType = 20;
/// Is a directory (`pdFREERTOS_ERRNO_EISDIR`). Value: 21.
pub const ERRNO_EISDIR: FreeRtosBaseType = 21;
/// Invalid argument (`pdFREERTOS_ERRNO_EINVAL`). Value: 22.
pub const ERRNO_EINVAL: FreeRtosBaseType = 22;
/// No space left on device (`pdFREERTOS_ERRNO_ENOSPC`). Value: 28.
pub const ERRNO_ENOSPC: FreeRtosBaseType = 28;
/// Illegal seek (`pdFREERTOS_ERRNO_ESPIPE`). Value: 29.
pub const ERRNO_ESPIPE: FreeRtosBaseType = 29;
/// Read only file system (`pdFREERTOS_ERRNO_EROFS`). Value: 30.
pub const ERRNO_EROFS: FreeRtosBaseType = 30;
/// Protocol driver not attached (`pdFREERTOS_ERRNO_EUNATCH`). Value: 42.
pub const ERRNO_EUNATCH: FreeRtosBaseType = 42;
/// Invalid exchange (`pdFREERTOS_ERRNO_EBADE`). Value: 50.
pub const ERRNO_EBADE: FreeRtosBaseType = 50;
/// Inappropriate file type or format (`pdFREERTOS_ERRNO_EFTYPE`). Value: 79.
pub const ERRNO_EFTYPE: FreeRtosBaseType = 79;
/// No more files (`pdFREERTOS_ERRNO_ENMFILE`). Value: 89.
pub const ERRNO_ENMFILE: FreeRtosBaseType = 89;
/// Directory not empty (`pdFREERTOS_ERRNO_ENOTEMPTY`). Value: 90.
pub const ERRNO_ENOTEMPTY: FreeRtosBaseType = 90;
/// File or path name too long (`pdFREERTOS_ERRNO_ENAMETOOLONG`). Value: 91.
pub const ERRNO_ENAMETOOLONG: FreeRtosBaseType = 91;
/// Operation not supported on transport endpoint (`pdFREERTOS_ERRNO_EOPNOTSUPP`). Value: 95.
pub const ERRNO_EOPNOTSUPP: FreeRtosBaseType = 95;
/// Address family not supported by protocol (`pdFREERTOS_ERRNO_EAFNOSUPPORT`). Value: 97.
pub const ERRNO_EAFNOSUPPORT: FreeRtosBaseType = 97;
/// No buffer space available (`pdFREERTOS_ERRNO_ENOBUFS`). Value: 105.
pub const ERRNO_ENOBUFS: FreeRtosBaseType = 105;
/// Protocol not available (`pdFREERTOS_ERRNO_ENOPROTOOPT`). Value: 109.
pub const ERRNO_ENOPROTOOPT: FreeRtosBaseType = 109;
/// Address already in use (`pdFREERTOS_ERRNO_EADDRINUSE`). Value: 112.
pub const ERRNO_EADDRINUSE: FreeRtosBaseType = 112;
/// Connection timed out (`pdFREERTOS_ERRNO_ETIMEDOUT`). Value: 116.
pub const ERRNO_ETIMEDOUT: FreeRtosBaseType = 116;
/// Connection already in progress (`pdFREERTOS_ERRNO_EINPROGRESS`). Value: 119.
pub const ERRNO_EINPROGRESS: FreeRtosBaseType = 119;
/// Socket already connected (`pdFREERTOS_ERRNO_EALREADY`). Value: 120.
pub const ERRNO_EALREADY: FreeRtosBaseType = 120;
/// Address not available (`pdFREERTOS_ERRNO_EADDRNOTAVAIL`). Value: 125.
pub const ERRNO_EADDRNOTAVAIL: FreeRtosBaseType = 125;
/// Socket is already connected (`pdFREERTOS_ERRNO_EISCONN`). Value: 127.
pub const ERRNO_EISCONN: FreeRtosBaseType = 127;
/// Socket is not connected (`pdFREERTOS_ERRNO_ENOTCONN`). Value: 128.
pub const ERRNO_ENOTCONN: FreeRtosBaseType = 128;
/// No medium inserted (`pdFREERTOS_ERRNO_ENOMEDIUM`). Value: 135.
pub const ERRNO_ENOMEDIUM: FreeRtosBaseType = 135;
/// Invalid UTF-16 sequence (`pdFREERTOS_ERRNO_EILSEQ`). Value: 138.
pub const ERRNO_EILSEQ: FreeRtosBaseType = 138;
/// Operation canceled (`pdFREERTOS_ERRNO_ECANCELED`). Value: 140.
pub const ERRNO_ECANCELED: FreeRtosBaseType = 140;

//===========================================================================
// EVENT GROUP CONTROL BIT CONSTANTS (event_groups.h)
//===========================================================================

use crate::base::FreeRtosEventBits;

/// Clear events on exit bit (`eventCLEAR_EVENTS_ON_EXIT_BIT`).
pub const EVENT_CLEAR_EVENTS_ON_EXIT_BIT: FreeRtosEventBits = 0x0100_0000;

/// Unblocked due to bit set (`eventUNBLOCKED_DUE_TO_BIT_SET`).
pub const EVENT_UNBLOCKED_DUE_TO_BIT_SET: FreeRtosEventBits = 0x0200_0000;

/// Wait for all bits (`eventWAIT_FOR_ALL_BITS`).
pub const EVENT_WAIT_FOR_ALL_BITS: FreeRtosEventBits = 0x0400_0000;

/// Event bits control bytes mask (`eventEVENT_BITS_CONTROL_BYTES`).
pub const EVENT_EVENT_BITS_CONTROL_BYTES: FreeRtosEventBits = 0xFF00_0000;

//===========================================================================
// COMPILE-TIME UTILITY FUNCTIONS
//===========================================================================

/// Converts milliseconds to `FreeRTOS` ticks at compile time.
///
/// # Note
///
/// This uses a default tick period of 1 ms. The actual conversion depends
/// on `configTICK_RATE_HZ` in `FreeRTOSConfig.h`. Adjust this function
/// or use the FFI version [`freertos_rs_ms_to_ticks`] for runtime accuracy.
pub const fn ms_to_ticks(ms: FreeRtosTickType) -> FreeRtosTickType {
    ms
}

/// Converts `FreeRTOS` ticks to milliseconds at compile time.
pub const fn ticks_to_ms(ticks: FreeRtosTickType) -> FreeRtosTickType {
    ticks
}

/// Converts milliseconds to `FreeRTOS` ticks using the runtime `pdMS_TO_TICKS` macro.
///
/// This calls the C shim which uses the actual `configTICK_RATE_HZ` value.
pub fn ms_to_ticks_runtime(ms: FreeRtosTickType) -> FreeRtosTickType {
    unsafe { freertos_rs_ms_to_ticks(ms) }
}

/// Converts `FreeRTOS` ticks to milliseconds using the runtime tick period.
///
/// This calls the C shim which uses the actual `configTICK_PERIOD_MS` value.
pub fn ticks_to_ms_runtime(ticks: FreeRtosTickType) -> FreeRtosTickType {
    unsafe { freertos_rs_ticks_to_ms(ticks) }
}

/// Returns the tick period in milliseconds (`configTICK_PERIOD_MS`).
pub fn get_tick_period_ms() -> FreeRtosTickType {
    unsafe { freertos_rs_get_port_tick_period_ms() }
}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

// Scheduler state constants match FreeRTOS task.h:
// taskSCHEDULER_SUSPENDED = 0, taskSCHEDULER_NOT_STARTED = 1, taskSCHEDULER_RUNNING = 2
const _: () = assert!(TASK_SCHEDULER_SUSPENDED == 0);
const _: () = assert!(TASK_SCHEDULER_NOT_STARTED == 1);
const _: () = assert!(TASK_SCHEDULER_RUNNING == 2);

const _: () = assert!(pdTRUE == 1);
const _: () = assert!(pdFALSE == 0);
const _: () = assert!(pdPASS == 1);
const _: () = assert!(pdFAIL == 0);

const _: () = assert!(ms_to_ticks(100) == 100);
const _: () = assert!(ticks_to_ms(100) == 100);

const _: () = assert!(ERR_QUEUE_EMPTY == 0);
const _: () = assert!(ERR_QUEUE_FULL == 0);
const _: () = assert!(ERR_COULD_NOT_ALLOCATE_REQUIRED_MEMORY == -1);
const _: () = assert!(ERR_QUEUE_BLOCKED == -4);
const _: () = assert!(ERR_QUEUE_YIELD == -5);

const _: () = assert!(EVENT_CLEAR_EVENTS_ON_EXIT_BIT == 0x0100_0000);
const _: () = assert!(EVENT_UNBLOCKED_DUE_TO_BIT_SET == 0x0200_0000);
const _: () = assert!(EVENT_WAIT_FOR_ALL_BITS == 0x0400_0000);
const _: () = assert!(EVENT_EVENT_BITS_CONTROL_BYTES == 0xFF00_0000);

// Signed/unsigned boolean variants
const _: () = assert!(PD_FALSE_SIGNED == 0);
const _: () = assert!(PD_TRUE_SIGNED == 1);
const _: () = assert!(PD_FALSE_UNSIGNED == 0);
const _: () = assert!(PD_TRUE_UNSIGNED == 1);

// Endianness
const _: () = assert!(PD_FREERTOS_LITTLE_ENDIAN == 0);
const _: () = assert!(PD_FREERTOS_BIG_ENDIAN == 1);
const _: () = assert!(PD_LITTLE_ENDIAN == 0);
const _: () = assert!(PD_BIG_ENDIAN == 1);

// Errno constants (spot-check a few critical ones)
const _: () = assert!(ERRNO_NONE == 0);
const _: () = assert!(ERRNO_ENOENT == 2);
const _: () = assert!(ERRNO_EAGAIN == 11);
const _: () = assert!(ERRNO_EWOULDBLOCK == 11);
const _: () = assert!(ERRNO_ENOMEM == 12);
const _: () = assert!(ERRNO_EINVAL == 22);
const _: () = assert!(ERRNO_ETIMEDOUT == 116);
const _: () = assert!(ERRNO_ECANCELED == 140);
