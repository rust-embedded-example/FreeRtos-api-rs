/*!
 * FreeRTOS Event Groups Module
 * 
 * This module provides unsafe extern "C" declarations for FreeRTOS event group functions.
 * The actual implementations are in api.c as C wrapper functions.
 * 
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosEventGroupHandle,
    FreeRtosEventBits, FreeRtosVoidPtr
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - EVENT GROUP CREATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xEventGroupCreate()
    /// Creates an event group
    pub fn freertos_rs_event_group_create() -> FreeRtosEventGroupHandle;
    
    /// Wrapper for xEventGroupCreateStatic()
    /// Creates an event group using statically allocated memory
    pub fn freertos_rs_event_group_create_static(
        event_group_buffer: FreeRtosVoidPtr
    ) -> FreeRtosEventGroupHandle;
    
    /// Wrapper for vEventGroupDelete()
    /// Deletes an event group
    pub fn freertos_rs_event_group_delete(event_group: FreeRtosEventGroupHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - EVENT GROUP OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xEventGroupSetBits()
    /// Sets bits in an event group
    pub fn freertos_rs_event_group_set_bits(
        event_group: FreeRtosEventGroupHandle,
        bits_to_set: FreeRtosEventBits
    ) -> FreeRtosEventBits;
    
    /// Wrapper for xEventGroupClearBits()
    /// Clears bits in an event group
    pub fn freertos_rs_event_group_clear_bits(
        event_group: FreeRtosEventGroupHandle,
        bits_to_clear: FreeRtosEventBits
    ) -> FreeRtosEventBits;
    
    /// Wrapper for xEventGroupGetBits()
    /// Gets the current value of the event group bits
    pub fn freertos_rs_event_group_get_bits(
        event_group: FreeRtosEventGroupHandle
    ) -> FreeRtosEventBits;
    
    /// Wrapper for xEventGroupWaitBits()
    /// Waits for bits to be set in an event group
    pub fn freertos_rs_event_group_wait_bits(
        event_group: FreeRtosEventGroupHandle,
        bits_to_wait_for: FreeRtosEventBits,
        clear_on_exit: FreeRtosBaseType,
        wait_for_all_bits: FreeRtosBaseType,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosEventBits;
    
    /// Wrapper for xEventGroupSync()
    /// Synchronizes tasks using an event group
    pub fn freertos_rs_event_group_sync(
        event_group: FreeRtosEventGroupHandle,
        bits_to_set: FreeRtosEventBits,
        bits_to_wait_for: FreeRtosEventBits,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosEventBits;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - EVENT GROUP ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xEventGroupSetBitsFromISR()
    /// Sets bits in an event group from an ISR
    pub fn freertos_rs_event_group_set_bits_from_isr(
        event_group: FreeRtosEventGroupHandle,
        bits_to_set: FreeRtosEventBits,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xEventGroupClearBitsFromISR()
    /// Clears bits in an event group from an ISR
    pub fn freertos_rs_event_group_clear_bits_from_isr(
        event_group: FreeRtosEventGroupHandle,
        bits_to_clear: FreeRtosEventBits
    ) -> FreeRtosEventBits;
    
    /// Wrapper for xEventGroupGetBitsFromISR()
    /// Gets the current value of the event group bits from an ISR
    pub fn freertos_rs_event_group_get_bits_from_isr(
        event_group: FreeRtosEventGroupHandle
    ) -> FreeRtosEventBits;
}
