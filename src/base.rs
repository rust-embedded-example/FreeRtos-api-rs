//! `FreeRTOS` base type definitions for Rust FFI bindings.
//!
//! This module provides the fundamental types, enumerations, structures, and
//! constants used throughout the `FreeRTOS` Rust API wrapper. All handle types
//! are opaque pointers (`*const c_void`) that cross the FFI boundary safely.
//!
//! # Type Mapping
//!
//! | `FreeRTOS` Type    | Rust Type         | Notes                        |
//! |------------------|-------------------|------------------------------|
//! | `BaseType_t`     | `i32`             | Signed integer               |
//! | `UBaseType_t`    | `u32`             | Unsigned integer             |
//! | `TickType_t`     | `u32`             | Tick count                   |
//! | `TaskHandle_t`   | `*const c_void`   | Opaque handle                |
//! | `QueueHandle_t`  | `*const c_void`   | Opaque handle                |
//!
//! # Constants
//!
//! `FreeRTOS` uses several sentinel constants. These are provided as Rust
//! constants for direct use without FFI overhead:
//!
//! - [`PD_TRUE`] / [`PD_FALSE`] - Boolean result constants
//! - [`PD_PASS`] / [`PD_FAIL`] - Operation result constants
//! - [`PORT_MAX_DELAY`] - Maximum delay value (wait indefinitely)

use core::ffi::c_void;

//===========================================================================
// BASIC TYPE DEFINITIONS
//===========================================================================

/// `FreeRTOS` `BaseType_t` — typically a signed 32-bit integer on 32-bit targets.
pub type FreeRtosBaseType = i32;

/// `FreeRTOS` `UBaseType_t` — typically an unsigned 32-bit integer on 32-bit targets.
pub type FreeRtosUBaseType = u32;

/// `FreeRTOS` `TickType_t` — type used for tick counts, delays, and timeouts.
pub type FreeRtosTickType = u32;

/// Mutable void pointer — used for passing generic data pointers.
pub type FreeRtosVoidPtr = *mut c_void;

/// Const void pointer — used for passing read-only generic data pointers.
pub type FreeRtosConstVoidPtr = *const c_void;

/// C string pointer — used for `FreeRTOS` name parameters.
pub type FreeRtosCharPtr = *const u8;

/// Single byte type.
pub type FreeRtosChar = u8;

/// `FreeRTOS` `EventBits_t` — used for event group bit masks.
pub type FreeRtosEventBitsType = u32;

/// Mutable pointer to `BaseType_t`.
pub type FreeRtosBaseTypeMutPtr = *mut FreeRtosBaseType;

//===========================================================================
// HANDLE TYPES (all opaque pointers)
//===========================================================================

/// Opaque handle to a `FreeRTOS` task.
///
/// Use with [`crate::task`] module functions. Can be `NULL` to refer to the
/// currently executing task.
pub type FreeRtosTaskHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` queue.
///
/// Use with [`crate::queue`] module functions.
pub type FreeRtosQueueHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` queue set.
///
/// Use with [`crate::queue`] queue set functions.
pub type FreeRtosQueueSetHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` queue set member (queue or semaphore).
pub type FreeRtosQueueSetMemberHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` semaphore.
///
/// Use with [`crate::semphr`] module functions.
pub type FreeRtosSemaphoreHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` mutex.
///
/// Internally the same as a semaphore handle, but distinguished for type safety.
pub type FreeRtosMutexHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` event group.
///
/// Use with [`crate::event_groups`] module functions.
pub type FreeRtosEventGroupHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` software timer.
///
/// Use with [`crate::timers`] module functions.
pub type FreeRtosTimerHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` stream buffer.
///
/// Use with [`crate::stream_buffer`] module functions.
pub type FreeRtosStreamBufferHandle = *const c_void;

/// Opaque handle to a `FreeRTOS` message buffer.
///
/// Use with [`crate::message_buffer`] module functions.
pub type FreeRtosMessageBufferHandle = *const c_void;

//===========================================================================
// FUNCTION POINTER TYPES
//===========================================================================

