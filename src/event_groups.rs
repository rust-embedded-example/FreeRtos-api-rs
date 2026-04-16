//! FreeRTOS event group module.
//!
//! Provides FFI bindings and a safe `EventGroup` wrapper for FreeRTOS event groups.
//! Event groups allow tasks to wait for a combination of bits (events) to be set,
//! enabling complex synchronization patterns like barrier synchronization.
//!
//! # Example
//!
//! ```rust,no_run
//! use freertos_api_rs::event_groups::EventGroup;
//!
//! let eg = EventGroup::new().expect("event group create failed");
//! eg.set_bits(0x01);
//! let bits = eg.wait_bits(0x01, true, true, 1000);
//! ```

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosEventGroupHandle, FreeRtosEventBits,
    FreeRtosVoidPtr, FreeRtosUBaseType, FreeRtosError, PD_PASS, PD_TRUE,
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - EVENT GROUP CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates an event group with dynamic allocation.
    pub fn freertos_rs_event_group_create() -> FreeRtosEventGroupHandle;

    /// Creates an event group with static allocation.
    pub fn freertos_rs_event_group_create_static(
        event_group_buffer: FreeRtosVoidPtr,
    ) -> FreeRtosEventGroupHandle;

    /// Deletes an event group.
    pub fn freertos_rs_event_group_delete(event_group: FreeRtosEventGroupHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - EVENT GROUP OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sets bits in an event group. Returns the previous value.
    pub fn freertos_rs_event_group_set_bits(
        event_group: FreeRtosEventGroupHandle,
        bits_to_set: FreeRtosEventBits,
    ) -> FreeRtosEventBits;

    /// Clears bits in an event group. Returns the previous value.
    pub fn freertos_rs_event_group_clear_bits(
        event_group: FreeRtosEventGroupHandle,
        bits_to_clear: FreeRtosEventBits,
    ) -> FreeRtosEventBits;

    /// Gets the current event group bits.
    pub fn freertos_rs_event_group_get_bits(
        event_group: FreeRtosEventGroupHandle,
    ) -> FreeRtosEventBits;

    /// Waits for bits to be set. Returns the value when the condition is met.
    pub fn freertos_rs_event_group_wait_bits(
        event_group: FreeRtosEventGroupHandle,
        bits_to_wait_for: FreeRtosEventBits,
        clear_on_exit: FreeRtosBaseType,
        wait_for_all_bits: FreeRtosBaseType,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosEventBits;

    /// Synchronizes multiple tasks at a barrier.
    pub fn freertos_rs_event_group_sync(
        event_group: FreeRtosEventGroupHandle,
        bits_to_set: FreeRtosEventBits,
        bits_to_wait_for: FreeRtosEventBits,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosEventBits;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - EVENT GROUP ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Sets bits in an event group from an ISR.
    pub fn freertos_rs_event_group_set_bits_from_isr(
        event_group: FreeRtosEventGroupHandle,
        bits_to_set: FreeRtosEventBits,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Clears bits in an event group from an ISR.
    pub fn freertos_rs_event_group_clear_bits_from_isr(
        event_group: FreeRtosEventGroupHandle,
        bits_to_clear: FreeRtosEventBits,
    ) -> FreeRtosEventBits;

    /// Gets the current bits from an ISR.
    pub fn freertos_rs_event_group_get_bits_from_isr(
        event_group: FreeRtosEventGroupHandle,
    ) -> FreeRtosEventBits;

    /// Gets the static buffer for an event group.
    pub fn freertos_rs_event_group_get_static_buffer(
        event_group: FreeRtosEventGroupHandle,
        event_group_buffer: *mut FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;

    /// Gets the event group number for tracing.
    pub fn freertos_rs_event_group_get_number(
        event_group: FreeRtosEventGroupHandle,
    ) -> FreeRtosUBaseType;

    /// Sets the event group number for tracing.
    pub fn freertos_rs_event_group_set_number(
        event_group: FreeRtosEventGroupHandle,
        event_group_number: FreeRtosUBaseType,
    );

    /// Timer callback to set bits in an event group.
    ///
    /// Wraps `vEventGroupSetBitsCallback()`. Intended for use as a timer
    /// callback function to set bits when a timer expires.
    pub fn freertos_rs_event_group_set_bits_callback(
        event_group: FreeRtosVoidPtr,
        bits_to_set: u32,
    );

    /// Timer callback to clear bits in an event group.
    ///
    /// Wraps `vEventGroupClearBitsCallback()`. Intended for use as a timer
    /// callback function to clear bits when a timer expires.
    pub fn freertos_rs_event_group_clear_bits_callback(
        event_group: FreeRtosVoidPtr,
        bits_to_clear: u32,
    );
}

//===========================================================================
// SAFE WRAPPER - EVENT GROUP
//===========================================================================

/// An event group for synchronizing multiple tasks using bit flags.
///
/// Event groups allow tasks to set, clear, and wait on bit patterns,
/// enabling complex synchronization like barriers and multi-event waiting.
pub struct EventGroup {
    handle: FreeRtosEventGroupHandle,
}

impl EventGroup {
    /// Creates a new event group.
    pub fn new() -> Result<Self, FreeRtosError> {
        let handle = unsafe { freertos_rs_event_group_create() };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Sets bits in the event group. Returns the previous value.
    pub fn set_bits(&self, bits_to_set: FreeRtosEventBits) -> FreeRtosEventBits {
        unsafe { freertos_rs_event_group_set_bits(self.handle, bits_to_set) }
    }

    /// Clears bits in the event group. Returns the previous value.
    pub fn clear_bits(&self, bits_to_clear: FreeRtosEventBits) -> FreeRtosEventBits {
        unsafe { freertos_rs_event_group_clear_bits(self.handle, bits_to_clear) }
    }

    /// Gets the current event bits value.
    pub fn get_bits(&self) -> FreeRtosEventBits {
        unsafe { freertos_rs_event_group_get_bits(self.handle) }
    }

    /// Waits for specified bits to be set.
    ///
    /// # Arguments
    /// * `bits_to_wait_for` — Bit mask of bits to wait for
    /// * `clear_on_exit` — If true, clear the bits before returning
    /// * `wait_for_all` — If true, wait for ALL bits; if false, wait for ANY bit
    /// * `ticks_to_wait` — Maximum time to wait
    pub fn wait_bits(
        &self,
        bits_to_wait_for: FreeRtosEventBits,
        clear_on_exit: bool,
        wait_for_all: bool,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosEventBits {
        unsafe {
            freertos_rs_event_group_wait_bits(
                self.handle,
                bits_to_wait_for,
                if clear_on_exit { PD_TRUE } else { 0 },
                if wait_for_all { PD_TRUE } else { 0 },
                ticks_to_wait,
            )
        }
    }

    /// Synchronizes tasks at a barrier point.
    ///
    /// Sets `bits_to_set` and waits for `bits_to_wait_for`. Returns the
    /// event bits value when the synchronization condition is met.
    pub fn sync(
        &self,
        bits_to_set: FreeRtosEventBits,
        bits_to_wait_for: FreeRtosEventBits,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosEventBits {
        unsafe { freertos_rs_event_group_sync(self.handle, bits_to_set, bits_to_wait_for, ticks_to_wait) }
    }

    /// Sets bits from an ISR.
    pub fn set_bits_from_isr(
        &self,
        bits_to_set: FreeRtosEventBits,
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> bool {
        unsafe {
            freertos_rs_event_group_set_bits_from_isr(self.handle, bits_to_set, higher_priority_task_woken)
                == PD_PASS
        }
    }

    /// Gets bits from an ISR.
    pub fn get_bits_from_isr(&self) -> FreeRtosEventBits {
        unsafe { freertos_rs_event_group_get_bits_from_isr(self.handle) }
    }

    /// Clears bits from an ISR.
    ///
    /// Returns the previous value of the event group bits before clearing.
    pub fn clear_bits_from_isr(&self, bits_to_clear: FreeRtosEventBits) -> FreeRtosEventBits {
        unsafe { freertos_rs_event_group_clear_bits_from_isr(self.handle, bits_to_clear) }
    }
}

impl Drop for EventGroup {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { freertos_rs_event_group_delete(self.handle) };
        }
    }
}

unsafe impl Send for EventGroup {}
unsafe impl Sync for EventGroup {}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

const _: () = {
    const fn assert_send<T: Send>() {}
    const fn assert_sync<T: Sync>() {}
    assert_send::<EventGroup>();
    assert_sync::<EventGroup>();
};

// EventGroup is pointer-sized
const _: () = assert!(core::mem::size_of::<EventGroup>() == core::mem::size_of::<FreeRtosEventGroupHandle>());
