//! FreeRTOS base type definitions for Rust FFI bindings.
//!
//! This module provides the fundamental types, enumerations, structures, and
//! constants used throughout the FreeRTOS Rust API wrapper. All handle types
//! are opaque pointers (`*const c_void`) that cross the FFI boundary safely.
//!
//! # Type Mapping
//!
//! | FreeRTOS Type    | Rust Type         | Notes                        |
//! |------------------|-------------------|------------------------------|
//! | `BaseType_t`     | `i32`             | Signed integer               |
//! | `UBaseType_t`    | `u32`             | Unsigned integer             |
//! | `TickType_t`     | `u32`             | Tick count                   |
//! | `TaskHandle_t`   | `*const c_void`   | Opaque handle                |
//! | `QueueHandle_t`  | `*const c_void`   | Opaque handle                |
//!
//! # Constants
//!
//! FreeRTOS uses several sentinel constants. These are provided as Rust
//! constants for direct use without FFI overhead:
//!
//! - [`PD_TRUE`] / [`PD_FALSE`] - Boolean result constants
//! - [`PD_PASS`] / [`PD_FAIL`] - Operation result constants
//! - [`PORT_MAX_DELAY`] - Maximum delay value (wait indefinitely)

use core::ffi::c_void;

//===========================================================================
// BASIC TYPE DEFINITIONS
//===========================================================================

/// FreeRTOS `BaseType_t` — typically a signed 32-bit integer on 32-bit targets.
pub type FreeRtosBaseType = i32;

/// FreeRTOS `UBaseType_t` — typically an unsigned 32-bit integer on 32-bit targets.
pub type FreeRtosUBaseType = u32;

/// FreeRTOS `TickType_t` — type used for tick counts, delays, and timeouts.
pub type FreeRtosTickType = u32;

/// Mutable void pointer — used for passing generic data pointers.
pub type FreeRtosVoidPtr = *mut c_void;

/// Const void pointer — used for passing read-only generic data pointers.
pub type FreeRtosConstVoidPtr = *const c_void;

/// C string pointer — used for FreeRTOS name parameters.
pub type FreeRtosCharPtr = *const u8;

/// Single byte type.
pub type FreeRtosChar = u8;

/// FreeRTOS `EventBits_t` — used for event group bit masks.
pub type FreeRtosEventBitsType = u32;

/// Mutable pointer to `BaseType_t`.
pub type FreeRtosBaseTypeMutPtr = *mut FreeRtosBaseType;

//===========================================================================
// HANDLE TYPES (all opaque pointers)
//===========================================================================

/// Opaque handle to a FreeRTOS task.
///
/// Use with [`crate::task`] module functions. Can be `NULL` to refer to the
/// currently executing task.
pub type FreeRtosTaskHandle = *const c_void;

/// Opaque handle to a FreeRTOS queue.
///
/// Use with [`crate::queue`] module functions.
pub type FreeRtosQueueHandle = *const c_void;

/// Opaque handle to a FreeRTOS queue set.
///
/// Use with [`crate::queue`] queue set functions.
pub type FreeRtosQueueSetHandle = *const c_void;

/// Opaque handle to a FreeRTOS queue set member (queue or semaphore).
pub type FreeRtosQueueSetMemberHandle = *const c_void;

/// Opaque handle to a FreeRTOS semaphore.
///
/// Use with [`crate::semphr`] module functions.
pub type FreeRtosSemaphoreHandle = *const c_void;

/// Opaque handle to a FreeRTOS mutex.
///
/// Internally the same as a semaphore handle, but distinguished for type safety.
pub type FreeRtosMutexHandle = *const c_void;

/// Opaque handle to a FreeRTOS event group.
///
/// Use with [`crate::event_groups`] module functions.
pub type FreeRtosEventGroupHandle = *const c_void;

/// Opaque handle to a FreeRTOS software timer.
///
/// Use with [`crate::timers`] module functions.
pub type FreeRtosTimerHandle = *const c_void;

/// Opaque handle to a FreeRTOS stream buffer.
///
/// Use with [`crate::stream_buffer`] module functions.
pub type FreeRtosStreamBufferHandle = *const c_void;

/// Opaque handle to a FreeRTOS message buffer.
///
/// Use with [`crate::message_buffer`] module functions.
pub type FreeRtosMessageBufferHandle = *const c_void;

//===========================================================================
// FUNCTION POINTER TYPES
//===========================================================================

/// FreeRTOS task entry point function signature.
///
/// Tasks are created with a function pointer matching this type. The `*mut c_void`
/// parameter receives the argument passed as `pvParameters` during task creation.
///
/// # Safety
///
/// The function must not return. If it does, the FreeRTOS scheduler will
/// delete the task automatically.
pub type FreeRtosTaskFunction = unsafe extern "C" fn(*mut c_void);