/// `FreeRTOS` task entry point function signature.
///
/// Tasks are created with a function pointer matching this type. The `*mut c_void`
/// parameter receives the argument passed as `pvParameters` during task creation.
///
/// # Safety
///
/// The function must not return. If it does, the `FreeRTOS` scheduler will
/// delete the task automatically.
pub type FreeRtosTaskFunction = unsafe extern "C" fn(*mut c_void);

/// `FreeRTOS` software timer callback function signature.
///
/// Called by the timer daemon task when a timer expires.
pub type FreeRtosTimerCallback = unsafe extern "C" fn(FreeRtosTimerHandle);

/// `FreeRTOS` pended function callback signature.
///
/// Used with `xTimerPendFunctionCall` / `xTimerPendFunctionCallFromISR`.
pub type FreeRtosPendedFunction = unsafe extern "C" fn(
    parameter1: *mut c_void,
    parameter2: u32,
);

/// `FreeRTOS` stream buffer send/receive completion callback signature.
pub type FreeRtosStreamBufferCallbackFunction = unsafe extern "C" fn(
    stream_buffer: FreeRtosStreamBufferHandle,
    pxHigherPriorityTaskWoken: *mut FreeRtosBaseType,
);

//===========================================================================
// STACK AND STATIC ALLOCATION TYPES
//===========================================================================

/// `FreeRTOS` stack type — pointer to the stack memory area.
pub type FreeRtosStackType = *mut c_void;

/// `FreeRTOS` `StaticTask_t` — pointer to statically allocated task buffer.
pub type FreeRtosStaticTask = *mut c_void;

//===========================================================================
// ADDITIONAL TYPES
//===========================================================================

/// `FreeRTOS` unsigned long type.
pub type FreeRtosUnsignedLong = u32;

/// `FreeRTOS` unsigned short type.
pub type FreeRtosUnsignedShort = u16;

/// `FreeRTOS` event bits type alias (convenience).
pub type FreeRtosEventBits = u32;

/// `FreeRTOS` notification value type.
pub type FreeRtosNotificationValue = u32;

/// `FreeRTOS` `configSTACK_DEPTH_TYPE` — typically `u16` on most platforms.
pub type FreeRtosConfigStackDepthType = u16;

/// `FreeRTOS` `configRUN_TIME_COUNTER_TYPE` — typically `u32`.
pub type FreeRtosRunTimeCounterType = u32;

//===========================================================================
// CONSTANTS
//===========================================================================

/// `FreeRTOS` `pdTRUE` — boolean true (value: 1).
pub const PD_TRUE: FreeRtosBaseType = 1;

/// `FreeRTOS` `pdFALSE` — boolean false (value: 0).
pub const PD_FALSE: FreeRtosBaseType = 0;

/// `FreeRTOS` `pdPASS` — operation succeeded (value: 1).
pub const PD_PASS: FreeRtosBaseType = 1;

/// `FreeRTOS` `pdFAIL` — operation failed (value: 0).
pub const PD_FAIL: FreeRtosBaseType = 0;

/// `FreeRTOS` `portMAX_DELAY` — wait indefinitely (value: `0xFFFFFFFF`).
///
/// Pass this as the timeout parameter to any `FreeRTOS` API that accepts a
/// `TickType_t` timeout to wait without a timeout.
pub const PORT_MAX_DELAY: FreeRtosTickType = 0xFFFF_FFFF;

/// `FreeRTOS` `tskDEFAULT_INDEX_TO_NOTIFY` — default notification index (0).
pub const TSK_DEFAULT_INDEX_TO_NOTIFY: FreeRtosUBaseType = 0;

/// `FreeRTOS` `tskIDLE_PRIORITY` — default priority for the idle task (0).
pub const TSK_IDLE_PRIORITY: FreeRtosUBaseType = 0;

/// `FreeRTOS` `tskNO_AFFINITY` — no CPU affinity (task can run on any core).
///
/// Used with SMP task creation functions (`xTaskCreateAffinitySet`, etc.).
/// Value is `UBaseType_t` max (`!0`).
pub const TSK_NO_AFFINITY: FreeRtosUBaseType = !0;

//===========================================================================
// STRUCTURES
//===========================================================================

/// `FreeRTOS` `TimeOut_t` — timeout tracking structure.
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

