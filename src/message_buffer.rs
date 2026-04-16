//! FreeRTOS message buffer module.
//!
//! Provides FFI bindings and a safe `MessageBuffer` wrapper for FreeRTOS message
//! buffers. Message buffers are built on top of stream buffers and add message
//! framing — each message is sent and received as a discrete unit with a length
//! prefix, unlike stream buffers which operate on raw byte streams.

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosMessageBufferHandle, FreeRtosVoidPtr,
    FreeRtosConstVoidPtr, FreeRtosError, PD_PASS, PD_TRUE, FreeRtosStreamBufferCallbackFunction,
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates a message buffer with dynamic allocation.
    pub fn freertos_rs_message_buffer_create(
        buffer_size_bytes: usize,
    ) -> FreeRtosMessageBufferHandle;

    /// Creates a message buffer with static allocation.
    pub fn freertos_rs_message_buffer_create_static(
        buffer_size_bytes: usize,
        storage_buffer: *mut u8,
        message_buffer_struct: FreeRtosVoidPtr,
    ) -> FreeRtosMessageBufferHandle;

    /// Deletes a message buffer.
    pub fn freertos_rs_message_buffer_delete(message_buffer: FreeRtosMessageBufferHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER WITH CALLBACK CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates a message buffer with send/receive completion callbacks.
    pub fn freertos_rs_message_buffer_create_with_callback(
        buffer_size_bytes: usize,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> FreeRtosMessageBufferHandle;

    /// Creates a static message buffer with send/receive completion callbacks.
    pub fn freertos_rs_message_buffer_create_static_with_callback(
        buffer_size_bytes: usize,
        storage_buffer: *mut u8,
        message_buffer_struct: FreeRtosVoidPtr,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> FreeRtosMessageBufferHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sends a message. Returns the number of bytes sent (0 on timeout or full).
    pub fn freertos_rs_message_buffer_send(
        message_buffer: FreeRtosMessageBufferHandle,
        message: FreeRtosConstVoidPtr,
        message_length_bytes: usize,
        ticks_to_wait: FreeRtosTickType,
    ) -> usize;

    /// Receives a message. Returns the number of bytes received (0 on timeout).
    pub fn freertos_rs_message_buffer_receive(
        message_buffer: FreeRtosMessageBufferHandle,
        buffer: FreeRtosVoidPtr,
        buffer_length_bytes: usize,
        ticks_to_wait: FreeRtosTickType,
    ) -> usize;

    /// Resets a message buffer.
    pub fn freertos_rs_message_buffer_reset(
        message_buffer: FreeRtosMessageBufferHandle,
    ) -> FreeRtosBaseType;

    /// Resets a message buffer from an ISR.
    pub fn freertos_rs_message_buffer_reset_from_isr(
        message_buffer: FreeRtosMessageBufferHandle,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sends a message from an ISR.
    pub fn freertos_rs_message_buffer_send_from_isr(
        message_buffer: FreeRtosMessageBufferHandle,
        message: FreeRtosConstVoidPtr,
        message_length_bytes: usize,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> usize;

    /// Receives a message from an ISR.
    pub fn freertos_rs_message_buffer_receive_from_isr(
        message_buffer: FreeRtosMessageBufferHandle,
        buffer: FreeRtosVoidPtr,
        buffer_length_bytes: usize,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> usize;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MESSAGE BUFFER INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Returns the free space available.
    pub fn freertos_rs_message_buffer_spaces_available(
        message_buffer: FreeRtosMessageBufferHandle,
    ) -> usize;

    /// Returns the length of the next message.
    pub fn freertos_rs_message_buffer_next_length_bytes(
        message_buffer: FreeRtosMessageBufferHandle,
    ) -> usize;

    /// Checks if a message buffer is empty.
    pub fn freertos_rs_message_buffer_is_empty(
        message_buffer: FreeRtosMessageBufferHandle,
    ) -> FreeRtosBaseType;

    /// Checks if a message buffer is full.
    pub fn freertos_rs_message_buffer_is_full(
        message_buffer: FreeRtosMessageBufferHandle,
    ) -> FreeRtosBaseType;

    /// Gets the static buffers associated with a message buffer.
    ///
    /// Wraps `xMessageBufferGetStaticBuffers()`.
    pub fn freertos_rs_message_buffer_get_static_buffers(
        message_buffer: FreeRtosMessageBufferHandle,
        storage_area: *mut *mut u8,
        static_buffer: *mut FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;

    /// ISR completion callback for send.
    ///
    /// Wraps `xMessageBufferSendCompletedFromISR()`. Called from an ISR after
    /// sending to a message buffer from non-FreeRTOS code.
    pub fn freertos_rs_message_buffer_send_completed_from_isr(
        message_buffer: FreeRtosMessageBufferHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// ISR completion callback for receive.
    ///
    /// Wraps `xMessageBufferReceiveCompletedFromISR()`. Called from an ISR after
    /// receiving from a message buffer from non-FreeRTOS code.
    pub fn freertos_rs_message_buffer_receive_completed_from_isr(
        message_buffer: FreeRtosMessageBufferHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// SAFE WRAPPER - MESSAGE BUFFER
//===========================================================================

/// A FreeRTOS message buffer for framed message transfer.
///
/// Unlike stream buffers, message buffers preserve message boundaries.
/// Each send/write is a discrete message; each read returns exactly one message.
pub struct MessageBuffer {
    handle: FreeRtosMessageBufferHandle,
}

impl MessageBuffer {
    /// Creates a new message buffer.
    pub fn new(buffer_size: usize) -> Result<Self, FreeRtosError> {
        let handle = unsafe { freertos_rs_message_buffer_create(buffer_size) };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Sends a message. Returns the number of bytes sent (0 on failure).
    pub fn send(&self, message: &[u8], ticks_to_wait: FreeRtosTickType) -> usize {
        unsafe {
            freertos_rs_message_buffer_send(
                self.handle,
                message.as_ptr() as FreeRtosConstVoidPtr,
                message.len(),
                ticks_to_wait,
            )
        }
    }

    /// Receives a message into the buffer. Returns bytes received (0 on timeout).
    pub fn receive(&self, buffer: &mut [u8], ticks_to_wait: FreeRtosTickType) -> usize {
        unsafe {
            freertos_rs_message_buffer_receive(
                self.handle,
                buffer.as_mut_ptr() as FreeRtosVoidPtr,
                buffer.len(),
                ticks_to_wait,
            )
        }
    }

    /// Resets the message buffer.
    pub fn reset(&self) -> bool {
        unsafe { freertos_rs_message_buffer_reset(self.handle) == PD_PASS }
    }

    /// Resets the message buffer from an ISR context.
    ///
    /// Returns `true` on success.
    pub fn reset_from_isr(&self) -> bool {
        unsafe { freertos_rs_message_buffer_reset_from_isr(self.handle) == PD_PASS }
    }

    /// Returns `true` if the message buffer is full.
    pub fn is_full(&self) -> bool {
        unsafe { freertos_rs_message_buffer_is_full(self.handle) == PD_TRUE }
    }

    /// Returns `true` if the message buffer is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { freertos_rs_message_buffer_is_empty(self.handle) == PD_TRUE }
    }

    /// Returns the available space in bytes.
    pub fn spaces_available(&self) -> usize {
        unsafe { freertos_rs_message_buffer_spaces_available(self.handle) }
    }

    /// Returns the length of the next queued message (0 if empty).
    pub fn next_length_bytes(&self) -> usize {
        unsafe { freertos_rs_message_buffer_next_length_bytes(self.handle) }
    }

    /// Sends a message from an ISR.
    pub fn send_from_isr(
        &self,
        message: &[u8],
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> usize {
        unsafe {
            freertos_rs_message_buffer_send_from_isr(
                self.handle,
                message.as_ptr() as FreeRtosConstVoidPtr,
                message.len(),
                higher_priority_task_woken,
            )
        }
    }

    /// Receives a message from an ISR.
    pub fn receive_from_isr(
        &self,
        buffer: &mut [u8],
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> usize {
        unsafe {
            freertos_rs_message_buffer_receive_from_isr(
                self.handle,
                buffer.as_mut_ptr() as FreeRtosVoidPtr,
                buffer.len(),
                higher_priority_task_woken,
            )
        }
    }
}

impl Drop for MessageBuffer {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { freertos_rs_message_buffer_delete(self.handle) };
        }
    }
}

unsafe impl Send for MessageBuffer {}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

const _: () = {
    const fn assert_send<T: Send>() {}
    assert_send::<MessageBuffer>();
};