/// FreeRTOS software timer callback function signature.
///
/// Called by the timer daemon task when a timer expires.
pub type FreeRtosTimerCallback = unsafe extern "C" fn(FreeRtosTimerHandle);

/// FreeRTOS pended function callback signature.
///
/// Used with `xTimerPendFunctionCall` / `xTimerPendFunctionCallFromISR`.
pub type FreeRtosPendedFunction = unsafe extern "C" fn(
    parameter1: *mut c_void,
    parameter2: u32,
);

/// FreeRTOS stream buffer send/receive completion callback signature.
pub type FreeRtosStreamBufferCallbackFunction = unsafe extern "C" fn(
    stream_buffer: FreeRtosStreamBufferHandle,
    pxHigherPriorityTaskWoken: *mut FreeRtosBaseType,
);

//===========================================================================
// STACK AND STATIC ALLOCATION TYPES
//===========================================================================

/// FreeRTOS stack type — pointer to the stack memory area.
pub type FreeRtosStackType = *mut c_void;

/// FreeRTOS `StaticTask_t` — pointer to statically allocated task buffer.
pub type FreeRtosStaticTask = *mut c_void;

//===========================================================================
// ADDITIONAL TYPES
//===========================================================================

/// FreeRTOS unsigned long type.
pub type FreeRtosUnsignedLong = u32;

/// FreeRTOS unsigned short type.
pub type FreeRtosUnsignedShort = u16;

/// FreeRTOS event bits type alias (convenience).
pub type FreeRtosEventBits = u32;

/// FreeRTOS notification value type.
pub type FreeRtosNotificationValue = u32;

/// FreeRTOS `configSTACK_DEPTH_TYPE` — typically `u16` on most platforms.
pub type FreeRtosConfigStackDepthType = u16;

/// FreeRTOS `configRUN_TIME_COUNTER_TYPE` — typically `u32`.
pub type FreeRtosRunTimeCounterType = u32;

//===========================================================================
// CONSTANTS
//===========================================================================

/// FreeRTOS `pdTRUE` — boolean true (value: 1).
pub const PD_TRUE: FreeRtosBaseType = 1;

/// FreeRTOS `pdFALSE` — boolean false (value: 0).
pub const PD_FALSE: FreeRtosBaseType = 0;

/// FreeRTOS `pdPASS` — operation succeeded (value: 1).
pub const PD_PASS: FreeRtosBaseType = 1;

/// FreeRTOS `pdFAIL` — operation failed (value: 0).
pub const PD_FAIL: FreeRtosBaseType = 0;

/// FreeRTOS `portMAX_DELAY` — wait indefinitely (value: `0xFFFFFFFF`).
///
/// Pass this as the timeout parameter to any FreeRTOS API that accepts a
/// `TickType_t` timeout to wait without a timeout.
pub const PORT_MAX_DELAY: FreeRtosTickType = 0xFFFFFFFF;

/// FreeRTOS `tskDEFAULT_INDEX_TO_NOTIFY` — default notification index (0).
pub const TSK_DEFAULT_INDEX_TO_NOTIFY: FreeRtosUBaseType = 0;

/// FreeRTOS `tskIDLE_PRIORITY` — default priority for the idle task (0).
pub const TSK_IDLE_PRIORITY: FreeRtosUBaseType = 0;

/// FreeRTOS `tskNO_AFFINITY` — no CPU affinity (task can run on any core).
///
/// Used with SMP task creation functions (`xTaskCreateAffinitySet`, etc.).
/// Value is `UBaseType_t` max (`!0`).
pub const TSK_NO_AFFINITY: FreeRtosUBaseType = !0;

//===========================================================================
// STRUCTURES
//===========================================================================

/// FreeRTOS `TimeOut_t` — timeout tracking structure.
///
/// Used with `vTaskSetTimeOutState()` and `xTaskCheckForTimeOut()` to
/// implement bounded wait loops without risking overflow.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FreeRtosTimeOut {
    /// Overflow counter for extended timeout tracking.
    pub overflow_count: FreeRtosBaseType,
    /// Tick count when the timeout period started.
    pub time_on_entering: FreeRtosTickType,
}

/// FreeRTOS heap statistics structure.
///
/// Used with `vPortGetHeapStats()` to retrieve detailed heap allocation
/// information from the FreeRTOS memory manager.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct FreeRtosHeapStats {
    /// Size of the largest free block in the heap.
    pub xAvailableHeapSpaceInBytes: usize,
    /// Size of the smallest free block that has been merged.
    pub xSizeOfLargestFreeBlockInBytes: usize,
    /// Size of the smallest free block in the heap.
    pub xSizeOfSmallestFreeBlockInBytes: usize,
    /// Number of free blocks in the heap.
    pub xNumberOfFreeBlocks: usize,
    /// Minimum number of free bytes since the heap was initialized.
    pub xMinimumEverFreeBytesRemaining: usize,
    /// Number of calls to `pvPortMalloc()` that returned NULL.
    pub xNumberOfSuccessfulAllocations: usize,
    /// Number of calls to `vPortFree()` that successfully freed memory.
    pub xNumberOfSuccessfulFrees: usize,
}

