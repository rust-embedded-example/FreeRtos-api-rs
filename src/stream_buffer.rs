/*!
 * FreeRTOS Stream Buffer Module
 * 
 * This module provides unsafe extern "C" declarations for FreeRTOS stream buffer functions.
 * The actual implementations are in api.c as C wrapper functions.
 * 
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosStreamBufferHandle,
    FreeRtosVoidPtr, FreeRtosUBaseType
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER CREATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xStreamBufferCreate()
    /// Creates a stream buffer
    pub fn freertos_rs_stream_buffer_create(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize
    ) -> FreeRtosStreamBufferHandle;
    
    /// Wrapper for xStreamBufferCreateStatic()
    /// Creates a stream buffer using statically allocated memory
    pub fn freertos_rs_stream_buffer_create_static(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
        storage_buffer: *mut u8,
        stream_buffer_struct: FreeRtosVoidPtr
    ) -> FreeRtosStreamBufferHandle;
    
    /// Wrapper for vStreamBufferDelete()
    /// Deletes a stream buffer
    pub fn freertos_rs_stream_buffer_delete(stream_buffer: FreeRtosStreamBufferHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xStreamBufferSend()
    /// Sends data to a stream buffer
    pub fn freertos_rs_stream_buffer_send(
        stream_buffer: FreeRtosStreamBufferHandle,
        data: *const FreeRtosVoidPtr,
        data_length_bytes: usize,
        ticks_to_wait: FreeRtosTickType
    ) -> usize;
    
    /// Wrapper for xStreamBufferReceive()
    /// Receives data from a stream buffer
    pub fn freertos_rs_stream_buffer_receive(
        stream_buffer: FreeRtosStreamBufferHandle,
        buffer: FreeRtosVoidPtr,
        buffer_length_bytes: usize,
        ticks_to_wait: FreeRtosTickType
    ) -> usize;
    
    /// Wrapper for xStreamBufferReset()
    /// Resets a stream buffer
    pub fn freertos_rs_stream_buffer_reset(
        stream_buffer: FreeRtosStreamBufferHandle
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xStreamBufferSendFromISR()
    /// Sends data to a stream buffer from an ISR
    pub fn freertos_rs_stream_buffer_send_from_isr(
        stream_buffer: FreeRtosStreamBufferHandle,
        data: *const FreeRtosVoidPtr,
        data_length_bytes: usize,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> usize;
    
    /// Wrapper for xStreamBufferReceiveFromISR()
    /// Receives data from a stream buffer from an ISR
    pub fn freertos_rs_stream_buffer_receive_from_isr(
        stream_buffer: FreeRtosStreamBufferHandle,
        buffer: FreeRtosVoidPtr,
        buffer_length_bytes: usize,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> usize;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xStreamBufferBytesAvailable()
    /// Returns the number of bytes available in a stream buffer
    pub fn freertos_rs_stream_buffer_bytes_available(
        stream_buffer: FreeRtosStreamBufferHandle
    ) -> usize;
    
    /// Wrapper for xStreamBufferSpacesAvailable()
    /// Returns the number of bytes that can be written to a stream buffer
    pub fn freertos_rs_stream_buffer_spaces_available(
        stream_buffer: FreeRtosStreamBufferHandle
    ) -> usize;
    
    /// Wrapper for xStreamBufferIsFull()
    /// Checks if a stream buffer is full
    pub fn freertos_rs_stream_buffer_is_full(
        stream_buffer: FreeRtosStreamBufferHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xStreamBufferIsEmpty()
    /// Checks if a stream buffer is empty
    pub fn freertos_rs_stream_buffer_is_empty(
        stream_buffer: FreeRtosStreamBufferHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xStreamBufferSetTriggerLevel()
    /// Sets the trigger level of a stream buffer
    pub fn freertos_rs_stream_buffer_set_trigger_level(
        stream_buffer: FreeRtosStreamBufferHandle,
        trigger_level: usize
    ) -> FreeRtosBaseType;
}