/// `FreeRTOS` heap statistics structure.
///
/// Used with `vPortGetHeapStats()` to retrieve detailed heap allocation
/// information from the `FreeRTOS` memory manager.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct FreeRtosHeapStats {
    /// Total heap size currently available (sum of all free blocks).
    pub xAvailableHeapSpaceInBytes: usize,
    /// Maximum size in bytes of all free blocks in the heap.
    pub xSizeOfLargestFreeBlockInBytes: usize,
    /// Minimum size in bytes of all free blocks in the heap.
    pub xSizeOfSmallestFreeBlockInBytes: usize,
    /// Number of free memory blocks in the heap.
    pub xNumberOfFreeBlocks: usize,
    /// Minimum amount of total free memory since boot.
    pub xMinimumEverFreeBytesRemaining: usize,
    /// Number of calls to `pvPortMalloc()` that returned a valid block.
    pub xNumberOfSuccessfulAllocations: usize,
    /// Number of calls to `vPortFree()` that successfully freed memory.
    pub xNumberOfSuccessfulFrees: usize,
}

//===========================================================================
// MPU MEMORY REGION AND TASK PARAMETERS STRUCTURES
//===========================================================================

/// Default number of configurable MPU regions per task.
///
/// Maps to `portNUM_CONFIGURABLE_REGIONS` from `portable.h`. The default is 1
/// but port implementations may override it.
pub const PORT_NUM_CONFIGURABLE_REGIONS: usize = 1;

/// MPU region access permission flags.
///
/// Maps to `tskMPU_REGION_*` constants from `task.h`.
pub const TSK_MPU_REGION_READ_ONLY: u32 = 1 << 0;
pub const TSK_MPU_REGION_READ_WRITE: u32 = 1 << 1;
pub const TSK_MPU_REGION_EXECUTE_NEVER: u32 = 1 << 2;
pub const TSK_MPU_REGION_NORMAL_MEMORY: u32 = 1 << 3;
pub const TSK_MPU_REGION_DEVICE_MEMORY: u32 = 1 << 4;
pub const TSK_MPU_REGION_NON_SHAREABLE: u32 = 1 << 6;
pub const TSK_MPU_REGION_OUTER_SHAREABLE: u32 = 1 << 7;
pub const TSK_MPU_REGION_INNER_SHAREABLE: u32 = 1 << 8;

/// `FreeRTOS` MPU memory region definition.
///
/// Maps to `MemoryRegion_t` from `task.h`. Each region specifies a base
/// address, length, and access permission parameters.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct FreeRtosMemoryRegion {
    /// Base address of the memory region.
    pub base_address: FreeRtosVoidPtr,
    /// Length of the region in bytes.
    pub length_in_bytes: u32,
    /// Access permission flags (`TSK_MPU_REGION_*`).
    pub parameters: u32,
}

