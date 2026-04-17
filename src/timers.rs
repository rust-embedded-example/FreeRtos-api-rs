//! `FreeRTOS` software timer module.
//!
//! Provides FFI bindings and a safe `Timer` wrapper for `FreeRTOS` software timers.
//! Timers are managed by the timer daemon task and execute their callback in
//! that context (not in ISR context).
//!
//! # Example
//!
//! ```rust,no_run
//! use freertos_api_rs::timers::Timer;
//!
//! unsafe extern "C" fn my_timer_callback(handle: freertos_api_rs::base::FreeRtosTimerHandle) {
//!     // Timer expired
//! }
//!
//! let timer = unsafe { Timer::new(
//!     b"MyTimer\0".as_ptr(),
//!     100,   // 100 ticks period
//!     true,  // auto-reload
//!     my_timer_callback,
//! ).expect("timer create failed") };
//!
//! timer.start(0);
//! ```

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosTimerHandle, FreeRtosTimerCallback,
    FreeRtosVoidPtr, FreeRtosUBaseType, FreeRtosPendedFunction, FreeRtosError,
    FreeRtosTaskHandle, PD_PASS, PD_TRUE, PD_FALSE, PORT_MAX_DELAY,
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates a software timer with dynamic allocation.
    pub fn freertos_rs_timer_create(
        timer_name: *const u8,
        timer_period: FreeRtosTickType,
        auto_reload: FreeRtosBaseType,
        timer_id: FreeRtosVoidPtr,
        callback_function: FreeRtosTimerCallback,
    ) -> FreeRtosTimerHandle;

    /// Creates a software timer with static allocation.
    pub fn freertos_rs_timer_create_static(
        timer_name: *const u8,
        timer_period: FreeRtosTickType,
        auto_reload: FreeRtosBaseType,
        timer_id: FreeRtosVoidPtr,
        callback_function: FreeRtosTimerCallback,
        timer_buffer: FreeRtosVoidPtr,
    ) -> FreeRtosTimerHandle;

    /// Deletes a software timer.
    pub fn freertos_rs_timer_delete(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER CONTROL
//===========================================================================

unsafe extern "C" {
    /// Starts a software timer.
    pub fn freertos_rs_timer_start(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Stops a software timer.
    pub fn freertos_rs_timer_stop(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Resets a software timer (restarts the period).
    pub fn freertos_rs_timer_reset(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Changes the period of a software timer.
    pub fn freertos_rs_timer_change_period(
        timer: FreeRtosTimerHandle,
        new_period: FreeRtosTickType,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Starts a software timer from an ISR.
    pub fn freertos_rs_timer_start_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Stops a software timer from an ISR.
    pub fn freertos_rs_timer_stop_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Resets a software timer from an ISR.
    pub fn freertos_rs_timer_reset_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Changes a timer's period from an ISR.
    pub fn freertos_rs_timer_change_period_from_isr(
        timer: FreeRtosTimerHandle,
        new_period: FreeRtosTickType,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Checks if a timer is currently active.
    pub fn freertos_rs_timer_is_timer_active(timer: FreeRtosTimerHandle) -> FreeRtosBaseType;

    /// Gets the handle of the timer daemon task.
    pub fn freertos_rs_timer_get_timer_daemon_task_handle() -> FreeRtosTaskHandle;

    /// Gets the period of a timer in ticks.
    pub fn freertos_rs_timer_get_period(timer: FreeRtosTimerHandle) -> FreeRtosTickType;

    /// Gets the expiry time of a timer.
    pub fn freertos_rs_timer_get_expiry_time(timer: FreeRtosTimerHandle) -> FreeRtosTickType;

    /// Gets the timer ID (user data).
    pub fn freertos_rs_timer_get_timer_id(timer: FreeRtosTimerHandle) -> FreeRtosVoidPtr;

    /// Sets the timer ID (user data).
    pub fn freertos_rs_timer_set_timer_id(timer: FreeRtosTimerHandle, new_id: FreeRtosVoidPtr);

    /// Gets the name of a timer.
    pub fn freertos_rs_timer_get_name(timer: FreeRtosTimerHandle) -> *const u8;

    /// Gets the static buffer associated with a timer.
    pub fn freertos_rs_timer_get_static_buffer(
        timer: FreeRtosTimerHandle,
        timer_buffer: *mut FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;

    /// Gets the timer number for tracing.
    pub fn freertos_rs_timer_get_timer_number(timer: FreeRtosTimerHandle) -> FreeRtosUBaseType;

    /// Sets the timer number for tracing.
    pub fn freertos_rs_timer_set_timer_number(
        timer: FreeRtosTimerHandle,
        timer_number: FreeRtosUBaseType,
    );

    /// Sets the reload mode of a timer.
    pub fn freertos_rs_timer_set_reload_mode(
        timer: FreeRtosTimerHandle,
        auto_reload: FreeRtosBaseType,
    );

    /// Gets the reload mode as `BaseType_t`.
    pub fn freertos_rs_timer_get_reload_mode(timer: FreeRtosTimerHandle) -> FreeRtosBaseType;

    /// Gets the reload mode as `UBaseType_t`.
    pub fn freertos_rs_ux_timer_get_reload_mode(timer: FreeRtosTimerHandle) -> FreeRtosUBaseType;

    /// Creates the timer daemon task (internal).
    pub fn freertos_rs_timer_create_timer_task() -> FreeRtosBaseType;

    /// Resets the timer state (internal).
    pub fn freertos_rs_timer_reset_state();

    /// Pends a function call to the timer daemon task.
    pub fn freertos_rs_timer_pend_function_call(
        function_to_pend: FreeRtosPendedFunction,
        parameter1: FreeRtosVoidPtr,
        parameter2: u32,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Pends a function call from an ISR.
    pub fn freertos_rs_timer_pend_function_call_from_isr(
        function_to_pend: FreeRtosPendedFunction,
        parameter1: FreeRtosVoidPtr,
        parameter2: u32,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// SAFE WRAPPER - TIMER
//===========================================================================

/// A `FreeRTOS` software timer with RAII cleanup.
///
/// The timer is deleted when dropped. The callback is executed in the
/// timer daemon task context (not ISR).
pub struct Timer {
    handle: FreeRtosTimerHandle,
}

impl Timer {
    /// Creates a new software timer.
    ///
    /// # Arguments
    ///
    /// * `name` — Null-terminated C string for the timer name
    /// * `period_ticks` — Timer period in ticks
    /// * `auto_reload` — `true` for auto-reload (periodic), `false` for one-shot
    /// * `callback` — Function called when the timer expires
    /// # Safety
    /// `name` must be a valid null-terminated C string.
    pub unsafe fn new(
        name: *const u8,
        period_ticks: FreeRtosTickType,
        auto_reload: bool,
        callback: FreeRtosTimerCallback,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_timer_create(
                name,
                period_ticks,
                if auto_reload { PD_TRUE } else { PD_FALSE },
                core::ptr::null_mut(),
                callback,
            )
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Starts the timer.
    pub fn start(&self, ticks_to_wait: FreeRtosTickType) -> bool {
        unsafe { freertos_rs_timer_start(self.handle, ticks_to_wait) == PD_PASS }
    }

    /// Stops the timer.
    pub fn stop(&self, ticks_to_wait: FreeRtosTickType) -> bool {
        unsafe { freertos_rs_timer_stop(self.handle, ticks_to_wait) == PD_PASS }
    }

    /// Resets the timer (restarts its period).
    pub fn reset(&self, ticks_to_wait: FreeRtosTickType) -> bool {
        unsafe { freertos_rs_timer_reset(self.handle, ticks_to_wait) == PD_PASS }
    }

    /// Changes the timer period.
    pub fn change_period(
        &self,
        new_period: FreeRtosTickType,
        ticks_to_wait: FreeRtosTickType,
    ) -> bool {
        unsafe { freertos_rs_timer_change_period(self.handle, new_period, ticks_to_wait) == PD_PASS }
    }

    /// Returns `true` if the timer is currently active.
    pub fn is_active(&self) -> bool {
        unsafe { freertos_rs_timer_is_timer_active(self.handle) == PD_TRUE }
    }

    /// Returns the timer period in ticks.
    pub fn get_period(&self) -> FreeRtosTickType {
        unsafe { freertos_rs_timer_get_period(self.handle) }
    }

    /// Returns the timer expiry time.
    pub fn get_expiry_time(&self) -> FreeRtosTickType {
        unsafe { freertos_rs_timer_get_expiry_time(self.handle) }
    }

    /// Returns the timer name.
    pub fn get_name(&self) -> *const u8 {
        unsafe { freertos_rs_timer_get_name(self.handle) }
    }

    /// Sets the reload mode.
    pub fn set_reload_mode(&self, auto_reload: bool) {
        unsafe {
            freertos_rs_timer_set_reload_mode(
                self.handle,
                if auto_reload { PD_TRUE } else { PD_FALSE },
            );
        }
    }

    /// Gets the reload mode.
    pub fn get_reload_mode(&self) -> bool {
        unsafe { freertos_rs_timer_get_reload_mode(self.handle) == PD_TRUE }
    }

    /// Starts the timer from an ISR.
    pub fn start_from_isr(&self, higher_priority_task_woken: &mut FreeRtosBaseType) -> bool {
        unsafe { freertos_rs_timer_start_from_isr(self.handle, higher_priority_task_woken) == PD_PASS }
    }

    /// Stops the timer from an ISR.
    pub fn stop_from_isr(&self, higher_priority_task_woken: &mut FreeRtosBaseType) -> bool {
        unsafe { freertos_rs_timer_stop_from_isr(self.handle, higher_priority_task_woken) == PD_PASS }
    }

    /// Resets the timer from an ISR.
    pub fn reset_from_isr(&self, higher_priority_task_woken: &mut FreeRtosBaseType) -> bool {
        unsafe { freertos_rs_timer_reset_from_isr(self.handle, higher_priority_task_woken) == PD_PASS }
    }

    /// Changes the timer period from an ISR.
    pub fn change_period_from_isr(
        &self,
        new_period: FreeRtosTickType,
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> bool {
        unsafe {
            freertos_rs_timer_change_period_from_isr(
                self.handle,
                new_period,
                higher_priority_task_woken,
            ) == PD_PASS
        }
    }

    /// Gets the timer ID (user-defined data pointer).
    pub fn get_timer_id(&self) -> FreeRtosVoidPtr {
        unsafe { freertos_rs_timer_get_timer_id(self.handle) }
    }

    /// Sets the timer ID (user-defined data pointer).
    ///
    /// # Safety
    /// `new_id` must be a valid pointer or null for the intended use.
    pub unsafe fn set_timer_id(&self, new_id: FreeRtosVoidPtr) {
        unsafe { freertos_rs_timer_set_timer_id(self.handle, new_id) };
    }

    /// Creates a software timer with static memory allocation.
    ///
    /// # Arguments
    ///
    /// * `name` — Null-terminated C string for the timer name
    /// * `period_ticks` — Timer period in ticks
    /// * `auto_reload` — `true` for auto-reload (periodic), `false` for one-shot
    /// * `callback` — Function called when the timer expires
    /// * `timer_buffer` — Pointer to a `StaticTimer_t` buffer
    ///
    /// # Safety
    /// `name` must be a valid null-terminated C string. `timer_buffer` must be
    /// properly aligned for `StaticTimer_t` and remain valid for the timer's lifetime.
    pub unsafe fn new_static(
        name: *const u8,
        period_ticks: FreeRtosTickType,
        auto_reload: bool,
        callback: FreeRtosTimerCallback,
        timer_buffer: FreeRtosVoidPtr,
    ) -> Result<Self, FreeRtosError> {
        let handle = unsafe {
            freertos_rs_timer_create_static(
                name,
                period_ticks,
                if auto_reload { PD_TRUE } else { PD_FALSE },
                core::ptr::null_mut(),
                callback,
                timer_buffer,
            )
        };
        if handle.is_null() {
            Err(FreeRtosError::OutOfMemory)
        } else {
            Ok(Self { handle })
        }
    }

    /// Gets the handle of the timer daemon task.
    pub fn get_timer_daemon_task_handle() -> FreeRtosTaskHandle {
        unsafe { freertos_rs_timer_get_timer_daemon_task_handle() }
    }

    /// Gets the reload mode as `UBaseType_t`.
    pub fn get_reload_mode_u32(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_ux_timer_get_reload_mode(self.handle) }
    }

    /// Gets the static buffer associated with this timer.
    ///
    /// Returns `true` on success.
    ///
    /// # Safety
    /// `timer_buffer` must be a valid pointer to a `*mut c_void` for output.
    pub unsafe fn get_static_buffer(&self, timer_buffer: *mut FreeRtosVoidPtr) -> bool {
        unsafe { freertos_rs_timer_get_static_buffer(self.handle, timer_buffer) != 0 }
    }

    /// Gets the timer number (trace facility).
    pub fn timer_number(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_timer_get_timer_number(self.handle) }
    }

    /// Sets the timer number (trace facility).
    pub fn set_timer_number(&self, number: FreeRtosUBaseType) {
        unsafe { freertos_rs_timer_set_timer_number(self.handle, number) }
    }

    /// Pends a function call to be executed in the timer daemon task context.
    ///
    /// Returns `true` if the function was successfully pended.
    ///
    /// # Safety
    /// `parameter1` must be a valid pointer or null for the pended function's use.
    pub unsafe fn pend_function_call(
        function: FreeRtosPendedFunction,
        parameter1: FreeRtosVoidPtr,
        parameter2: u32,
        ticks_to_wait: FreeRtosTickType,
    ) -> bool {
        unsafe {
            freertos_rs_timer_pend_function_call(function, parameter1, parameter2, ticks_to_wait) == PD_PASS
        }
    }

    /// Pends a function call from an ISR.
    ///
    /// Returns `true` if the function was successfully pended.
    ///
    /// # Safety
    /// `parameter1` must be a valid pointer or null for the pended function's use.
    pub unsafe fn pend_function_call_from_isr(
        function: FreeRtosPendedFunction,
        parameter1: FreeRtosVoidPtr,
        parameter2: u32,
        higher_priority_task_woken: &mut FreeRtosBaseType,
    ) -> bool {
        unsafe {
            freertos_rs_timer_pend_function_call_from_isr(
                function, parameter1, parameter2, higher_priority_task_woken,
            ) == PD_PASS
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { freertos_rs_timer_delete(self.handle, PORT_MAX_DELAY) };
        }
    }
}

unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

//===========================================================================
// ADDITIONAL SAFE WRAPPERS - TIMER TASK
//===========================================================================

/// Creates the timer task (daemon task).
///
/// Returns `pdPASS` on success, or an error code on failure.
/// This is typically called internally during scheduler initialization.
///
/// # Safety
/// Must only be called before the scheduler starts.
pub unsafe fn create_timer_task() -> FreeRtosBaseType {
    unsafe { freertos_rs_timer_create_timer_task() }
}

/// Resets the timer task state (internal).
///
/// # Safety
/// For kernel internal use only.
pub unsafe fn timer_reset_state() {
    unsafe { freertos_rs_timer_reset_state() };
}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

const _: () = {
    const fn assert_send<T: Send>() {}
    const fn assert_sync<T: Sync>() {}
    assert_send::<Timer>();
    assert_sync::<Timer>();
};

// Timer is pointer-sized (just a handle)
const _: () = assert!(core::mem::size_of::<Timer>() == core::mem::size_of::<FreeRtosTimerHandle>());
const _: () = assert!(core::mem::align_of::<Timer>() == core::mem::align_of::<FreeRtosTimerHandle>());