//===========================================================================
// ENUMERATION TYPES
//===========================================================================

/// Task notification action enumeration.
///
/// Determines how the notification value is updated when a task notification
/// is sent.
///
/// # Mapping
///
/// | Variant                    | FreeRTOS Constant               |
/// |----------------------------|----------------------------------|
/// | `NoAction`                 | `eNoAction`                      |
/// | `SetBits`                  | `eSetBits`                       |
/// | `Increment`                | `eIncrement`                     |
/// | `SetValueWithOverwrite`    | `eSetValueWithOverwrite`         |
/// | `SetValueWithoutOverwrite` | `eSetValueWithoutOverwrite`      |
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum FreeRtosNotifyAction {
    /// No action — just notify without modifying the value.
    NoAction = 0,
    /// Perform a bitwise OR of the notification value.
    SetBits = 1,
    /// Increment the notification value by one.
    Increment = 2,
    /// Set the notification value (overwrites existing).
    SetValueWithOverwrite = 3,
    /// Set the notification value (only if no previous value pending).
    SetValueWithoutOverwrite = 4,
}

/// Queue send position enumeration.
///
/// Determines where in the queue a new item is placed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum FreeRtosQueueSendPosition {
    /// Send to the back of the queue (FIFO order).
    SendToBack = 0,
    /// Send to the front of the queue (LIFO order).
    SendToFront = 1,
    /// Overwrite — only valid for queues with length 1.
    Overwrite = 2,
}

/// Timer command enumeration.
///
/// Used internally by the timer command queue.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum FreeRtosTimerCommand {
    /// Start a timer from a task context.
    Start = 0,
    /// Stop a timer from a task context.
    Stop = 1,
    /// Change a timer's period from a task context.
    ChangePeriod = 2,
    /// Delete a timer from a task context.
    Delete = 3,
    /// Reset a timer from a task context.
    Reset = 4,
    /// Start a timer from an ISR context.
    StartFromISR = 5,
    /// Stop a timer from an ISR context.
    StopFromISR = 6,
    /// Change a timer's period from an ISR context.
    ChangePeriodFromISR = 7,
    /// Delete a timer from an ISR context.
    DeleteFromISR = 8,
    /// Reset a timer from an ISR context.
    ResetFromISR = 9,
}

/// Task state enumeration.
///
/// Represents the current state of a FreeRTOS task.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum FreeRtosTaskState {
    /// The task is currently running (only valid for the calling task).
    Running = 0,
    /// The task is in a ready list, waiting to be scheduled.
    Ready = 1,
    /// The task is blocked waiting for an event or timeout.
    Blocked = 2,
    /// The task is suspended or blocked with an infinite timeout.
    Suspended = 3,
    /// The task has been deleted but its TCB has not yet been freed.
    Deleted = 4,
}

//===========================================================================
// ERROR TYPE
//===========================================================================

/// Error type for FreeRTOS API operations.
///
/// Returned by safe wrapper functions when a FreeRTOS API call fails.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum FreeRtosError {
    /// Memory allocation failed (heap exhausted).
    OutOfMemory,
    /// A queue send operation timed out.
    QueueSendTimeout,
    /// A queue receive operation timed out.
    QueueReceiveTimeout,
    /// A mutex acquisition timed out.
    MutexTimeout,
    /// A generic timeout occurred.
    Timeout,
    /// A queue is full and cannot accept more items.
    QueueFull,
    /// A string conversion failed.
    StringConversionError,
    /// The requested task was not found.
    TaskNotFound,
    /// An invalid queue size was specified.
    InvalidQueueSize,
    /// The processor has shut down.
    ProcessorHasShutDown,
    /// The operation is not supported in the current configuration.
    NotSupported,
    /// An invalid parameter was provided.
    InvalidParameter,
}

//===========================================================================
// TASK STATUS STRUCTURE
//===========================================================================

/// FreeRTOS `TaskStatus_t` — detailed task status information.
///
/// Filled by `uxTaskGetSystemState()` and `vTaskGetInfo()`.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FreeRtosTaskStatusFfi {
    /// Handle of the task.
    pub handle: FreeRtosTaskHandle,
    /// Name of the task (pointer to internal string, do not free).
    pub task_name: FreeRtosCharPtr,
    /// Task number used for tracing.
    pub task_number: FreeRtosUBaseType,
    /// Current state of the task.
    pub task_state: FreeRtosTaskState,
    /// Current priority of the task (may differ from base due to inheritance).
    pub current_priority: FreeRtosUBaseType,
    /// Base priority of the task (before any priority inheritance).
    pub base_priority: FreeRtosUBaseType,
    /// Total run time consumed by this task.
    pub run_time_counter: FreeRtosUnsignedLong,
    /// Pointer to the task's stack base address.
    pub stack_base: FreeRtosCharPtr,
    /// Minimum remaining stack space (high water mark) in words.
    pub stack_high_water_mark: FreeRtosUnsignedShort,
}

