//! `FreeRTOS` stream buffer module.
//!
//! Provides FFI bindings and a safe `StreamBuffer` wrapper for `FreeRTOS` stream
//! buffers. Stream buffers are variable-length byte streams optimized for
//! sending continuous streams of data between tasks and ISRs.
//!
//! Unlike queues, stream buffers operate on byte arrays rather than fixed-size items.

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosStreamBufferHandle, FreeRtosVoidPtr,
    FreeRtosConstVoidPtr, FreeRtosUBaseType, FreeRtosError, PD_PASS, PD_TRUE,
    FreeRtosStreamBufferCallbackFunction,
};

//===========================================================================
// STREAM BUFFER TYPE CONSTANTS
//===========================================================================

/// Stream buffer type identifier (`sbTYPE_STREAM_BUFFER`).
pub const SB_TYPE_STREAM_BUFFER: u8 = 0;

/// Message buffer type identifier (`sbTYPE_MESSAGE_BUFFER`).
pub const SB_TYPE_MESSAGE_BUFFER: u8 = 1;

/// Stream batching buffer type identifier (`sbTYPE_STREAM_BATCHING_BUFFER`).
pub const SB_TYPE_STREAM_BATCHING_BUFFER: u8 = 2;

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates a stream buffer with dynamic allocation.
    pub fn freertos_rs_stream_buffer_create(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
    ) -> FreeRtosStreamBufferHandle;

    /// Creates a stream buffer with static allocation.
    pub fn freertos_rs_stream_buffer_create_static(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
        storage_buffer: *mut u8,
        stream_buffer_struct: FreeRtosVoidPtr,
    ) -> FreeRtosStreamBufferHandle;

    /// Deletes a stream buffer.
    pub fn freertos_rs_stream_buffer_delete(stream_buffer: FreeRtosStreamBufferHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER WITH CALLBACK CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates a stream buffer with send/receive completion callbacks.
    pub fn freertos_rs_stream_buffer_create_with_callback(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> FreeRtosStreamBufferHandle;

    /// Creates a static stream buffer with send/receive completion callbacks.
    pub fn freertos_rs_stream_buffer_create_static_with_callback(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
        storage_buffer: *mut u8,
        stream_buffer_struct: FreeRtosVoidPtr,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> FreeRtosStreamBufferHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BATCHING BUFFER
//===========================================================================

unsafe extern "C" {
    /// Creates a batching stream buffer.
    pub fn freertos_rs_stream_batching_buffer_create(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
    ) -> FreeRtosStreamBufferHandle;

    /// Creates a batching stream buffer with callbacks.
    pub fn freertos_rs_stream_batching_buffer_create_with_callback(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> FreeRtosStreamBufferHandle;

    /// Creates a static batching stream buffer.
    pub fn freertos_rs_stream_batching_buffer_create_static(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
        storage_buffer: *mut u8,
        stream_buffer_struct: FreeRtosVoidPtr,
    ) -> FreeRtosStreamBufferHandle;

    /// Creates a static batching stream buffer with callbacks.
    pub fn freertos_rs_stream_batching_buffer_create_static_with_callback(
        buffer_size_bytes: usize,
        trigger_level_bytes: usize,
        storage_buffer: *mut u8,
        stream_buffer_struct: FreeRtosVoidPtr,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> FreeRtosStreamBufferHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sends data to a stream buffer.
    pub fn freertos_rs_stream_buffer_send(
        stream_buffer: FreeRtosStreamBufferHandle,
        data: FreeRtosConstVoidPtr,
        data_length_bytes: usize,
        ticks_to_wait: FreeRtosTickType,
    ) -> usize;

    /// Receives data from a stream buffer.
    pub fn freertos_rs_stream_buffer_receive(
        stream_buffer: FreeRtosStreamBufferHandle,
        buffer: FreeRtosVoidPtr,
        buffer_length_bytes: usize,
        ticks_to_wait: FreeRtosTickType,
    ) -> usize;

    /// Resets a stream buffer.
    pub fn freertos_rs_stream_buffer_reset(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> FreeRtosBaseType;

    /// Resets a stream buffer from an ISR.
    pub fn freertos_rs_stream_buffer_reset_from_isr(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> FreeRtosBaseType;

    /// ISR completion callback for send.
    pub fn freertos_rs_stream_buffer_send_completed_from_isr(
        stream_buffer: FreeRtosStreamBufferHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// ISR completion callback for receive.
    pub fn freertos_rs_stream_buffer_receive_completed_from_isr(
        stream_buffer: FreeRtosStreamBufferHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Gets the length of the next message in a batching stream buffer.
    pub fn freertos_rs_stream_buffer_next_message_length_bytes(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> usize;

    /// Gets the notification index for a stream buffer.
    pub fn freertos_rs_stream_buffer_get_notification_index(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> FreeRtosUBaseType;

    /// Sets the notification index for a stream buffer.
    pub fn freertos_rs_stream_buffer_set_notification_index(
        stream_buffer: FreeRtosStreamBufferHandle,
        notification_index: FreeRtosUBaseType,
    );
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sends data to a stream buffer from an ISR.
    pub fn freertos_rs_stream_buffer_send_from_isr(
        stream_buffer: FreeRtosStreamBufferHandle,
        data: FreeRtosConstVoidPtr,
        data_length_bytes: usize,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> usize;

    /// Receives data from a stream buffer from an ISR.
    pub fn freertos_rs_stream_buffer_receive_from_isr(
        stream_buffer: FreeRtosStreamBufferHandle,
        buffer: FreeRtosVoidPtr,
        buffer_length_bytes: usize,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> usize;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - STREAM BUFFER INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Returns the number of bytes available to read.
    pub fn freertos_rs_stream_buffer_bytes_available(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> usize;

    /// Returns the number of bytes that can be written.
    pub fn freertos_rs_stream_buffer_spaces_available(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> usize;

    /// Checks if a stream buffer is full.
    pub fn freertos_rs_stream_buffer_is_full(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> FreeRtosBaseType;

    /// Checks if a stream buffer is empty.
    pub fn freertos_rs_stream_buffer_is_empty(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> FreeRtosBaseType;

    /// Sets the trigger level.
    pub fn freertos_rs_stream_buffer_set_trigger_level(
        stream_buffer: FreeRtosStreamBufferHandle,
        trigger_level: usize,
    ) -> FreeRtosBaseType;

    /// Gets static buffers for a stream buffer.
    pub fn freertos_rs_stream_buffer_get_static_buffers(
        stream_buffer: FreeRtosStreamBufferHandle,
        storage_area: *mut *mut u8,
        static_buffer: *mut FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;

    /// Sets the stream buffer number for tracing.
    pub fn freertos_rs_stream_buffer_set_stream_buffer_number(
        stream_buffer: FreeRtosStreamBufferHandle,
        stream_buffer_number: FreeRtosUBaseType,
    );

    /// Gets the stream buffer number for tracing.
    pub fn freertos_rs_stream_buffer_get_stream_buffer_number(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> FreeRtosUBaseType;

    /// Gets the stream buffer type.
    pub fn freertos_rs_stream_buffer_get_stream_buffer_type(
        stream_buffer: FreeRtosStreamBufferHandle,
    ) -> u8;
}

//===========================================================================
// SAFE WRAPPER - STREAM BUFFER
//===========================================================================

/// A `FreeRTOS` stream buffer for byte-oriented data transfer.
///
/// Stream buffers are optimized for continuous data streams where the sender
/// and receiver may operate at different rates.
pub struct StreamBuffer {
    handle: FreeRtosStreamBufferHandle,
}

impl StreamBuffer {
    /// Creates a new stream buffer.
    ///
    /// # Arguments
    /// * `buffer_size` — Size of the internal buffer in bytes
    /// * `trigger_level` — Minimum bytes required before a blocked read unblocks
    pub fn new(buffer_size: usize, trigger_level: usize) -> Result<Self, FreeRtosError> {
        let handle = unsafe { freertos_rs_stream_buffer_create(buffer_size, trigger_level) };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Sends data to the stream buffer. Returns the number of bytes actually sent.
    pub fn send(&self, data: &[u8], ticks_to_wait: FreeRtosTickType) -> usize {
        unsafe {
            freertos_rs_stream_buffer_send(
                self.handle,
                data.as_ptr() as FreeRtosConstVoidPtr,
                data.len(),
                ticks_to_wait,
            )
        }
    }

    /// Receives data from the stream buffer. Returns the number of bytes received.
    pub fn receive(&self, buffer: &mut [u8], ticks_to_wait: FreeRtosTickType) -> usize {
        unsafe {
            freertos_rs_stream_buffer_receive(
                self.handle,
                buffer.as_mut_ptr() as FreeRtosVoidPtr,
                buffer.len(),
                ticks_to_wait,
            )
        }
    }

    /// Resets the stream buffer.
    pub fn reset(&self) -> bool {
        unsafe { freertos_rs_stream_buffer_reset(self.handle) == PD_PASS }
    }

    /// Returns `true` if the stream buffer is full.
    pub fn is_full(&self) -> bool {
        unsafe { freertos_rs_stream_buffer_is_full(self.handle) == PD_TRUE }
    }

    /// Returns `true` if the stream buffer is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { freertos_rs_stream_buffer_is_empty(self.handle) == PD_TRUE }
    }

    /// Returns the number of bytes available to read.
    pub fn bytes_available(&self) -> usize {
        unsafe { freertos_rs_stream_buffer_bytes_available(self.handle) }
    }

    /// Returns the number of bytes that can be written.
    pub fn spaces_available(&self) -> usize {
        unsafe { freertos_rs_stream_buffer_spaces_available(self.handle) }
    }

    /// Sets the trigger level. Returns `true` on success.
    pub fn set_trigger_level(&self, trigger_level: usize) -> bool {
        unsafe { freertos_rs_stream_buffer_set_trigger_level(self.handle, trigger_level) == PD_TRUE }
    }

    /// Sends data from an ISR. Returns bytes sent.
    pub fn send_from_isr(
        &self,
        data: &[u8],
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> usize {
        unsafe {
            freertos_rs_stream_buffer_send_from_isr(
                self.handle,
                data.as_ptr() as FreeRtosConstVoidPtr,
                data.len(),
                higher_priority_task_woken,
            )
        }
    }

    /// Receives data from an ISR. Returns bytes received.
    pub fn receive_from_isr(
        &self,
        buffer: &mut [u8],
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> usize {
        unsafe {
            freertos_rs_stream_buffer_receive_from_isr(
                self.handle,
                buffer.as_mut_ptr() as FreeRtosVoidPtr,
                buffer.len(),
                higher_priority_task_woken,
            )
        }
    }

    /// Sets the task notification index used by this stream buffer.
    ///
    /// When data is sent/received, the stream buffer notifies the waiting
    /// task using this notification index.
    pub fn set_notification_index(&self, index: FreeRtosUBaseType) {
        unsafe { freertos_rs_stream_buffer_set_notification_index(self.handle, index) };
    }

    /// Gets the task notification index used by this stream buffer.
    pub fn get_notification_index(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_stream_buffer_get_notification_index(self.handle) }
    }

    /// Creates a stream buffer with static memory allocation.
    ///
    /// # Safety
    /// `storage_buffer` must be properly aligned and `stream_buffer_struct`
    /// must point to valid `StaticStreamBuffer_t` memory. Both must remain
    /// valid for the stream buffer's lifetime.
    pub unsafe fn new_static(
        buffer_size: usize,
        trigger_level: usize,
        storage_buffer: *mut u8,
        stream_buffer_struct: FreeRtosVoidPtr,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_stream_buffer_create_static(buffer_size, trigger_level, storage_buffer, stream_buffer_struct)
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Resets the stream buffer from an ISR context.
    pub fn reset_from_isr(&self) -> bool {
        unsafe { freertos_rs_stream_buffer_reset_from_isr(self.handle) == PD_PASS }
    }

    /// Gets the stream buffer number (trace facility).
    pub fn stream_buffer_number(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_stream_buffer_get_stream_buffer_number(self.handle) }
    }

    /// Sets the stream buffer number (trace facility).
    pub fn set_stream_buffer_number(&self, number: FreeRtosUBaseType) {
        unsafe { freertos_rs_stream_buffer_set_stream_buffer_number(self.handle, number) }
    }

    /// Gets the stream buffer type (0=stream, 1=message, 2=batching).
    pub fn stream_buffer_type(&self) -> u8 {
        unsafe { freertos_rs_stream_buffer_get_stream_buffer_type(self.handle) }
    }

    /// ISR completion callback after sending to a stream buffer from non-FreeRTOS code.
    ///
    /// Call this from an ISR after writing data to the stream buffer's storage
    /// area directly (bypassing the `FreeRTOS` API). Returns `true` if a higher
    /// priority task was woken.
    pub fn send_completed_from_isr(&self, higher_priority_task_woken: &mut FreeRtosBaseType) -> bool {
        unsafe { freertos_rs_stream_buffer_send_completed_from_isr(self.handle, higher_priority_task_woken) != 0 }
    }

    /// ISR completion callback after receiving from a stream buffer from non-FreeRTOS code.
    ///
    /// Call this from an ISR after reading data from the stream buffer's storage
    /// area directly (bypassing the `FreeRTOS` API). Returns `true` if a higher
    /// priority task was woken.
    pub fn receive_completed_from_isr(&self, higher_priority_task_woken: &mut FreeRtosBaseType) -> bool {
        unsafe { freertos_rs_stream_buffer_receive_completed_from_isr(self.handle, higher_priority_task_woken) != 0 }
    }

    /// Gets the static buffers associated with this stream buffer.
    ///
    /// Returns `true` on success.
    ///
    /// # Safety
    /// `storage_area` must be a valid `*mut *mut u8` and `static_buffer`
    /// must be a valid `*mut FreeRtosVoidPtr` for output.
    pub unsafe fn get_static_buffers(
        &self,
        storage_area: *mut *mut u8,
        static_buffer: *mut FreeRtosVoidPtr,
    ) -> bool {
        unsafe { freertos_rs_stream_buffer_get_static_buffers(self.handle, storage_area, static_buffer) != 0 }
    }

    /// Creates a stream buffer with send/receive completed callbacks.
    ///
    /// # Safety
    /// `send_callback` and `receive_callback` must be valid function pointers or null.
    pub unsafe fn new_with_callback(
        buffer_size: usize,
        trigger_level: usize,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_stream_buffer_create_with_callback(
                buffer_size,
                trigger_level,
                send_callback,
                receive_callback,
            )
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Creates a stream buffer with static storage and send/receive completed callbacks.
    ///
    /// # Safety
    /// `stream_buffer_storage` must be large enough for `buffer_size + 1` bytes.
    /// `static_buffer` must point to a valid `StaticStreamBuffer_t`-sized buffer.
    /// Both must remain valid for the lifetime of the stream buffer.
    pub unsafe fn new_static_with_callback(
        buffer_size: usize,
        trigger_level: usize,
        stream_buffer_storage: *mut u8,
        static_buffer: FreeRtosVoidPtr,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_stream_buffer_create_static_with_callback(
                buffer_size,
                trigger_level,
                stream_buffer_storage,
                static_buffer,
                send_callback,
                receive_callback,
            )
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }
}

impl Drop for StreamBuffer {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { freertos_rs_stream_buffer_delete(self.handle) };
        }
    }
}

unsafe impl Send for StreamBuffer {}
unsafe impl Sync for StreamBuffer {}

//===========================================================================
// SAFE WRAPPER - BATCHING STREAM BUFFER
//===========================================================================

/// A `FreeRTOS` batching stream buffer for accumulating data before triggering a read.
///
/// Unlike regular stream buffers, batching buffers accumulate data until the
/// trigger level is reached before unblocking a waiting task. This is useful
/// for batching small writes into larger reads.
pub struct BatchingBuffer {
    handle: FreeRtosStreamBufferHandle,
}

impl BatchingBuffer {
    /// Creates a new batching stream buffer.
    ///
    /// # Arguments
    /// * `buffer_size` — Size of the internal buffer in bytes
    /// * `trigger_level` — Minimum bytes required before a blocked read unblocks
    pub fn new(buffer_size: usize, trigger_level: usize) -> Result<Self, FreeRtosError> {
        let handle = unsafe { freertos_rs_stream_batching_buffer_create(buffer_size, trigger_level) };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Sends data to the batching buffer. Returns the number of bytes actually sent.
    pub fn send(&self, data: &[u8], ticks_to_wait: FreeRtosTickType) -> usize {
        unsafe {
            freertos_rs_stream_buffer_send(
                self.handle,
                data.as_ptr() as FreeRtosConstVoidPtr,
                data.len(),
                ticks_to_wait,
            )
        }
    }

    /// Receives data from the batching buffer. Returns the number of bytes received.
    pub fn receive(&self, buffer: &mut [u8], ticks_to_wait: FreeRtosTickType) -> usize {
        unsafe {
            freertos_rs_stream_buffer_receive(
                self.handle,
                buffer.as_mut_ptr() as FreeRtosVoidPtr,
                buffer.len(),
                ticks_to_wait,
            )
        }
    }

    /// Resets the batching buffer.
    pub fn reset(&self) -> bool {
        unsafe { freertos_rs_stream_buffer_reset(self.handle) == PD_PASS }
    }

    /// Returns `true` if the batching buffer is full.
    pub fn is_full(&self) -> bool {
        unsafe { freertos_rs_stream_buffer_is_full(self.handle) == PD_TRUE }
    }

    /// Returns `true` if the batching buffer is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { freertos_rs_stream_buffer_is_empty(self.handle) == PD_TRUE }
    }

    /// Returns the number of bytes available to read.
    pub fn bytes_available(&self) -> usize {
        unsafe { freertos_rs_stream_buffer_bytes_available(self.handle) }
    }

    /// Returns the number of bytes that can be written.
    pub fn spaces_available(&self) -> usize {
        unsafe { freertos_rs_stream_buffer_spaces_available(self.handle) }
    }

    /// Sets the trigger level. Returns `true` on success.
    pub fn set_trigger_level(&self, trigger_level: usize) -> bool {
        unsafe { freertos_rs_stream_buffer_set_trigger_level(self.handle, trigger_level) == PD_TRUE }
    }

    /// Gets the length of the next message in the batching buffer.
    pub fn next_message_length_bytes(&self) -> usize {
        unsafe { freertos_rs_stream_buffer_next_message_length_bytes(self.handle) }
    }

    /// Sends data from an ISR. Returns bytes sent.
    pub fn send_from_isr(
        &self,
        data: &[u8],
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> usize {
        unsafe {
            freertos_rs_stream_buffer_send_from_isr(
                self.handle,
                data.as_ptr() as FreeRtosConstVoidPtr,
                data.len(),
                higher_priority_task_woken,
            )
        }
    }

    /// Receives data from an ISR. Returns bytes received.
    pub fn receive_from_isr(
        &self,
        buffer: &mut [u8],
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> usize {
        unsafe {
            freertos_rs_stream_buffer_receive_from_isr(
                self.handle,
                buffer.as_mut_ptr() as FreeRtosVoidPtr,
                buffer.len(),
                higher_priority_task_woken,
            )
        }
    }

    /// Creates a batching stream buffer with send/receive completed callbacks.
    ///
    /// # Safety
    /// `send_callback` and `receive_callback` must be valid function pointers or null.
    pub unsafe fn new_with_callback(
        buffer_size: usize,
        trigger_level: usize,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_stream_batching_buffer_create_with_callback(
                buffer_size,
                trigger_level,
                send_callback,
                receive_callback,
            )
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Creates a batching stream buffer with static storage.
    ///
    /// # Safety
    /// `stream_buffer_storage` must be large enough for `buffer_size + 1` bytes.
    /// `static_buffer` must point to a valid `StaticStreamBuffer_t`-sized buffer.
    pub unsafe fn new_static(
        buffer_size: usize,
        trigger_level: usize,
        stream_buffer_storage: *mut u8,
        static_buffer: FreeRtosVoidPtr,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_stream_batching_buffer_create_static(
                buffer_size,
                trigger_level,
                stream_buffer_storage,
                static_buffer,
            )
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Creates a batching stream buffer with static storage and send/receive completed callbacks.
    ///
    /// # Safety
    /// `stream_buffer_storage` must be large enough for `buffer_size + 1` bytes.
    /// `static_buffer` must point to a valid `StaticStreamBuffer_t`-sized buffer.
    pub unsafe fn new_static_with_callback(
        buffer_size: usize,
        trigger_level: usize,
        stream_buffer_storage: *mut u8,
        static_buffer: FreeRtosVoidPtr,
        send_callback: FreeRtosStreamBufferCallbackFunction,
        receive_callback: FreeRtosStreamBufferCallbackFunction,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_stream_batching_buffer_create_static_with_callback(
                buffer_size,
                trigger_level,
                stream_buffer_storage,
                static_buffer,
                send_callback,
                receive_callback,
            )
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }
}

impl Drop for BatchingBuffer {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { freertos_rs_stream_buffer_delete(self.handle) };
        }
    }
}

unsafe impl Send for BatchingBuffer {}
unsafe impl Sync for BatchingBuffer {}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

const _: () = {
    const fn assert_send<T: Send>() {}
    const fn assert_sync<T: Sync>() {}
    assert_send::<StreamBuffer>();
    assert_sync::<StreamBuffer>();
    assert_send::<BatchingBuffer>();
    assert_sync::<BatchingBuffer>();
};

const _: () = assert!(SB_TYPE_STREAM_BUFFER == 0);
const _: () = assert!(SB_TYPE_MESSAGE_BUFFER == 1);
const _: () = assert!(SB_TYPE_STREAM_BATCHING_BUFFER == 2);

// StreamBuffer and BatchingBuffer are pointer-sized
const _: () = assert!(core::mem::size_of::<StreamBuffer>() == core::mem::size_of::<FreeRtosStreamBufferHandle>());
const _: () = assert!(core::mem::size_of::<BatchingBuffer>() == core::mem::size_of::<FreeRtosStreamBufferHandle>());

// Alignment matches handle type
const _: () = assert!(core::mem::align_of::<StreamBuffer>() == core::mem::align_of::<FreeRtosStreamBufferHandle>());
const _: () = assert!(core::mem::align_of::<BatchingBuffer>() == core::mem::align_of::<FreeRtosStreamBufferHandle>());
