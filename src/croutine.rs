//! FreeRTOS co-routine module (deprecated).
//!
//! Provides FFI bindings for FreeRTOS co-routine APIs. Co-routines are a
//! lightweight alternative to tasks with very low overhead, but they are
//! **deprecated** in favor of tasks and should not be used in new designs.
//!
//! # Warning
//!
//! FreeRTOS considers co-routines deprecated. Use tasks instead for all
//! new development.

use crate::base::{FreeRtosBaseType, FreeRtosTickType, FreeRtosUBaseType, FreeRtosVoidPtr};

//===========================================================================
// TYPE DEFINITIONS
//===========================================================================

/// Co-routine handle type.
pub type FreeRtosCoRoutineHandle = *const core::ffi::c_void;

/// Co-routine function signature.
pub type FreeRtosCoRoutineFunction = unsafe extern "C" fn(
    handle: FreeRtosCoRoutineHandle,
    index: FreeRtosUBaseType,
);

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CO-ROUTINE CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates a co-routine.
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_co_routine_create(
        co_routine_code: FreeRtosCoRoutineFunction,
        priority: FreeRtosUBaseType,
        index: FreeRtosUBaseType,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CO-ROUTINE SCHEDULER
//===========================================================================

unsafe extern "C" {
    /// Schedules co-routines.
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_co_routine_schedule();

    /// Adds a co-routine to the delayed list.
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_co_routine_add_to_delayed_list(
        ticks_to_delay: FreeRtosTickType,
        event_list: FreeRtosVoidPtr,
    );

    /// Removes a co-routine from an event list.
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_co_routine_remove_from_event_list(event_list: FreeRtosVoidPtr) -> FreeRtosBaseType;

    /// Resets the co-routine state (internal).
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_co_routine_reset_state();
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CO-ROUTINE QUEUE OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sends data to a queue from a co-routine context.
    ///
    /// Wraps `xQueueCRSend()`. Note: this is only the queue operation half;
    /// the full `crQUEUE_SEND` macro also includes co-routine state machine
    /// transitions that cannot be replicated in Rust.
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_queue_cr_send(
        queue_handle: FreeRtosVoidPtr,
        item_to_queue: FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Receives data from a queue in a co-routine context.
    ///
    /// Wraps `xQueueCRReceive()`. Note: this is only the queue operation half;
    /// the full `crQUEUE_RECEIVE` macro also includes co-routine state machine
    /// transitions that cannot be replicated in Rust.
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_queue_cr_receive(
        queue_handle: FreeRtosVoidPtr,
        buffer: FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Sends data to a queue from an ISR for co-routines.
    ///
    /// Wraps `xQueueCRSendFromISR()`.
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_queue_cr_send_from_isr(
        queue_handle: FreeRtosVoidPtr,
        item_to_queue: FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Receives data from a queue from an ISR for co-routines.
    ///
    /// Wraps `xQueueCRReceiveFromISR()`.
    #[deprecated(note = "Co-routines are deprecated in FreeRTOS. Use tasks instead.")]
    pub fn freertos_rs_queue_cr_receive_from_isr(
        queue_handle: FreeRtosVoidPtr,
        buffer: FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;
}

// Note: crDELAY() is a C macro that combines vCoRoutineAddToDelayedList() with
// co-routine state machine transitions (crSET_STATE0). It cannot be wrapped as
// a standalone function. Use freertos_rs_co_routine_add_to_delayed_list() directly
// if you need the low-level delay functionality.
