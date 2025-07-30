/*!
 * FreeRTOS Base Types and Error Definitions
 *
 * This module provides the fundamental types and error definitions used
 * throughout the FreeRTOS Rust API wrapper.
 *
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

use core::ffi::c_void;

//===========================================================================
// BASIC TYPE DEFINITIONS
//===========================================================================

/// FreeRTOS BaseType_t - typically a signed integer type
pub type FreeRtosBaseType = i32;

/// FreeRTOS UBaseType_t - typically an unsigned integer type
pub type FreeRtosUBaseType = u32;

/// FreeRTOS TickType_t - type used for tick counts and delays
pub type FreeRtosTickType = u32;

/// FreeRTOS void pointer type (mutable)
pub type FreeRtosVoidPtr = *mut c_void;

/// FreeRTOS void pointer type (const)
pub type FreeRtosConstVoidPtr = *const c_void;

/// FreeRTOS character pointer type
pub type FreeRtosCharPtr = *const u8;

/// FreeRTOS character type
pub type FreeRtosChar = u8;

/// FreeRTOS event bits type
pub type FreeRtosEventBitsType = u32;

/// FreeRTOS BaseType_t mutable pointer
pub type FreeRtosBaseTypeMutPtr = *mut FreeRtosBaseType;

//===========================================================================
// HANDLE TYPES
//===========================================================================

/// FreeRTOS task handle type
pub type FreeRtosTaskHandle = *const c_void;

/// FreeRTOS queue handle type
pub type FreeRtosQueueHandle = *const c_void;

/// FreeRTOS queue set handle type
pub type FreeRtosQueueSetHandle = *const c_void;

/// FreeRTOS queue set member handle type
pub type FreeRtosQueueSetMemberHandle = *const c_void;

/// FreeRTOS semaphore handle type
pub type FreeRtosSemaphoreHandle = *const c_void;

/// FreeRTOS mutex handle type
pub type FreeRtosMutexHandle = *const c_void;

/// FreeRTOS event group handle type
pub type FreeRtosEventGroupHandle = *const c_void;

/// FreeRTOS timer handle type
pub type FreeRtosTimerHandle = *const c_void;

/// FreeRTOS stream buffer handle type
pub type FreeRtosStreamBufferHandle = *const c_void;

/// FreeRTOS message buffer handle type
pub type FreeRtosMessageBufferHandle = *const c_void;

/// FreeRTOS task function type
pub type FreeRtosTaskFunction = unsafe extern "C" fn(*mut c_void);

/// FreeRTOS timer callback type
pub type FreeRtosTimerCallback = unsafe extern "C" fn(FreeRtosTimerHandle);

/// FreeRTOS stack type
pub type FreeRtosStackType = *mut c_void;

/// FreeRTOS static task type
pub type FreeRtosStaticTask = *mut c_void;

//===========================================================================
// ADDITIONAL TYPES
//===========================================================================

/// FreeRTOS unsigned long type
pub type FreeRtosUnsignedLong = u32;

/// FreeRTOS unsigned short type
pub type FreeRtosUnsignedShort = u16;

/// FreeRTOS event bits type
pub type FreeRtosEventBits = u32;

/// FreeRTOS notification value type
pub type FreeRtosNotificationValue = u32;

/// FreeRTOS configuration stack depth type
pub type FreeRtosConfigStackDepthType = u16;

/// FreeRTOS timeout structure
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FreeRtosTimeOut {
    pub overflow_count: FreeRtosBaseType,
    pub time_on_entering: FreeRtosTickType,
}

//===========================================================================
// ENUMERATION TYPES
//===========================================================================

/// Task notification action enumeration
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum FreeRtosNotifyAction {
    /// No action
    NoAction = 0,
    /// Set bits
    SetBits = 1,
    /// Increment
    Increment = 2,
    /// Set value with overwrite
    SetValueWithOverwrite = 3,
    /// Set value without overwrite
    SetValueWithoutOverwrite = 4,
}

/// Queue send position enumeration
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum FreeRtosQueueSendPosition {
    /// Send to back of queue
    SendToBack = 0,
    /// Send to front of queue
    SendToFront = 1,
    /// Overwrite queue
    Overwrite = 2,
}

/// Timer command enumeration
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum FreeRtosTimerCommand {
    /// Start timer
    Start = 0,
    /// Stop timer
    Stop = 1,
    /// Change period
    ChangePeriod = 2,
    /// Delete timer
    Delete = 3,
    /// Reset timer
    Reset = 4,
    /// Start from ISR
    StartFromISR = 5,
    /// Stop from ISR
    StopFromISR = 6,
    /// Change period from ISR
    ChangePeriodFromISR = 7,
    /// Delete from ISR
    DeleteFromISR = 8,
    /// Reset from ISR
    ResetFromISR = 9,
}

//===========================================================================
// ERROR DEFINITIONS
//===========================================================================

/// Basic error type for the library.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum FreeRtosError {
    OutOfMemory,
    QueueSendTimeout,
    QueueReceiveTimeout,
    MutexTimeout,
    Timeout,
    QueueFull,
    StringConversionError,
    TaskNotFound,
    InvalidQueueSize,
    ProcessorHasShutDown,
}

//===========================================================================
// TASK STATUS STRUCTURES
//===========================================================================

/// FreeRTOS task status structure for FFI
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FreeRtosTaskStatusFfi {
    pub handle: FreeRtosTaskHandle,
    pub task_name: FreeRtosCharPtr,
    pub task_number: FreeRtosUBaseType,
    pub task_state: FreeRtosTaskState,
    pub current_priority: FreeRtosUBaseType,
    pub base_priority: FreeRtosUBaseType,
    pub run_time_counter: FreeRtosUnsignedLong,
    pub stack_base: FreeRtosCharPtr,
    pub stack_high_water_mark: FreeRtosUnsignedShort,
}

/// FreeRTOS task state enumeration
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum FreeRtosTaskState {
    /// A task is querying the state of itself, so must be running.
    Running = 0,
    /// The task being queried is in a read or pending ready list.
    Ready = 1,
    /// The task being queried is in the Blocked state.
    Blocked = 2,
    /// The task being queried is in the Suspended state, or is in the Blocked state with an infinite time out.
    Suspended = 3,
    /// The task being queried has been deleted, but its TCB has not yet been freed.
    Deleted = 4,
}