/*!
 * FreeRTOS Co-routine Module
 * 
 * This module provides unsafe extern "C" declarations for FreeRTOS co-routine functions.
 * The actual implementations are in api.c as C wrapper functions.
 * 
 * Note: Co-routines are deprecated in favor of tasks in modern FreeRTOS applications.
 * 
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosUBaseType, FreeRtosVoidPtr
};

/// Co-routine handle type
pub type FreeRtosCoRoutineHandle = *const core::ffi::c_void;

/// Co-routine function type
pub type FreeRtosCoRoutineFunction = unsafe extern "C" fn(
    handle: FreeRtosCoRoutineHandle,
    index: FreeRtosUBaseType
);

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CO-ROUTINE CREATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xCoRoutineCreate()
    /// Creates a co-routine
    pub fn freertos_rs_co_routine_create(
        co_routine_code: FreeRtosCoRoutineFunction,
        priority: FreeRtosUBaseType,
        index: FreeRtosUBaseType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CO-ROUTINE SCHEDULER
//===========================================================================

unsafe extern "C" {
    /// Wrapper for vCoRoutineSchedule()
    /// Schedules co-routines
    pub fn freertos_rs_co_routine_schedule();
    
    /// Wrapper for vCoRoutineAddToDelayedList()
    /// Adds a co-routine to the delayed list
    pub fn freertos_rs_co_routine_add_to_delayed_list(
        ticks_to_delay: FreeRtosTickType,
        event_list: FreeRtosVoidPtr
    );
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CO-ROUTINE QUEUE OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for crQUEUE_SEND()
    /// Sends data to a queue from a co-routine
    pub fn freertos_rs_co_routine_queue_send(
        queue_handle: FreeRtosVoidPtr,
        item_to_queue: FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
        co_routine_handle: FreeRtosCoRoutineHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for crQUEUE_RECEIVE()
    /// Receives data from a queue in a co-routine
    pub fn freertos_rs_co_routine_queue_receive(
        queue_handle: FreeRtosVoidPtr,
        buffer: FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
        co_routine_handle: FreeRtosCoRoutineHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for crQUEUE_SEND_FROM_ISR()
    /// Sends data to a queue from an ISR for co-routines
    pub fn freertos_rs_co_routine_queue_send_from_isr(
        queue_handle: FreeRtosVoidPtr,
        item_to_queue: FreeRtosVoidPtr,
        co_routine_previously_woken: FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for crQUEUE_RECEIVE_FROM_ISR()
    /// Receives data from a queue from an ISR for co-routines
    pub fn freertos_rs_co_routine_queue_receive_from_isr(
        queue_handle: FreeRtosVoidPtr,
        buffer: FreeRtosVoidPtr,
        co_routine_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CO-ROUTINE DELAY
//===========================================================================

unsafe extern "C" {
    /// Wrapper for crDELAY()
    /// Delays a co-routine
    pub fn freertos_rs_co_routine_delay(
        ticks_to_delay: FreeRtosTickType,
        co_routine_handle: FreeRtosCoRoutineHandle
    );
}
