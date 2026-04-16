//! FreeRTOS queue management module.
//!
//! Provides FFI bindings and a safe generic `Queue<T>` wrapper for FreeRTOS
//! queue operations. Queues are the primary inter-task communication mechanism
//! in FreeRTOS, supporting FIFO, LIFO (front-send), and overwrite semantics.
//!
//! # Safe Wrapper
//!
//! [`Queue<T>`] provides a type-safe, RAII-managed queue. The generic parameter
//! `T` must be `Copy` or at least `Send` to ensure safe cross-task transfer.
//!
//! # Example
//!
//! ```rust,no_run
//! use freertos_api_rs::queue::Queue;
//!
//! let queue: Queue<u32> = Queue::new(10).expect("queue create failed");
//! queue.send(&42, 100).expect("send timeout");
//! if let Some(val) = queue.receive(100) {
//!     // Process val
//! }
//! ```

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosQueueHandle, FreeRtosUBaseType,
    FreeRtosVoidPtr, FreeRtosQueueSetHandle, FreeRtosQueueSetMemberHandle,
    FreeRtosTaskHandle, FreeRtosError, PD_PASS,
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates a new queue with dynamically allocated memory.
    pub fn freertos_rs_queue_create(
        queue_length: FreeRtosUBaseType,
        item_size: FreeRtosUBaseType,
    ) -> FreeRtosQueueHandle;

    /// Creates a new queue with statically allocated memory.
    pub fn freertos_rs_queue_create_static(
        queue_length: FreeRtosUBaseType,
        item_size: FreeRtosUBaseType,
        storage_buffer: *mut u8,
        queue_buffer: FreeRtosVoidPtr,
    ) -> FreeRtosQueueHandle;

    /// Deletes a queue and frees its memory.
    pub fn freertos_rs_queue_delete(queue: FreeRtosQueueHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sends an item to the back of a queue.
    ///
    /// Wraps `xQueueSend()` (backwards-compatible alias for `xQueueSendToBack`).
    pub fn freertos_rs_queue_send(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Sends an item to the front of a queue.
    pub fn freertos_rs_queue_send_to_front(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Sends an item to the back of a queue (explicit).
    pub fn freertos_rs_queue_send_to_back(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Receives an item from a queue.
    pub fn freertos_rs_queue_receive(
        queue: FreeRtosQueueHandle,
        buffer: FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Peeks at an item without removing it.
    pub fn freertos_rs_queue_peek(
        queue: FreeRtosQueueHandle,
        buffer: FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Overwrites an item in a length-1 queue.
    pub fn freertos_rs_queue_overwrite(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;

    /// Generic send with explicit copy position.
    pub fn freertos_rs_queue_generic_send(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
        copy_position: FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Resets a queue to its empty state.
    pub fn freertos_rs_queue_reset(queue: FreeRtosQueueHandle) -> FreeRtosBaseType;

    /// Generic reset with `xNewQueue` flag.
    pub fn freertos_rs_queue_generic_reset(
        queue: FreeRtosQueueHandle,
        new_queue: FreeRtosBaseType,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sends to the back of a queue from an ISR.
    ///
    /// Wraps `xQueueSendFromISR()` (backwards-compatible alias for `xQueueSendToBackFromISR`).
    pub fn freertos_rs_queue_send_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Sends to the front of a queue from an ISR.
    pub fn freertos_rs_queue_send_to_front_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Sends to the back of a queue from an ISR (explicit).
    pub fn freertos_rs_queue_send_to_back_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Receives from a queue in an ISR.
    pub fn freertos_rs_queue_receive_from_isr(
        queue: FreeRtosQueueHandle,
        buffer: FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Gives to a queue from ISR without copying data.
    ///
    /// Wraps `xQueueGiveFromISR()`. Use with queues that have item size 0
    /// (i.e., queues used as counting semaphores). Returns `pdTRUE` if a
    /// context switch is needed.
    pub fn freertos_rs_queue_give_from_isr(
        queue: FreeRtosQueueHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Overwrites from an ISR.
    pub fn freertos_rs_queue_overwrite_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Generic send from ISR.
    pub fn freertos_rs_queue_generic_send_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
        copy_position: FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Peeks at an item from an ISR.
    pub fn freertos_rs_queue_peek_from_isr(
        queue: FreeRtosQueueHandle,
        buffer: FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Returns the number of messages waiting in a queue.
    pub fn freertos_rs_queue_messages_waiting(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;

    /// Returns messages waiting from an ISR.
    pub fn freertos_rs_queue_messages_waiting_from_isr(
        queue: FreeRtosQueueHandle,
    ) -> FreeRtosUBaseType;

    /// Checks if a queue is empty from an ISR.
    pub fn freertos_rs_queue_is_queue_empty_from_isr(
        queue: FreeRtosQueueHandle,
    ) -> FreeRtosBaseType;

    /// Checks if a queue is full from an ISR.
    pub fn freertos_rs_queue_is_queue_full_from_isr(
        queue: FreeRtosQueueHandle,
    ) -> FreeRtosBaseType;

    /// Returns the number of free spaces in a queue.
    pub fn freertos_rs_queue_spaces_available(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE SETS
//===========================================================================

unsafe extern "C" {
    /// Creates a queue set.
    pub fn freertos_rs_queue_create_set(
        set_length: FreeRtosUBaseType,
    ) -> FreeRtosQueueSetHandle;

    /// Creates a queue set with static allocation.
    pub fn freertos_rs_queue_create_set_static(
        set_length: FreeRtosUBaseType,
        storage_buffer: *mut u8,
        static_queue: FreeRtosVoidPtr,
    ) -> FreeRtosQueueSetHandle;

    /// Adds a queue or semaphore to a queue set.
    pub fn freertos_rs_queue_add_to_set(
        queue_or_semaphore: FreeRtosQueueSetMemberHandle,
        queue_set: FreeRtosQueueSetHandle,
    ) -> FreeRtosBaseType;

    /// Removes a queue or semaphore from a queue set.
    pub fn freertos_rs_queue_remove_from_set(
        queue_or_semaphore: FreeRtosQueueSetMemberHandle,
        queue_set: FreeRtosQueueSetHandle,
    ) -> FreeRtosBaseType;

    /// Selects from a queue set (blocks until a member is ready).
    pub fn freertos_rs_queue_select_from_set(
        queue_set: FreeRtosQueueSetHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosQueueSetMemberHandle;

    /// Selects from a queue set from an ISR.
    pub fn freertos_rs_queue_select_from_set_from_isr(
        queue_set: FreeRtosQueueSetHandle,
    ) -> FreeRtosQueueSetMemberHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MUTEX INTERNALS
//===========================================================================

unsafe extern "C" {
    /// Creates a mutex (internal).
    pub fn freertos_rs_queue_create_mutex(queue_type: u8) -> FreeRtosQueueHandle;

    /// Creates a mutex with static allocation (internal).
    pub fn freertos_rs_queue_create_mutex_static(
        queue_type: u8,
        static_queue: FreeRtosVoidPtr,
    ) -> FreeRtosQueueHandle;

    /// Gets the mutex holder task handle.
    pub fn freertos_rs_queue_get_mutex_holder(semaphore: FreeRtosQueueHandle) -> FreeRtosTaskHandle;

    /// Gets the mutex holder from an ISR.
    pub fn freertos_rs_queue_get_mutex_holder_from_isr(
        semaphore: FreeRtosQueueHandle,
    ) -> FreeRtosTaskHandle;

    /// Takes a semaphore (binary or counting).
    pub fn freertos_rs_queue_semaphore_take(
        queue: FreeRtosQueueHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Takes a recursive mutex.
    pub fn freertos_rs_queue_take_mutex_recursive(
        mutex: FreeRtosQueueHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Gives a recursive mutex.
    pub fn freertos_rs_queue_give_mutex_recursive(mutex: FreeRtosQueueHandle) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE REGISTRY & DEBUG
//===========================================================================

unsafe extern "C" {
    /// Gets static buffers for a queue.
    pub fn freertos_rs_queue_get_static_buffers(
        queue: FreeRtosQueueHandle,
        queue_storage: *mut *mut u8,
        static_queue: *mut FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;

    /// Gets the item size of a queue.
    pub fn freertos_rs_queue_get_queue_item_size(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;

    /// Gets the length (capacity) of a queue.
    pub fn freertos_rs_queue_get_queue_length(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;

    /// Adds a queue to the registry.
    pub fn freertos_rs_queue_add_to_registry(queue: FreeRtosQueueHandle, queue_name: *const u8);

    /// Removes a queue from the registry.
    pub fn freertos_rs_queue_unregister_queue(queue: FreeRtosQueueHandle);

    /// Gets the name of a queue.
    pub fn freertos_rs_queue_get_name(queue: FreeRtosQueueHandle) -> *const u8;

    /// Sets the queue number for tracing.
    pub fn freertos_rs_queue_set_queue_number(queue: FreeRtosQueueHandle, queue_number: FreeRtosUBaseType);

    /// Gets the queue number for tracing.
    pub fn freertos_rs_queue_get_queue_number(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;

    /// Gets the queue type.
    pub fn freertos_rs_queue_get_queue_type(queue: FreeRtosQueueHandle) -> u8;

    /// Waits for a message with restricted permissions (MPU).
    ///
    /// Wraps `vQueueWaitForMessageRestricted()`.
    pub fn freertos_rs_queue_wait_for_message_restricted(
        queue: FreeRtosQueueHandle,
        ticks_to_wait: FreeRtosTickType,
        wait_indefinitely: FreeRtosBaseType,
    );
}

//===========================================================================
// SAFE WRAPPER - QUEUE<T>
//===========================================================================

/// A type-safe FreeRTOS queue with RAII memory management.
///
/// The queue stores items of type `T` and automatically deletes itself when dropped.
///
/// # Type Requirements
///
/// `T` must be `Send` because items are transferred between tasks.
///
/// # Example
///
/// ```rust,no_run
/// use freertos_api_rs::queue::Queue;
///
/// let queue: Queue<u32> = Queue::new(10).unwrap();
/// queue.send(&42, 100).unwrap();
/// let value = queue.receive(100);
/// ```
pub struct Queue<T> {
    handle: FreeRtosQueueHandle,
    _marker: core::marker::PhantomData<T>,
}

impl<T> Queue<T> {
    /// Creates a new queue that can hold `length` items of type `T`.
    ///
    /// # Errors
    ///
    /// Returns [`FreeRtosError::OutOfMemory`] if the FreeRTOS heap cannot
    /// accommodate the queue.
    pub fn new(length: FreeRtosUBaseType) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_queue_create(length, core::mem::size_of::<T>() as FreeRtosUBaseType)
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self {
                handle,
                _marker: core::marker::PhantomData,
            })
        }
    }

    /// Sends an item to the back of the queue.
    ///
    /// Blocks for up to `ticks_to_wait` ticks if the queue is full.
    /// Use [`PORT_MAX_DELAY`](crate::base::PORT_MAX_DELAY) to wait indefinitely.
    pub fn send(&self, item: &T, ticks_to_wait: FreeRtosTickType) -> Result<(), FreeRtosError> {
        let result = unsafe {
            freertos_rs_queue_send(
                self.handle,
                item as *const T as *const FreeRtosVoidPtr,
                ticks_to_wait,
            )
        };
        if result == PD_PASS {
            Ok(())
        } else {
            Err(FreeRtosError::QueueSendTimeout)
        }
    }

    /// Sends an item to the front of the queue.
    pub fn send_to_front(&self, item: &T, ticks_to_wait: FreeRtosTickType) -> Result<(), FreeRtosError> {
        let result = unsafe {
            freertos_rs_queue_send_to_front(
                self.handle,
                item as *const T as *const FreeRtosVoidPtr,
                ticks_to_wait,
            )
        };
        if result == PD_PASS {
            Ok(())
        } else {
            Err(FreeRtosError::QueueSendTimeout)
        }
    }

    /// Receives an item from the queue.
    ///
    /// Returns `Some(item)` on success, `None` if the receive timed out.
    /// **Note:** `T` must be `Copy` or you must ensure the item is fully
    /// initialized by FreeRTOS before use.
    pub fn receive(&self, ticks_to_wait: FreeRtosTickType) -> Option<T> {
        let mut item = core::mem::MaybeUninit::<T>::uninit();
        let result = unsafe {
            freertos_rs_queue_receive(
                self.handle,
                item.as_mut_ptr() as FreeRtosVoidPtr,
                ticks_to_wait,
            )
        };
        if result == PD_PASS {
            Some(unsafe { item.assume_init() })
        } else {
            None
        }
    }

    /// Peeks at the front item without removing it.
    pub fn peek(&self, ticks_to_wait: FreeRtosTickType) -> Option<T> {
        let mut item = core::mem::MaybeUninit::<T>::uninit();
        let result = unsafe {
            freertos_rs_queue_peek(
                self.handle,
                item.as_mut_ptr() as FreeRtosVoidPtr,
                ticks_to_wait,
            )
        };
        if result == PD_PASS {
            Some(unsafe { item.assume_init() })
        } else {
            None
        }
    }

    /// Returns the number of items currently in the queue.
    pub fn messages_waiting(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_queue_messages_waiting(self.handle) }
    }

    /// Returns the number of free spaces in the queue.
    pub fn spaces_available(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_queue_spaces_available(self.handle) }
    }

    /// Resets the queue to its empty state.
    pub fn reset(&self) -> Result<(), FreeRtosError> {
        let result = unsafe { freertos_rs_queue_reset(self.handle) };
        if result == PD_PASS {
            Ok(())
        } else {
            Err(FreeRtosError::QueueFull)
        }
    }

    /// Sends an item from an ISR context.
    pub fn send_from_isr(
        &self,
        item: &T,
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> Result<(), FreeRtosError> {
        let result = unsafe {
            freertos_rs_queue_send_from_isr(
                self.handle,
                item as *const T as *const FreeRtosVoidPtr,
                higher_priority_task_woken,
            )
        };
        if result == PD_PASS {
            Ok(())
        } else {
            Err(FreeRtosError::QueueSendTimeout)
        }
    }

    /// Receives an item from an ISR context.
    pub fn receive_from_isr(
        &self,
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> Option<T> {
        let mut item = core::mem::MaybeUninit::<T>::uninit();
        let result = unsafe {
            freertos_rs_queue_receive_from_isr(
                self.handle,
                item.as_mut_ptr() as FreeRtosVoidPtr,
                higher_priority_task_woken,
            )
        };
        if result == PD_PASS {
            Some(unsafe { item.assume_init() })
        } else {
            None
        }
    }

    /// Overwrites the front item in a length-1 queue.
    ///
    /// Only valid for queues created with length 1. Does not block.
    pub fn overwrite(&self, item: &T) -> Result<(), FreeRtosError> {
        let result = unsafe {
            freertos_rs_queue_overwrite(
                self.handle,
                item as *const T as *const FreeRtosVoidPtr,
            )
        };
        if result == PD_PASS {
            Ok(())
        } else {
            Err(FreeRtosError::QueueFull)
        }
    }

    /// Overwrites the front item from an ISR.
    ///
    /// Only valid for queues created with length 1.
    pub fn overwrite_from_isr(
        &self,
        item: &T,
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> bool {
        let result = unsafe {
            freertos_rs_queue_overwrite_from_isr(
                self.handle,
                item as *const T as *const FreeRtosVoidPtr,
                higher_priority_task_woken,
            )
        };
        result == PD_PASS
    }

    /// Peeks at the front item from an ISR without removing it.
    pub fn peek_from_isr(&self) -> Option<T> {
        let mut item = core::mem::MaybeUninit::<T>::uninit();
        let result = unsafe {
            freertos_rs_queue_peek_from_isr(
                self.handle,
                item.as_mut_ptr() as FreeRtosVoidPtr,
            )
        };
        if result == PD_PASS {
            Some(unsafe { item.assume_init() })
        } else {
            None
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe { freertos_rs_queue_delete(self.handle) };
    }
}

// Safety: Queue handles are safe to share between threads — FreeRTOS manages
// the internal synchronization.
unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Send> Sync for Queue<T> {}

//===========================================================================
// UNIT TESTS
//===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<Queue<u32>>();
        assert_sync::<Queue<u32>>();
    }
}
