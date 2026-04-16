//! FreeRTOS semaphore and mutex module.
//!
//! Provides FFI bindings and safe wrappers for FreeRTOS synchronization primitives:
//!
//! - **Binary Semaphore** ([`BinarySemaphore`]) — Simple signaling between tasks/ISRs
//! - **Counting Semaphore** ([`CountingSemaphore`]) — Count of available resources
//! - **Mutex** ([`Mutex`]) — Mutual exclusion with priority inheritance
//! - **Recursive Mutex** ([`RecursiveMutex`]) — Re-entrant mutual exclusion
//!
//! # Example
//!
//! ```rust,no_run
//! use freertos_api_rs::semphr::Mutex;
//!
//! let mut mutex = Mutex::new().expect("mutex create failed");
//! if mutex.lock(100) {
//!     // Critical section — exclusive access
//!     mutex.unlock();
//! }
//! ```

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosSemaphoreHandle, FreeRtosMutexHandle,
    FreeRtosUBaseType, FreeRtosVoidPtr, FreeRtosError, PD_PASS,
    FreeRtosTaskHandle,
};

//===========================================================================
// SEMAPHORE CONSTANTS
//===========================================================================

/// Binary semaphore queue length (`semBINARY_SEMAPHORE_QUEUE_LENGTH`).
pub const SEM_BINARY_SEMAPHORE_QUEUE_LENGTH: u8 = 1;

/// Semaphore queue item length (`semSEMAPHORE_QUEUE_ITEM_LENGTH`).
pub const SEM_SEMAPHORE_QUEUE_ITEM_LENGTH: u8 = 0;

/// Semaphore give block time (`semGIVE_BLOCK_TIME`).
pub const SEM_GIVE_BLOCK_TIME: FreeRtosTickType = 0;

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - BINARY SEMAPHORES
//===========================================================================