/// `FreeRTOS` MPU task parameters.
///
/// Maps to `TaskParameters_t` from `task.h`. Used when creating MPU-restricted
/// tasks via `xTaskCreateRestricted()` and related functions.
///
/// # Layout Note
///
/// In the C `TaskParameters_t`, the `task_buffer` field (`pxTaskBuffer`) is
/// conditionally compiled and only present when **both**:
/// - `portUSING_MPU_WRAPPERS == 1`
/// - `configSUPPORT_STATIC_ALLOCATION == 1`
///
/// This Rust struct always includes `task_buffer`. Since MPU-restricted task
/// creation functions (`xTaskCreateRestricted*`) require `portUSING_MPU_WRAPPERS == 1`,
/// this struct should only be used in MPU-enabled configurations where the
/// field is present. **Do not pass this struct to FFI if your FreeRTOS config
/// does not enable both MPU wrappers and static allocation.**
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FreeRtosTaskParameters {
    /// Task entry function.
    pub task_code: FreeRtosTaskFunction,
    /// Null-terminated task name string.
    pub name: FreeRtosCharPtr,
    /// Stack depth in words (not bytes).
    pub stack_depth: FreeRtosConfigStackDepthType,
    /// Parameter passed to the task function.
    pub parameters: FreeRtosVoidPtr,
    /// Task priority (may include `portPRIVILEGE_BIT`).
    pub priority: FreeRtosUBaseType,
    /// Stack buffer pointer, or null for dynamic allocation.
    pub stack_buffer: FreeRtosStackType,
    /// MPU region definitions.
    pub regions: [FreeRtosMemoryRegion; PORT_NUM_CONFIGURABLE_REGIONS],
    /// Static task buffer (`StaticTask_t*`), or null for dynamic allocation.
    pub task_buffer: FreeRtosStaticTask,
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
#[repr(i32)]
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
#[repr(i32)]
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
/// Used internally by the timer command queue. Values match the `tmrCOMMAND_*`
/// macros defined in `timers.h`.
///
/// | Variant                      | FreeRTOS Constant                       | Value |
/// |------------------------------|-----------------------------------------|-------|
/// | `ExecuteCallbackFromISR`     | `tmrCOMMAND_EXECUTE_CALLBACK_FROM_ISR`  | -2    |
/// | `ExecuteCallback`            | `tmrCOMMAND_EXECUTE_CALLBACK`           | -1    |
/// | `StartDontTrace`             | `tmrCOMMAND_START_DONT_TRACE`           | 0     |
/// | `Start`                      | `tmrCOMMAND_START`                      | 1     |
/// | `Reset`                      | `tmrCOMMAND_RESET`                      | 2     |
/// | `Stop`                       | `tmrCOMMAND_STOP`                       | 3     |
/// | `ChangePeriod`               | `tmrCOMMAND_CHANGE_PERIOD`              | 4     |
/// | `Delete`                     | `tmrCOMMAND_DELETE`                     | 5     |
/// | `StartFromISR`               | `tmrCOMMAND_START_FROM_ISR`             | 6     |
/// | `ResetFromISR`               | `tmrCOMMAND_RESET_FROM_ISR`             | 7     |
/// | `StopFromISR`                | `tmrCOMMAND_STOP_FROM_ISR`              | 8     |
/// | `ChangePeriodFromISR`        | `tmrCOMMAND_CHANGE_PERIOD_FROM_ISR`     | 9     |
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum FreeRtosTimerCommand {
    /// Execute a callback from an ISR context (`tmrCOMMAND_EXECUTE_CALLBACK_FROM_ISR`).
    ExecuteCallbackFromISR = -2,
    /// Execute a callback from a task context (`tmrCOMMAND_EXECUTE_CALLBACK`).
    ExecuteCallback = -1,
    /// Start without trace (`tmrCOMMAND_START_DONT_TRACE`).
    StartDontTrace = 0,
    /// Start a timer from a task context (`tmrCOMMAND_START`).
    Start = 1,
    /// Reset a timer from a task context (`tmrCOMMAND_RESET`).
    Reset = 2,
    /// Stop a timer from a task context (`tmrCOMMAND_STOP`).
    Stop = 3,
    /// Change a timer's period from a task context (`tmrCOMMAND_CHANGE_PERIOD`).
    ChangePeriod = 4,
    /// Delete a timer from a task context (`tmrCOMMAND_DELETE`).
    Delete = 5,
    /// First ISR command / Start a timer from an ISR context (`tmrCOMMAND_START_FROM_ISR`).
    StartFromISR = 6,
    /// Reset a timer from an ISR context (`tmrCOMMAND_RESET_FROM_ISR`).
    ResetFromISR = 7,
    /// Stop a timer from an ISR context (`tmrCOMMAND_STOP_FROM_ISR`).
    StopFromISR = 8,
    /// Change a timer's period from an ISR context (`tmrCOMMAND_CHANGE_PERIOD_FROM_ISR`).
    ChangePeriodFromISR = 9,
}

/// Task state enumeration.
///
/// Represents the current state of a `FreeRTOS` task. Uses `repr(u32)` to
/// match the 4-byte size of C `enum eTaskState` on 32-bit targets.
///
/// # Mapping
///
/// | Variant     | FreeRTOS Constant | Value |
/// |-------------|-------------------|-------|
/// | `Running`   | `eRunning`        | 0     |
/// | `Ready`     | `eReady`          | 1     |
/// | `Blocked`   | `eBlocked`        | 2     |
/// | `Suspended` | `eSuspended`      | 3     |
/// | `Deleted`   | `eDeleted`        | 4     |
/// | `Invalid`   | `eInvalid`        | 5     |
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
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
    /// Invalid/not a real task handle (`eInvalid`).
    Invalid = 5,
}

