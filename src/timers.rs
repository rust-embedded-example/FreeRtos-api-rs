/*!
 * FreeRTOS Software Timers Module
 * 
 * This module provides unsafe extern "C" declarations for FreeRTOS software timer functions.
 * The actual implementations are in api.c as C wrapper functions.
 * 
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosTimerHandle,
    FreeRtosTimerCallback, FreeRtosVoidPtr
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER CREATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTimerCreate()
    /// Creates a software timer
    pub fn freertos_rs_timer_create(
        timer_name: *const u8,
        timer_period: FreeRtosTickType,
        auto_reload: FreeRtosBaseType,
        timer_id: FreeRtosVoidPtr,
        callback_function: FreeRtosTimerCallback
    ) -> FreeRtosTimerHandle;
    
    /// Wrapper for xTimerCreateStatic()
    /// Creates a software timer using statically allocated memory
    pub fn freertos_rs_timer_create_static(
        timer_name: *const u8,
        timer_period: FreeRtosTickType,
        auto_reload: FreeRtosBaseType,
        timer_id: FreeRtosVoidPtr,
        callback_function: FreeRtosTimerCallback,
        timer_buffer: FreeRtosVoidPtr
    ) -> FreeRtosTimerHandle;
    
    /// Wrapper for xTimerDelete()
    /// Deletes a software timer
    pub fn freertos_rs_timer_delete(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER CONTROL
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTimerStart()
    /// Starts a software timer
    pub fn freertos_rs_timer_start(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerStop()
    /// Stops a software timer
    pub fn freertos_rs_timer_stop(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerReset()
    /// Resets a software timer
    pub fn freertos_rs_timer_reset(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerChangePeriod()
    /// Changes the period of a software timer
    pub fn freertos_rs_timer_change_period(
        timer: FreeRtosTimerHandle,
        new_period: FreeRtosTickType,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTimerStartFromISR()
    /// Starts a software timer from an ISR
    pub fn freertos_rs_timer_start_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerStopFromISR()
    /// Stops a software timer from an ISR
    pub fn freertos_rs_timer_stop_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerResetFromISR()
    /// Resets a software timer from an ISR
    pub fn freertos_rs_timer_reset_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerChangePeriodFromISR()
    /// Changes the period of a software timer from an ISR
    pub fn freertos_rs_timer_change_period_from_isr(
        timer: FreeRtosTimerHandle,
        new_period: FreeRtosTickType,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTimerIsTimerActive()
    /// Checks if a timer is active
    pub fn freertos_rs_timer_is_timer_active(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerGetTimerDaemonTaskHandle()
    /// Gets the handle of the timer daemon task
    pub fn freertos_rs_timer_get_timer_daemon_task_handle() -> FreeRtosVoidPtr;
    
    /// Wrapper for xTimerGetPeriod()
    /// Gets the period of a timer
    pub fn freertos_rs_timer_get_period(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosTickType;
    
    /// Wrapper for xTimerGetExpiryTime()
    /// Gets the expiry time of a timer
    pub fn freertos_rs_timer_get_expiry_time(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosTickType;
    
    /// Wrapper for pvTimerGetTimerID()
    /// Gets the ID of a timer
    pub fn freertos_rs_timer_get_timer_id(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosVoidPtr;
    
    /// Wrapper for vTimerSetTimerID()
    /// Sets the ID of a timer
    pub fn freertos_rs_timer_set_timer_id(
        timer: FreeRtosTimerHandle,
        new_id: FreeRtosVoidPtr
    );
}
