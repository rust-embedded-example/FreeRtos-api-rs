/*!
 * FreeRTOS Semaphore and Mutex Module
 * 
 * This module provides unsafe extern "C" declarations for FreeRTOS semaphore and mutex functions.
 * The actual implementations are in api.c as C wrapper functions.
 * 
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosSemaphoreHandle, FreeRtosMutexHandle,
    FreeRtosUBaseType, FreeRtosVoidPtr
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - BINARY SEMAPHORES
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xSemaphoreCreateBinary()
    /// Creates a binary semaphore
    pub fn freertos_rs_semaphore_create_binary() -> FreeRtosSemaphoreHandle;
    
    /// Wrapper for xSemaphoreCreateBinaryStatic()
    /// Creates a binary semaphore using statically allocated memory
    pub fn freertos_rs_semaphore_create_binary_static(
        semaphore_buffer: FreeRtosVoidPtr
    ) -> FreeRtosSemaphoreHandle;
    
    /// Wrapper for vSemaphoreDelete()
    /// Deletes a semaphore
    pub fn freertos_rs_semaphore_delete(semaphore: FreeRtosSemaphoreHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - COUNTING SEMAPHORES
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xSemaphoreCreateCounting()
    /// Creates a counting semaphore
    pub fn freertos_rs_semaphore_create_counting(
        max_count: FreeRtosUBaseType,
        initial_count: FreeRtosUBaseType
    ) -> FreeRtosSemaphoreHandle;
    
    /// Wrapper for xSemaphoreCreateCountingStatic()
    /// Creates a counting semaphore using statically allocated memory
    pub fn freertos_rs_semaphore_create_counting_static(
        max_count: FreeRtosUBaseType,
        initial_count: FreeRtosUBaseType,
        semaphore_buffer: FreeRtosVoidPtr
    ) -> FreeRtosSemaphoreHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MUTEXES
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xSemaphoreCreateMutex()
    /// Creates a mutex
    pub fn freertos_rs_semaphore_create_mutex() -> FreeRtosMutexHandle;
    
    /// Wrapper for xSemaphoreCreateMutexStatic()
    /// Creates a mutex using statically allocated memory
    pub fn freertos_rs_semaphore_create_mutex_static(
        mutex_buffer: FreeRtosVoidPtr
    ) -> FreeRtosMutexHandle;
    
    /// Wrapper for xSemaphoreCreateRecursiveMutex()
    /// Creates a recursive mutex
    pub fn freertos_rs_semaphore_create_recursive_mutex() -> FreeRtosMutexHandle;
    
    /// Wrapper for xSemaphoreCreateRecursiveMutexStatic()
    /// Creates a recursive mutex using statically allocated memory
    pub fn freertos_rs_semaphore_create_recursive_mutex_static(
        mutex_buffer: FreeRtosVoidPtr
    ) -> FreeRtosMutexHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - SEMAPHORE OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xSemaphoreTake()
    /// Takes (acquires) a semaphore
    pub fn freertos_rs_semaphore_take(
        semaphore: FreeRtosSemaphoreHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xSemaphoreGive()
    /// Gives (releases) a semaphore
    pub fn freertos_rs_semaphore_give(
        semaphore: FreeRtosSemaphoreHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xSemaphoreTakeRecursive()
    /// Takes a recursive mutex
    pub fn freertos_rs_semaphore_take_recursive(
        mutex: FreeRtosMutexHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xSemaphoreGiveRecursive()
    /// Gives a recursive mutex
    pub fn freertos_rs_semaphore_give_recursive(
        mutex: FreeRtosMutexHandle
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - SEMAPHORE ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xSemaphoreTakeFromISR()
    /// Takes a semaphore from an ISR
    pub fn freertos_rs_semaphore_take_from_isr(
        semaphore: FreeRtosSemaphoreHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xSemaphoreGiveFromISR()
    /// Gives a semaphore from an ISR
    pub fn freertos_rs_semaphore_give_from_isr(
        semaphore: FreeRtosSemaphoreHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - SEMAPHORE INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for uxSemaphoreGetCount()
    /// Gets the count value of a semaphore
    pub fn freertos_rs_semaphore_get_count(
        semaphore: FreeRtosSemaphoreHandle
    ) -> FreeRtosUBaseType;
    
    /// Wrapper for uxSemaphoreGetCountFromISR()
    /// Gets the count value of a semaphore from an ISR
    pub fn freertos_rs_semaphore_get_count_from_isr(
        semaphore: FreeRtosSemaphoreHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for xSemaphoreGetStaticBuffer()
    /// Gets the static buffer associated with a semaphore
    pub fn freertos_rs_semaphore_get_static_buffer(
        semaphore: FreeRtosSemaphoreHandle,
        semaphore_buffer: *mut FreeRtosVoidPtr
    ) -> FreeRtosBaseType;
}