unsafe extern "C" {
    /// Creates a binary semaphore.
    pub fn freertos_rs_semaphore_create_binary() -> FreeRtosSemaphoreHandle;

    /// Creates a binary semaphore with static allocation.
    pub fn freertos_rs_semaphore_create_binary_static(
        semaphore_buffer: FreeRtosVoidPtr,
    ) -> FreeRtosSemaphoreHandle;

    /// Deletes a semaphore.
    pub fn freertos_rs_semaphore_delete(semaphore: FreeRtosSemaphoreHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - COUNTING SEMAPHORES
//===========================================================================

unsafe extern "C" {
    /// Creates a counting semaphore.
    pub fn freertos_rs_semaphore_create_counting(
        max_count: FreeRtosUBaseType,
        initial_count: FreeRtosUBaseType,
    ) -> FreeRtosSemaphoreHandle;

    /// Creates a counting semaphore with static allocation.
    pub fn freertos_rs_semaphore_create_counting_static(
        max_count: FreeRtosUBaseType,
        initial_count: FreeRtosUBaseType,
        semaphore_buffer: FreeRtosVoidPtr,
    ) -> FreeRtosSemaphoreHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - MUTEXES
//===========================================================================

unsafe extern "C" {
    /// Creates a mutex with priority inheritance.
    pub fn freertos_rs_semaphore_create_mutex() -> FreeRtosMutexHandle;

    /// Creates a mutex with static allocation.
    pub fn freertos_rs_semaphore_create_mutex_static(
        mutex_buffer: FreeRtosVoidPtr,
    ) -> FreeRtosMutexHandle;

    /// Creates a recursive mutex.
    pub fn freertos_rs_semaphore_create_recursive_mutex() -> FreeRtosMutexHandle;

    /// Creates a recursive mutex with static allocation.
    pub fn freertos_rs_semaphore_create_recursive_mutex_static(
        mutex_buffer: FreeRtosVoidPtr,
    ) -> FreeRtosMutexHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - SEMAPHORE OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Takes (acquires) a semaphore.
    pub fn freertos_rs_semaphore_take(
        semaphore: FreeRtosSemaphoreHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Gives (releases) a semaphore.
    pub fn freertos_rs_semaphore_give(semaphore: FreeRtosSemaphoreHandle) -> FreeRtosBaseType;

    /// Takes a recursive mutex.
    pub fn freertos_rs_semaphore_take_recursive(
        mutex: FreeRtosMutexHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Gives a recursive mutex.
    pub fn freertos_rs_semaphore_give_recursive(mutex: FreeRtosMutexHandle) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - SEMAPHORE ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Takes a semaphore from an ISR.
    pub fn freertos_rs_semaphore_take_from_isr(
        semaphore: FreeRtosSemaphoreHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Gives a semaphore from an ISR.
    pub fn freertos_rs_semaphore_give_from_isr(
        semaphore: FreeRtosSemaphoreHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - SEMAPHORE INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Gets the count value of a semaphore.
    pub fn freertos_rs_semaphore_get_count(semaphore: FreeRtosSemaphoreHandle) -> FreeRtosUBaseType;

    /// Gets the count value from an ISR.
    pub fn freertos_rs_semaphore_get_count_from_isr(
        semaphore: FreeRtosSemaphoreHandle,
    ) -> FreeRtosUBaseType;

    /// Gets the static buffer associated with a semaphore.
    pub fn freertos_rs_semaphore_get_static_buffer(
        semaphore: FreeRtosSemaphoreHandle,
        semaphore_buffer: *mut FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;

    /// Gets the task that currently holds a mutex.
    ///
    /// Wraps `xSemaphoreGetMutexHolder()`. Returns `NULL` if the mutex is not held.
    pub fn freertos_rs_semaphore_get_mutex_holder(
        semaphore: FreeRtosSemaphoreHandle,
    ) -> FreeRtosTaskHandle;

    /// Gets the mutex holder from an ISR.
    ///
    /// Wraps `xSemaphoreGetMutexHolderFromISR()`.
    pub fn freertos_rs_semaphore_get_mutex_holder_from_isr(
        semaphore: FreeRtosSemaphoreHandle,
    ) -> FreeRtosTaskHandle;
}

//===========================================================================
// SAFE WRAPPER - BINARY SEMAPHORE
//===========================================================================

/// A binary semaphore for task synchronization.
///
/// Binary semaphores have two states: available (count = 1) and unavailable (count = 0).
/// Commonly used for ISR-to-task signaling.
pub struct BinarySemaphore {
    handle: FreeRtosSemaphoreHandle,
}

impl BinarySemaphore {
    /// Creates a new binary semaphore.
    ///
    /// The semaphore is created in the "taken" (unavailable) state.
    pub fn new() -> Result<Self, FreeRtosError> {
        let handle = unsafe { freertos_rs_semaphore_create_binary() };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Takes (acquires) the semaphore, blocking up to `ticks_to_wait`.
    ///
    /// Returns `true` if the semaphore was successfully taken.
    pub fn take(&self, ticks_to_wait: FreeRtosTickType) -> bool {
        unsafe { freertos_rs_semaphore_take(self.handle, ticks_to_wait) == PD_PASS }
    }

    /// Gives (releases) the semaphore.
    ///
    /// Returns `true` if the semaphore was successfully given.
    pub fn give(&self) -> bool {
        unsafe { freertos_rs_semaphore_give(self.handle) == PD_PASS }
    }

    /// Takes from an ISR. Returns `true` on success.
    pub fn take_from_isr(&self, higher_priority_task_woken: &mut FreeRtosBaseType) -> bool {
        unsafe {
            freertos_rs_semaphore_take_from_isr(self.handle, higher_priority_task_woken) == PD_PASS
        }
    }

    /// Gives from an ISR. Returns `true` on success.
    pub fn give_from_isr(&self, higher_priority_task_woken: &mut FreeRtosBaseType) -> bool {
        unsafe {
            freertos_rs_semaphore_give_from_isr(self.handle, higher_priority_task_woken) == PD_PASS
        }
    }
}

impl Drop for BinarySemaphore {
    fn drop(&mut self) {
        unsafe { freertos_rs_semaphore_delete(self.handle) };
    }
}

unsafe impl Send for BinarySemaphore {}
unsafe impl Sync for BinarySemaphore {}

//===========================================================================
// SAFE WRAPPER - COUNTING SEMAPHORE
//===========================================================================

/// A counting semaphore for resource management.
///
/// Counting semaphores track a count value between 0 and `max_count`.
/// Each `give` increments the count; each `take` decrements it.
pub struct CountingSemaphore {
    handle: FreeRtosSemaphoreHandle,
}

impl CountingSemaphore {
    /// Creates a new counting semaphore.
    ///
    /// # Arguments
    /// * `max_count` — Maximum count value
    /// * `initial_count` — Starting count value
    pub fn new(
        max_count: FreeRtosUBaseType,
        initial_count: FreeRtosUBaseType,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe { freertos_rs_semaphore_create_counting(max_count, initial_count) };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Takes (decrements) the semaphore.
    pub fn take(&self, ticks_to_wait: FreeRtosTickType) -> bool {
        unsafe { freertos_rs_semaphore_take(self.handle, ticks_to_wait) == PD_PASS }
    }

    /// Gives (increments) the semaphore.
    pub fn give(&self) -> bool {
        unsafe { freertos_rs_semaphore_give(self.handle) == PD_PASS }
    }

    /// Returns the current count value.
    pub fn count(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_semaphore_get_count(self.handle) }
    }
}

impl Drop for CountingSemaphore {
    fn drop(&mut self) {
        unsafe { freertos_rs_semaphore_delete(self.handle) };
    }
}

unsafe impl Send for CountingSemaphore {}
unsafe impl Sync for CountingSemaphore {}

//===========================================================================
// SAFE WRAPPER - MUTEX
//===========================================================================

/// A mutual exclusion primitive with priority inheritance.
///
/// Only one task can hold the mutex at a time. If a higher-priority task
/// attempts to acquire a mutex held by a lower-priority task, the holder's
/// priority is temporarily elevated (priority inheritance).
pub struct Mutex {
    handle: FreeRtosMutexHandle,
    owned: bool,
}

impl Mutex {
    /// Creates a new mutex.
    pub fn new() -> Result<Self, FreeRtosError> {
        let handle = unsafe { freertos_rs_semaphore_create_mutex() };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self {
                handle,
                owned: false,
            })
        }
    }

    /// Acquires the mutex, blocking up to `ticks_to_wait`.
    ///
    /// Returns `true` if the mutex was successfully acquired.
    pub fn lock(&mut self, ticks_to_wait: FreeRtosTickType) -> bool {
        let result =
            unsafe { freertos_rs_semaphore_take(self.handle as FreeRtosSemaphoreHandle, ticks_to_wait) };
        if result == PD_PASS {
            self.owned = true;
            true
        } else {
            false
        }
    }

    /// Releases the mutex.
    ///
    /// Returns `true` if the mutex was successfully released.
    pub fn unlock(&mut self) -> bool {
        let result = unsafe { freertos_rs_semaphore_give(self.handle as FreeRtosSemaphoreHandle) };
        if result == PD_PASS {
            self.owned = false;
            true
        } else {
            false
        }
    }

    /// Returns whether this mutex is currently held (locked).
    pub fn is_owned(&self) -> bool {
        self.owned
    }

    /// Returns the handle of the task currently holding this mutex, or `NULL` if unheld.
    pub fn get_holder(&self) -> FreeRtosTaskHandle {
        unsafe { freertos_rs_semaphore_get_mutex_holder(self.handle as FreeRtosSemaphoreHandle) }
    }

    /// Returns the handle of the task currently holding this mutex from an ISR.
    pub fn get_holder_from_isr(&self) -> FreeRtosTaskHandle {
        unsafe { freertos_rs_semaphore_get_mutex_holder_from_isr(self.handle as FreeRtosSemaphoreHandle) }
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        if self.owned {
            self.unlock();
        }
        unsafe { freertos_rs_semaphore_delete(self.handle as FreeRtosSemaphoreHandle) };
    }
}

unsafe impl Send for Mutex {}

//===========================================================================
// SAFE WRAPPER - RECURSIVE MUTEX
//===========================================================================

/// A recursive mutex that can be acquired multiple times by the same task.
///
/// Each `lock` must be paired with a corresponding `unlock`. The mutex is
/// only fully released when the lock count reaches zero.
pub struct RecursiveMutex {
    handle: FreeRtosMutexHandle,
}

impl RecursiveMutex {
    /// Creates a new recursive mutex.
    pub fn new() -> Result<Self, FreeRtosError> {
        let handle = unsafe { freertos_rs_semaphore_create_recursive_mutex() };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Acquires the recursive mutex. Can be called multiple times by the same task.
    pub fn lock(&self, ticks_to_wait: FreeRtosTickType) -> bool {
        unsafe {
            freertos_rs_semaphore_take_recursive(self.handle, ticks_to_wait) == PD_PASS
        }
    }

    /// Releases the recursive mutex. Must be called once per `lock`.
    pub fn unlock(&self) -> bool {
        unsafe { freertos_rs_semaphore_give_recursive(self.handle) == PD_PASS }
    }
}

impl Drop for RecursiveMutex {
    fn drop(&mut self) {
        unsafe { freertos_rs_semaphore_delete(self.handle as FreeRtosSemaphoreHandle) };
    }
}

unsafe impl Send for RecursiveMutex {}

//===========================================================================
// UNIT TESTS
//===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semaphore_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<BinarySemaphore>();
        assert_sync::<BinarySemaphore>();
        assert_send::<CountingSemaphore>();
        assert_sync::<CountingSemaphore>();
    }

    #[test]
    fn test_mutex_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Mutex>();
        assert_send::<RecursiveMutex>();
    }
}