//===========================================================================
// ERROR TYPE
//===========================================================================

/// Error type for `FreeRTOS` API operations.
///
/// Returned by safe wrapper functions when a `FreeRTOS` API call fails.
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

/// `FreeRTOS` `TaskStatus_t` — detailed task status information.
///
/// Filled by `uxTaskGetSystemState()` and `vTaskGetInfo()`.
///
/// **Layout matches C `TaskStatus_t` on 32-bit targets** (repr(C)):
///
/// ```text
/// offset  size  field
/// 0x00    4     handle         (TaskHandle_t = pointer)
/// 0x04    4     task_name      (const char *)
/// 0x08    4     task_number    (UBaseType_t)
/// 0x0C    4     task_state     (eTaskState = enum int)
/// 0x10    4     current_priority (UBaseType_t)
/// 0x14    4     base_priority  (UBaseType_t)
/// 0x18    4     run_time_counter (configRUN_TIME_COUNTER_TYPE)
/// 0x1C    4     stack_base     (StackType_t *)
/// 0x20    2     stack_high_water_mark (configSTACK_DEPTH_TYPE)
/// 0x22    2     (padding)
/// ```
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
    pub run_time_counter: FreeRtosRunTimeCounterType,
    /// Pointer to the task's stack base address.
    pub stack_base: FreeRtosCharPtr,
    /// Minimum remaining stack space (high water mark) in words.
    pub stack_high_water_mark: FreeRtosConfigStackDepthType,
}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

// Base type sizes
const _: () = assert!(core::mem::size_of::<FreeRtosBaseType>() == 4);
const _: () = assert!(core::mem::size_of::<FreeRtosUBaseType>() == 4);
const _: () = assert!(core::mem::size_of::<FreeRtosTickType>() == 4);
const _: () = assert!(core::mem::size_of::<FreeRtosConfigStackDepthType>() == 2);

// Struct layout verification (ARM Cortex-M7, 32-bit pointers)
// TaskStatus_t: handle(4) + name(4) + number(4) + state(4) + curr_pri(4) + base_pri(4) + runtime(4) + stack_base(4) + hwm(2) + pad(2) = 36
const _: () = assert!(core::mem::size_of::<FreeRtosTaskStatusFfi>() == 36);
// TimeOut_t: overflow_count(4) + time_on_entering(4) = 8
const _: () = assert!(core::mem::size_of::<FreeRtosTimeOut>() == 8);

// Handle types are pointer-sized
const _: () = assert!(core::mem::size_of::<FreeRtosTaskHandle>() == core::mem::size_of::<*const c_void>());
const _: () = assert!(core::mem::size_of::<FreeRtosQueueHandle>() == core::mem::size_of::<*const c_void>());
const _: () = assert!(core::mem::size_of::<FreeRtosSemaphoreHandle>() == core::mem::size_of::<*const c_void>());
const _: () = assert!(core::mem::size_of::<FreeRtosTimerHandle>() == core::mem::size_of::<*const c_void>());
const _: () = assert!(core::mem::size_of::<FreeRtosEventGroupHandle>() == core::mem::size_of::<*const c_void>());
const _: () = assert!(core::mem::size_of::<FreeRtosStreamBufferHandle>() == core::mem::size_of::<*const c_void>());
const _: () = assert!(core::mem::size_of::<FreeRtosMessageBufferHandle>() == core::mem::size_of::<*const c_void>());

// Enum discriminant values
const _: () = assert!(FreeRtosNotifyAction::NoAction as u32 == 0);
const _: () = assert!(FreeRtosNotifyAction::SetBits as u32 == 1);
const _: () = assert!(FreeRtosNotifyAction::Increment as u32 == 2);
const _: () = assert!(FreeRtosNotifyAction::SetValueWithOverwrite as u32 == 3);
const _: () = assert!(FreeRtosNotifyAction::SetValueWithoutOverwrite as u32 == 4);

const _: () = assert!(FreeRtosQueueSendPosition::SendToBack as u32 == 0);
const _: () = assert!(FreeRtosQueueSendPosition::SendToFront as u32 == 1);
const _: () = assert!(FreeRtosQueueSendPosition::Overwrite as u32 == 2);

