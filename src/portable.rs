/*!
 * FreeRTOS Portable Layer Module
 * 
 * This module provides unsafe extern "C" declarations for FreeRTOS portable layer functions,
 * primarily memory management. The actual implementations are in api.c as C wrapper functions.
 * 
 * Only includes FreeRTOS public APIs from portable.h.
 * 
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

use crate::base::FreeRtosVoidPtr;

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MEMORY MANAGEMENT
//===========================================================================

unsafe extern "C" {
    /// Wrapper for pvPortMalloc()
    /// Allocates memory from the FreeRTOS heap
    pub fn freertos_rs_port_malloc(wanted_size: usize) -> FreeRtosVoidPtr;
    
    /// Wrapper for vPortFree()
    /// Frees memory previously allocated from the FreeRTOS heap
    pub fn freertos_rs_port_free(ptr: FreeRtosVoidPtr);
    
    /// Wrapper for xPortGetFreeHeapSize()
    /// Gets the amount of free heap space available
    pub fn freertos_rs_port_get_free_heap_size() -> usize;
    
    /// Wrapper for xPortGetMinimumEverFreeHeapSize()
    /// Gets the minimum amount of free heap space that has ever existed
    pub fn freertos_rs_port_get_minimum_ever_free_heap_size() -> usize;
}
