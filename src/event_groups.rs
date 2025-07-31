use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosEventGroupHandle,
    FreeRtosEventBits, FreeRtosVoidPtr, FreeRtosUBaseType
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

    /// Wrapper for xEventGroupGetStaticBuffer()
    /// Gets the static buffer associated with an event group
    pub fn freertos_rs_event_group_get_static_buffer(
        event_group: FreeRtosEventGroupHandle,
        event_group_buffer: *mut FreeRtosVoidPtr
    ) -> FreeRtosBaseType;

    /// Wrapper for uxEventGroupGetNumber()
    /// Gets the event group number for tracing
    pub fn freertos_rs_event_group_get_number(
        event_group: FreeRtosEventGroupHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for vEventGroupSetNumber()
    /// Sets the event group number for tracing
    pub fn freertos_rs_event_group_set_number(
        event_group: FreeRtosEventGroupHandle,
        event_group_number: FreeRtosUBaseType
    );
}