const _: () = assert!(FreeRtosTaskState::Running as u32 == 0);
const _: () = assert!(FreeRtosTaskState::Ready as u32 == 1);
const _: () = assert!(FreeRtosTaskState::Blocked as u32 == 2);
const _: () = assert!(FreeRtosTaskState::Suspended as u32 == 3);
const _: () = assert!(FreeRtosTaskState::Deleted as u32 == 4);
const _: () = assert!(FreeRtosTaskState::Invalid as u32 == 5);

const _: () = assert!(FreeRtosTimerCommand::ExecuteCallbackFromISR as i32 == -2);
const _: () = assert!(FreeRtosTimerCommand::ExecuteCallback as i32 == -1);
const _: () = assert!(FreeRtosTimerCommand::StartDontTrace as i32 == 0);
const _: () = assert!(FreeRtosTimerCommand::Start as i32 == 1);
const _: () = assert!(FreeRtosTimerCommand::Reset as i32 == 2);
const _: () = assert!(FreeRtosTimerCommand::Stop as i32 == 3);
const _: () = assert!(FreeRtosTimerCommand::ChangePeriod as i32 == 4);
const _: () = assert!(FreeRtosTimerCommand::Delete as i32 == 5);
const _: () = assert!(FreeRtosTimerCommand::StartFromISR as i32 == 6);
const _: () = assert!(FreeRtosTimerCommand::ResetFromISR as i32 == 7);
const _: () = assert!(FreeRtosTimerCommand::StopFromISR as i32 == 8);
const _: () = assert!(FreeRtosTimerCommand::ChangePeriodFromISR as i32 == 9);

// Constants
const _: () = assert!(PD_TRUE == 1);
const _: () = assert!(PD_FALSE == 0);
const _: () = assert!(PD_PASS == 1);
const _: () = assert!(PD_FAIL == 0);
const _: () = assert!(PORT_MAX_DELAY == 0xFFFF_FFFF);
const _: () = assert!(TSK_DEFAULT_INDEX_TO_NOTIFY == 0);
const _: () = assert!(TSK_IDLE_PRIORITY == 0);
const _: () = assert!(TSK_NO_AFFINITY == 0xFFFF_FFFF);

// Struct layouts
const _: () = assert!(core::mem::size_of::<FreeRtosTimeOut>() == core::mem::size_of::<FreeRtosBaseType>() + core::mem::size_of::<FreeRtosTickType>());
const _: () = assert!(core::mem::align_of::<FreeRtosTimeOut>() >= 4);
const _: () = assert!(core::mem::size_of::<FreeRtosHeapStats>() == 7 * core::mem::size_of::<usize>());

// FreeRtosTaskStatusFfi must match C TaskStatus_t on 32-bit targets:
// 7 pointers/u32s + 1 u16 + padding = 7*4 + 2 + 2(padding) = 32 bytes on 32-bit
// On host (64-bit): pointers are 8 bytes, so size differs — use a range check.
const _: () = assert!(core::mem::size_of::<FreeRtosTaskStatusFfi>() >= 7 * 4 + 2);

// FreeRtosTaskState is repr(u32) — 4 bytes, matching C enum eTaskState (int)
const _: () = assert!(core::mem::size_of::<FreeRtosTaskState>() == 4);

// NotifyAction is repr(C) — matches C enum eNotifyAction (int)
const _: () = assert!(core::mem::size_of::<FreeRtosNotifyAction>() == 4);

// MPU region flags
const _: () = assert!(TSK_MPU_REGION_READ_ONLY == 1);
const _: () = assert!(TSK_MPU_REGION_READ_WRITE == 2);
const _: () = assert!(TSK_MPU_REGION_EXECUTE_NEVER == 4);
const _: () = assert!(TSK_MPU_REGION_NORMAL_MEMORY == 8);
const _: () = assert!(TSK_MPU_REGION_DEVICE_MEMORY == 16);

// FreeRtosMemoryRegion is 3 pointers/ints on 32-bit
const _: () = assert!(core::mem::size_of::<FreeRtosMemoryRegion>() == core::mem::size_of::<usize>() + 8);

// PORT_NUM_CONFIGURABLE_REGIONS default
const _: () = assert!(PORT_NUM_CONFIGURABLE_REGIONS >= 1);