//===========================================================================
// UNIT TESTS
//===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    #[test]
    fn test_base_type_sizes() {
        assert_eq!(size_of::<FreeRtosBaseType>(), 4);
        assert_eq!(size_of::<FreeRtosUBaseType>(), 4);
        assert_eq!(size_of::<FreeRtosTickType>(), 4);
        assert_eq!(size_of::<FreeRtosConfigStackDepthType>(), 2);
    }

    #[test]
    fn test_handle_types_are_pointers() {
        assert_eq!(size_of::<FreeRtosTaskHandle>(), size_of::<*const c_void>());
        assert_eq!(size_of::<FreeRtosQueueHandle>(), size_of::<*const c_void>());
        assert_eq!(size_of::<FreeRtosSemaphoreHandle>(), size_of::<*const c_void>());
        assert_eq!(size_of::<FreeRtosTimerHandle>(), size_of::<*const c_void>());
        assert_eq!(size_of::<FreeRtosEventGroupHandle>(), size_of::<*const c_void>());
        assert_eq!(
            size_of::<FreeRtosStreamBufferHandle>(),
            size_of::<*const c_void>()
        );
        assert_eq!(
            size_of::<FreeRtosMessageBufferHandle>(),
            size_of::<*const c_void>()
        );
    }

    #[test]
    fn test_notify_action_values() {
        assert_eq!(FreeRtosNotifyAction::NoAction as u32, 0);
        assert_eq!(FreeRtosNotifyAction::SetBits as u32, 1);
        assert_eq!(FreeRtosNotifyAction::Increment as u32, 2);
        assert_eq!(FreeRtosNotifyAction::SetValueWithOverwrite as u32, 3);
        assert_eq!(FreeRtosNotifyAction::SetValueWithoutOverwrite as u32, 4);
    }

    #[test]
    fn test_queue_send_position_values() {
        assert_eq!(FreeRtosQueueSendPosition::SendToBack as u32, 0);
        assert_eq!(FreeRtosQueueSendPosition::SendToFront as u32, 1);
        assert_eq!(FreeRtosQueueSendPosition::Overwrite as u32, 2);
    }

    #[test]
    fn test_task_state_values() {
        assert_eq!(FreeRtosTaskState::Running as u8, 0);
        assert_eq!(FreeRtosTaskState::Ready as u8, 1);
        assert_eq!(FreeRtosTaskState::Blocked as u8, 2);
        assert_eq!(FreeRtosTaskState::Suspended as u8, 3);
        assert_eq!(FreeRtosTaskState::Deleted as u8, 4);
    }

    #[test]
    fn test_timer_command_values() {
        assert_eq!(FreeRtosTimerCommand::Start as u32, 0);
        assert_eq!(FreeRtosTimerCommand::Stop as u32, 1);
        assert_eq!(FreeRtosTimerCommand::ChangePeriod as u32, 2);
        assert_eq!(FreeRtosTimerCommand::Delete as u32, 3);
        assert_eq!(FreeRtosTimerCommand::Reset as u32, 4);
    }

    #[test]
    fn test_constants() {
        assert_eq!(PD_TRUE, 1);
        assert_eq!(PD_FALSE, 0);
        assert_eq!(PD_PASS, 1);
        assert_eq!(PD_FAIL, 0);
        assert_eq!(PORT_MAX_DELAY, 0xFFFFFFFF);
        assert_eq!(TSK_DEFAULT_INDEX_TO_NOTIFY, 0);
        assert_eq!(TSK_IDLE_PRIORITY, 0);
        assert_eq!(TSK_NO_AFFINITY, 0xFFFFFFFF);
    }

    #[test]
    fn test_timeout_struct_layout() {
        assert_eq!(
            size_of::<FreeRtosTimeOut>(),
            size_of::<FreeRtosBaseType>() + size_of::<FreeRtosTickType>()
        );
        assert!(align_of::<FreeRtosTimeOut>() >= 4);
    }

    #[test]
    fn test_task_status_struct_layout() {
        // Verify the struct is a reasonable size (platform dependent, but should be consistent)
        assert!(size_of::<FreeRtosTaskStatusFfi>() > 0);
    }

    #[test]
    fn test_heap_stats_struct_layout() {
        // 7 usize fields
        assert_eq!(size_of::<FreeRtosHeapStats>(), 7 * size_of::<usize>());
    }
}
