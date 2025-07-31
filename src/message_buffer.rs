use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosMessageBufferHandle,
    FreeRtosVoidPtr
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER CREATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xMessageBufferCreate()
    /// Creates a message buffer
    pub fn freertos_rs_message_buffer_create(
        buffer_size_bytes: usize
    ) -> FreeRtosMessageBufferHandle;
    
    /// Wrapper for xMessageBufferCreateStatic()
    /// Creates a message buffer using statically allocated memory
    pub fn freertos_rs_message_buffer_create_static(
        buffer_size_bytes: usize,
        storage_buffer: *mut u8,
        message_buffer_struct: FreeRtosVoidPtr
    ) -> FreeRtosMessageBufferHandle;
    
    /// Wrapper for vMessageBufferDelete()
    /// Deletes a message buffer
    pub fn freertos_rs_message_buffer_delete(message_buffer: FreeRtosMessageBufferHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xMessageBufferSend()
    /// Sends a message to a message buffer
    pub fn freertos_rs_message_buffer_send(
        message_buffer: FreeRtosMessageBufferHandle,
        message: *const FreeRtosVoidPtr,
        message_length_bytes: usize,
        ticks_to_wait: FreeRtosTickType
    ) -> usize;
    
    /// Wrapper for xMessageBufferReceive()
    /// Receives a message from a message buffer
    pub fn freertos_rs_message_buffer_receive(
        message_buffer: FreeRtosMessageBufferHandle,
        buffer: FreeRtosVoidPtr,
        buffer_length_bytes: usize,
        ticks_to_wait: FreeRtosTickType
    ) -> usize;
    
    /// Wrapper for xMessageBufferReset()
    /// Resets a message buffer
    pub fn freertos_rs_message_buffer_reset(
        message_buffer: FreeRtosMessageBufferHandle
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xMessageBufferSendFromISR()
    /// Sends a message to a message buffer from an ISR
    pub fn freertos_rs_message_buffer_send_from_isr(
        message_buffer: FreeRtosMessageBufferHandle,
        message: *const FreeRtosVoidPtr,
        message_length_bytes: usize,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> usize;
    
    /// Wrapper for xMessageBufferReceiveFromISR()
    /// Receives a message from a message buffer from an ISR
    pub fn freertos_rs_message_buffer_receive_from_isr(
        message_buffer: FreeRtosMessageBufferHandle,
        buffer: FreeRtosVoidPtr,
        buffer_length_bytes: usize,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> usize;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xMessageBufferSpaceAvailable()
    /// Returns the amount of space available in a message buffer
    pub fn freertos_rs_message_buffer_space_available(
        message_buffer: FreeRtosMessageBufferHandle
    ) -> usize;
    
    /// Wrapper for xMessageBufferNextLengthBytes()
    /// Returns the length of the next message in a message buffer
    pub fn freertos_rs_message_buffer_next_length_bytes(
        message_buffer: FreeRtosMessageBufferHandle
    ) -> usize;
    
    /// Wrapper for xMessageBufferIsEmpty()
    /// Checks if a message buffer is empty
    pub fn freertos_rs_message_buffer_is_empty(
        message_buffer: FreeRtosMessageBufferHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xMessageBufferIsFull()
    /// Checks if a message buffer is full
    pub fn freertos_rs_message_buffer_is_full(
        message_buffer: FreeRtosMessageBufferHandle
    ) -> FreeRtosBaseType;
}
