//! FreeRTOS task management module.
//!
//! This module provides Rust FFI bindings and safe wrappers for FreeRTOS task
//! management APIs defined in `task.h`. Tasks are the fundamental unit of
//! execution in FreeRTOS — each task has its own stack, priority, and state.
//!
//! # Core Concepts
//!
//! - **Task Creation**: Use the raw FFI functions for task creation.
//!   the raw FFI functions directly.
//! - **Scheduler Control**: Start/stop the scheduler, suspend/resume all tasks.
//! - **Task Notifications**: Lightweight, fast alternative to queues for task
//!   signaling (available since FreeRTOS 8.2.0).
//! - **Critical Sections**: Use [`CriticalSection`] for RAII-managed interrupt
//!   masking.
//!
//! # Example
//!
//! ```rust,no_run
//! use freertos_api_rs::task::{Task, CriticalSection};
//!
//! extern "C" fn blink_task(_param: *mut core::ffi::c_void) {
//!     loop {
//!         // Toggle LED
//!         unsafe { freertos_api_rs::task::freertos_rs_task_delay(500); }
//!     }
//! }
//! ```

use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosTaskHandle, FreeRtosUBaseType,
    FreeRtosTaskFunction, FreeRtosConfigStackDepthType, FreeRtosStackType,
    FreeRtosStaticTask, FreeRtosVoidPtr, FreeRtosTimeOut, FreeRtosConstVoidPtr,
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TASK CREATION
//===========================================================================

unsafe extern "C" {
    /// Creates a new task with dynamically allocated memory.
    ///
    /// Wraps `xTaskCreate()`. Returns `pdPASS` on success or `errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY`.
    pub fn freertos_rs_task_create(
        task_code: FreeRtosTaskFunction,
        name: *const u8,
        stack_depth: FreeRtosConfigStackDepthType,
        parameters: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
        created_task: *mut FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Creates a new task with statically allocated memory.
    ///
    /// Wraps `xTaskCreateStatic()`. Returns the task handle, or `NULL` if buffers are invalid.
    pub fn freertos_rs_task_create_static(
        task_code: FreeRtosTaskFunction,
        name: *const u8,
        stack_depth: u32,
        parameters: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
        stack_buffer: FreeRtosStackType,
        task_buffer: FreeRtosStaticTask,
    ) -> FreeRtosTaskHandle;

    /// Creates a new MPU-restricted task.
    ///
    /// Wraps `xTaskCreateRestricted()`. Only available when `portUSING_MPU_WRAPPERS == 1`.
    pub fn freertos_rs_task_create_restricted(
        task_definition: FreeRtosConstVoidPtr,
        created_task: *mut FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Creates a new MPU-restricted task with static allocation.
    pub fn freertos_rs_task_create_restricted_static(
        task_definition: FreeRtosConstVoidPtr,
        created_task: *mut FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Creates a new MPU-restricted task with static allocation and core affinity.
    pub fn freertos_rs_task_create_restricted_static_affinity_set(
        task_definition: FreeRtosConstVoidPtr,
        core_affinity_mask: FreeRtosUBaseType,
        created_task: *mut FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Creates a new MPU-restricted task with core affinity (dynamic allocation).
    ///
    /// Wraps `xTaskCreateRestrictedAffinitySet()`.
    pub fn freertos_rs_task_create_restricted_affinity_set(
        task_definition: FreeRtosConstVoidPtr,
        core_affinity_mask: FreeRtosUBaseType,
        created_task: *mut FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Creates a new task with core affinity (SMP systems).
    ///
    /// Wraps `xTaskCreateAffinitySet()`.
    pub fn freertos_rs_task_create_affinity_set(
        task_code: FreeRtosTaskFunction,
        name: *const u8,
        stack_depth: FreeRtosConfigStackDepthType,
        parameters: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
        core_affinity_mask: FreeRtosUBaseType,
        created_task: *mut FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Creates a new task with static allocation and core affinity.
    ///
    /// Wraps `xTaskCreateStaticAffinitySet()`.
    pub fn freertos_rs_task_create_static_affinity_set(
        task_code: FreeRtosTaskFunction,
        name: *const u8,
        stack_depth: FreeRtosConfigStackDepthType,
        parameters: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
        stack_buffer: FreeRtosStackType,
        task_buffer: FreeRtosStaticTask,
        core_affinity_mask: FreeRtosUBaseType,
    ) -> FreeRtosTaskHandle;

    /// Sets the core affinity mask for a task (SMP systems).
    ///
    /// Wraps `vTaskCoreAffinitySet()`.
    pub fn freertos_rs_task_core_affinity_set(
        task: FreeRtosTaskHandle,
        core_affinity_mask: FreeRtosUBaseType,
    );

    /// Gets the core affinity mask for a task (SMP systems).
    ///
    /// Wraps `uxTaskCoreAffinityGet()`.
    pub fn freertos_rs_task_core_affinity_get(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - SCHEDULER CONTROL
//===========================================================================

unsafe extern "C" {
    /// Starts the FreeRTOS scheduler.
    ///
    /// This function does not return unless a task calls `vTaskEndScheduler()`.
    /// At least one task must have been created before calling this.
    pub fn freertos_rs_task_start_scheduler();

    /// Stops the FreeRTOS scheduler.
    ///
    /// Only available on x86-like architectures. Most embedded ports do not
    /// implement this.
    pub fn freertos_rs_task_end_scheduler();

    /// Suspends the scheduler without disabling interrupts.
    ///
    /// Critical for implementing atomic multi-step operations. Must be paired
    /// with a call to [`freertos_rs_task_resume_all`].
    pub fn freertos_rs_task_suspend_all();

    /// Resumes the scheduler after a previous suspension.
    ///
    /// Returns `pdTRUE` if a context switch is pending.
    pub fn freertos_rs_task_resume_all() -> FreeRtosBaseType;

    /// Gets the current scheduler state.
    ///
    /// Returns one of: `taskSCHEDULER_NOT_STARTED`, `taskSCHEDULER_RUNNING`,
    /// or `taskSCHEDULER_SUSPENDED`.
    pub fn freertos_rs_task_get_scheduler_state() -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TASK CONTROL
//===========================================================================

unsafe extern "C" {
    /// Delays the current task for a specified number of ticks.
    ///
    /// Wraps `vTaskDelay()`. The task is blocked for approximately `ticks_to_delay`
    /// tick periods. Use [`crate::projdefs::ms_to_ticks`] to convert milliseconds.
    pub fn freertos_rs_task_delay(ticks_to_delay: FreeRtosTickType);

    /// Delays a task until a specified absolute time.
    ///
    /// Wraps `xTaskDelayUntil()`. Use for periodic tasks with fixed frequency.
    /// Returns `pdTRUE` if the task was actually delayed.
    pub fn freertos_rs_task_delay_until(
        previous_wake_time: *mut FreeRtosTickType,
        time_increment: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Deletes a task.
    ///
    /// Wraps `vTaskDelete()`. Pass `NULL` to delete the calling task.
    /// The task's memory is freed by the idle task.
    pub fn freertos_rs_task_delete(task_to_delete: FreeRtosTaskHandle);

    /// Suspends a task.
    ///
    /// Wraps `vTaskSuspend()`. A suspended task will never run until resumed.
    pub fn freertos_rs_task_suspend(task_to_suspend: FreeRtosTaskHandle);

    /// Resumes a suspended task.
    ///
    /// Wraps `vTaskResume()`.
    pub fn freertos_rs_task_resume(task_to_resume: FreeRtosTaskHandle);

    /// Resumes a suspended task from an ISR.
    ///
    /// Wraps `xTaskResumeFromISR()`. Returns `pdTRUE` if a context switch
    /// should be requested.
    pub fn freertos_rs_task_resume_from_isr(
        task_to_resume: FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Sets the priority of a task.
    pub fn freertos_rs_task_priority_set(
        task: FreeRtosTaskHandle,
        new_priority: FreeRtosUBaseType,
    );

    /// Gets the priority of a task.
    pub fn freertos_rs_task_priority_get(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;

    /// Gets the priority of a task from an ISR.
    pub fn freertos_rs_task_priority_get_from_isr(
        task: FreeRtosTaskHandle,
    ) -> FreeRtosUBaseType;

    /// Gets the base priority of a task (before priority inheritance).
    pub fn freertos_rs_task_base_priority_get(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;

    /// Gets the base priority of a task from an ISR.
    pub fn freertos_rs_task_base_priority_get_from_isr(
        task: FreeRtosTaskHandle,
    ) -> FreeRtosUBaseType;

    /// Aborts the delay of a task, causing it to re-enter the ready state.
    pub fn freertos_rs_task_abort_delay(task: FreeRtosTaskHandle) -> FreeRtosBaseType;

    /// Disables preemption for a task (cooperative scheduling only).
    pub fn freertos_rs_task_preemption_disable(task: FreeRtosTaskHandle);

    /// Enables preemption for a task (cooperative scheduling only).
    pub fn freertos_rs_task_preemption_enable(task: FreeRtosTaskHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TICK FUNCTIONS
//===========================================================================

unsafe extern "C" {
    /// Gets the current tick count.
    pub fn freertos_rs_task_get_tick_count() -> FreeRtosTickType;

    /// Gets the current tick count from an ISR.
    pub fn freertos_rs_task_get_tick_count_from_isr() -> FreeRtosTickType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TASK NOTIFICATIONS
//===========================================================================

unsafe extern "C" {
    /// Sends a notification to a task (index 0).
    ///
    /// Wraps `xTaskNotify()`. This is a lightweight alternative to binary
    /// semaphores, queues, and event groups.
    pub fn freertos_rs_task_notify(
        task_to_notify: FreeRtosTaskHandle,
        value: u32,
        action: u32,
    ) -> FreeRtosBaseType;

    /// Sends a notification to a task from an ISR.
    pub fn freertos_rs_task_notify_from_isr(
        task_to_notify: FreeRtosTaskHandle,
        value: u32,
        action: u32,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Waits for a notification on index 0.
    pub fn freertos_rs_task_notify_wait(
        bits_to_clear_on_entry: u32,
        bits_to_clear_on_exit: u32,
        notification_value: *mut u32,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Increments a task's notification value (index 0).
    pub fn freertos_rs_task_notify_give(task_to_notify: FreeRtosTaskHandle) -> FreeRtosBaseType;

    /// Takes (decrements or clears) a task notification value.
    pub fn freertos_rs_task_notify_take(
        clear_count_on_exit: FreeRtosBaseType,
        ticks_to_wait: FreeRtosTickType,
    ) -> u32;

    /// Generic notification take with explicit index.
    ///
    /// Wraps `ulTaskGenericNotifyTake()`. The indexed version of
    /// [`freertos_rs_task_notify_take`] which operates on index 0 only.
    pub fn freertos_rs_task_generic_notify_take(
        index_to_wait_on: FreeRtosUBaseType,
        clear_count_on_exit: FreeRtosBaseType,
        ticks_to_wait: FreeRtosTickType,
    ) -> u32;

    /// Generic task notification with explicit index.
    pub fn freertos_rs_task_generic_notify(
        task_to_notify: FreeRtosTaskHandle,
        index_to_notify: FreeRtosUBaseType,
        value: u32,
        action: u32,
        previous_notification_value: *mut u32,
    ) -> FreeRtosBaseType;

    /// Generic task notification from ISR with explicit index.
    pub fn freertos_rs_task_generic_notify_from_isr(
        task_to_notify: FreeRtosTaskHandle,
        index_to_notify: FreeRtosUBaseType,
        value: u32,
        action: u32,
        previous_notification_value: *mut u32,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    ) -> FreeRtosBaseType;

    /// Gives a notification (increment) from ISR with explicit index.
    pub fn freertos_rs_task_generic_notify_give_from_isr(
        task_to_notify: FreeRtosTaskHandle,
        index_to_notify: FreeRtosUBaseType,
        higher_priority_task_woken: *mut FreeRtosBaseType,
    );

    /// Generic notification wait with explicit index.
    pub fn freertos_rs_task_generic_notify_wait(
        index_to_wait_on: FreeRtosUBaseType,
        bits_to_clear_on_entry: u32,
        bits_to_clear_on_exit: u32,
        notification_value: *mut u32,
        ticks_to_wait: FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Clears the notification state (pending/not-pending) for a given index.
    pub fn freertos_rs_task_generic_notify_state_clear(
        task: FreeRtosTaskHandle,
        index_to_clear: FreeRtosUBaseType,
    ) -> FreeRtosBaseType;

    /// Clears specific bits in a task notification value.
    pub fn freertos_rs_task_generic_notify_value_clear(
        task: FreeRtosTaskHandle,
        index_to_clear: FreeRtosUBaseType,
        bits_to_clear: u32,
    ) -> u32;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TASK UTILITIES
//===========================================================================

unsafe extern "C" {
    /// Gets the application task tag.
    pub fn freertos_rs_task_get_application_task_tag(task: FreeRtosTaskHandle) -> FreeRtosVoidPtr;

    /// Gets the application task tag from ISR context.
    ///
    /// Wraps `xTaskGetApplicationTaskTagFromISR()`. ISR-safe version of
    /// [`freertos_rs_task_get_application_task_tag`].
    pub fn freertos_rs_task_get_application_task_tag_from_isr(task: FreeRtosTaskHandle) -> FreeRtosVoidPtr;

    /// Sets the application task tag.
    pub fn freertos_rs_task_set_application_task_tag(task: FreeRtosTaskHandle, tag_value: FreeRtosVoidPtr);

    /// Calls the application task hook function.
    pub fn freertos_rs_task_call_application_task_hook(
        task: FreeRtosTaskHandle,
        parameter: FreeRtosVoidPtr,
    ) -> FreeRtosBaseType;

    /// Gets the name of a task.
    pub fn freertos_rs_task_get_name(task: FreeRtosTaskHandle) -> *const u8;

    /// Gets a task handle by name.
    pub fn freertos_rs_task_get_handle(task_name: *const u8) -> FreeRtosTaskHandle;

    /// Gets the handle of the currently running task.
    pub fn freertos_rs_task_get_current_task_handle() -> FreeRtosTaskHandle;

    /// Gets the handle of the idle task.
    pub fn freertos_rs_task_get_idle_task_handle() -> FreeRtosTaskHandle;

    /// Gets the high water mark of a task's stack (in words).
    pub fn freertos_rs_task_get_stack_high_water_mark(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;

    /// Gets the high water mark of a task's stack (configSTACK_DEPTH_TYPE).
    pub fn freertos_rs_task_get_stack_high_water_mark2(task: FreeRtosTaskHandle) -> FreeRtosConfigStackDepthType;

    /// Gets the static buffers associated with a task.
    pub fn freertos_rs_task_get_static_buffers(
        task: FreeRtosTaskHandle,
        stack_buffer: *mut FreeRtosStackType,
        task_buffer: *mut FreeRtosStaticTask,
    ) -> FreeRtosBaseType;

    /// Gets the run time counter for a task.
    pub fn freertos_rs_task_get_run_time_counter(task: FreeRtosTaskHandle) -> u32;

    /// Gets the run time percentage for a task.
    pub fn freertos_rs_task_get_run_time_percent(task: FreeRtosTaskHandle) -> u32;

    /// Gets the run time counter of the idle task.
    ///
    /// Wraps `ulTaskGetIdleRunTimeCounter()`. Available when
    /// `configGENERATE_RUN_TIME_STATS` is enabled.
    pub fn freertos_rs_task_get_idle_run_time_counter() -> u32;

    /// Gets the percentage of CPU time used by the idle task.
    ///
    /// Wraps `ulTaskGetIdleRunTimePercent()`. Available when
    /// `configGENERATE_RUN_TIME_STATS` is enabled.
    pub fn freertos_rs_task_get_idle_run_time_percent() -> u32;

    /// Gets the state of a task as a `u32`.
    pub fn freertos_rs_task_get_state(task: FreeRtosTaskHandle) -> u32;

    /// Generates a human-readable table of task states.
    pub fn freertos_rs_task_list(write_buffer: *mut u8);

    /// Generates a human-readable table of run time statistics.
    pub fn freertos_rs_task_get_run_time_stats(write_buffer: *mut u8);

    /// Gets the number of tasks in the system.
    pub fn freertos_rs_task_get_number_of_tasks() -> FreeRtosUBaseType;

    /// Gets detailed task information for all tasks.
    pub fn freertos_rs_task_get_system_state(
        task_status_array: FreeRtosVoidPtr,
        array_size: FreeRtosUBaseType,
        total_run_time: *mut u32,
    ) -> FreeRtosUBaseType;

    /// Gets information about a specific task.
    pub fn freertos_rs_task_get_info(
        task: FreeRtosTaskHandle,
        task_status: FreeRtosVoidPtr,
        get_free_stack_space: FreeRtosBaseType,
        state: u32,
    );

    /// Sets a thread-local storage pointer.
    pub fn freertos_rs_task_set_thread_local_storage_pointer(
        task: FreeRtosTaskHandle,
        index: FreeRtosBaseType,
        value: FreeRtosVoidPtr,
    );

    /// Gets a thread-local storage pointer.
    pub fn freertos_rs_task_get_thread_local_storage_pointer(
        task: FreeRtosTaskHandle,
        index: FreeRtosBaseType,
    ) -> FreeRtosVoidPtr;

    /// Sets the timeout state for bounded wait loops.
    pub fn freertos_rs_task_set_time_out_state(time_out: *mut FreeRtosTimeOut);

    /// Checks if a timeout has occurred.
    pub fn freertos_rs_task_check_for_time_out(
        time_out: *mut FreeRtosTimeOut,
        ticks_to_wait: *mut FreeRtosTickType,
    ) -> FreeRtosBaseType;

    /// Catches up ticks after exiting a low-power mode.
    pub fn freertos_rs_task_catch_up_ticks(ticks_to_catch_up: FreeRtosTickType) -> FreeRtosBaseType;

    /// Resets the task state (internal).
    pub fn freertos_rs_task_reset_state();

    /// Generates a task list with buffer length safety.
    pub fn freertos_rs_task_list_tasks(write_buffer: *mut u8, buffer_length: usize);

    /// Generates run time statistics with buffer length safety.
    pub fn freertos_rs_task_get_run_time_statistics(write_buffer: *mut u8, buffer_length: usize);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CRITICAL SECTIONS
//===========================================================================

unsafe extern "C" {
    /// Enters a critical section (disables interrupts).
    pub fn freertos_rs_task_enter_critical();

    /// Exits a critical section (re-enables interrupts).
    pub fn freertos_rs_task_exit_critical();

    /// Enters a critical section from ISR, returns previous interrupt state.
    pub fn freertos_rs_task_enter_critical_from_isr() -> FreeRtosUBaseType;

    /// Exits a critical section from ISR, restores previous interrupt state.
    pub fn freertos_rs_task_exit_critical_from_isr(saved_interrupt_status: FreeRtosUBaseType);

    /// Disables all maskable interrupts (port-level).
    pub fn freertos_rs_task_disable_interrupts();

    /// Enables all maskable interrupts (port-level).
    pub fn freertos_rs_task_enable_interrupts();

    /// Allocates MPU regions to a task.
    pub fn freertos_rs_task_allocate_mpu_regions(
        task_to_modify: FreeRtosTaskHandle,
        regions: FreeRtosConstVoidPtr,
    );

    /// Increments the tick count (called by the tick ISR).
    pub fn freertos_rs_task_increment_tick() -> FreeRtosBaseType;

    /// Steps the tick count forward (tickless idle support).
    pub fn freertos_rs_task_step_tick(ticks_to_jump: FreeRtosTickType);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - ADDITIONAL INTERNAL API
//===========================================================================
// These functions are used by the C shim / kernel internals. They are not
// called from Rust code but must remain linked. Suppress dead_code warning.
#[allow(dead_code)]
unsafe extern "C" {
    /// Indicates a context switch was missed (internal).
    pub(crate) fn freertos_rs_task_missed_yield();

    /// Inherits priority from a mutex-holding task.
    pub(crate) fn freertos_rs_task_priority_inherit(
        mutex_holder: FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Disinherits priority when releasing a mutex.
    pub(crate) fn freertos_rs_task_priority_disinherit(
        mutex_holder: FreeRtosTaskHandle,
    ) -> FreeRtosBaseType;

    /// Disinherits priority after a mutex timeout.
    pub(crate) fn freertos_rs_task_priority_disinherit_after_timeout(
        mutex_holder: FreeRtosTaskHandle,
        highest_priority_waiting: FreeRtosUBaseType,
    );

    /// Removes a task from an event list (internal).
    pub(crate) fn freertos_rs_task_remove_from_event_list(
        event_list: FreeRtosConstVoidPtr,
    ) -> FreeRtosBaseType;

    /// Resets the event item value of the current task (internal).
    pub(crate) fn freertos_rs_task_reset_event_item_value() -> FreeRtosUBaseType;

    /// Increments the mutex held count (internal).
    pub(crate) fn freertos_rs_task_increment_mutex_held_count() -> FreeRtosVoidPtr;

    /// Gets the current task handle for a specific core (SMP only).
    pub fn freertos_rs_task_get_current_task_handle_for_core(
        core_id: FreeRtosBaseType,
    ) -> FreeRtosTaskHandle;

    /// Gets the idle task handle for a specific core (SMP only).
    pub fn freertos_rs_task_get_idle_task_handle_for_core(
        core_id: FreeRtosBaseType,
    ) -> FreeRtosTaskHandle;

    /// Gets the task number used for tracing.
    pub fn freertos_rs_task_get_task_number(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;

    /// Sets the task number used for tracing.
    pub fn freertos_rs_task_set_task_number(
        task: FreeRtosTaskHandle,
        task_number: FreeRtosUBaseType,
    );

    /// Checks if the system can enter sleep mode (tickless idle support).
    pub fn freertos_rs_task_confirm_sleep_mode_status() -> FreeRtosBaseType;

    /// Gets the static memory for the idle task (static allocation).
    ///
    /// Wraps `vTaskGetIdleTaskMemory()`. Used when `configSUPPORT_STATIC_ALLOCATION == 1`
    /// to provide the idle task's TCB and stack buffers.
    pub fn freertos_rs_task_get_idle_task_memory(
        tcb_buffer: *mut FreeRtosStaticTask,
        stack_buffer: *mut FreeRtosStackType,
        stack_size: *mut u32,
    );

    /// Gets the static memory for the passive idle task (SMP, static allocation).
    ///
    /// Wraps `vTaskGetPassiveIdleTaskMemory()`. Only available when
    /// `configNUMBER_OF_CORES > 1` and `configSUPPORT_STATIC_ALLOCATION == 1`.
    pub fn freertos_rs_task_get_passive_idle_task_memory(
        tcb_buffer: *mut FreeRtosStaticTask,
        stack_buffer: *mut FreeRtosStackType,
        stack_size: *mut u32,
        core_id: FreeRtosBaseType,
    );
}

// Note: freertos_rs_port_yield() is declared in portable.rs to keep all
// port-level functions together. Use crate::portable::freertos_rs_port_yield.

//===========================================================================
// SAFE WRAPPER - CRITICAL SECTION RAII GUARD
//===========================================================================

/// RAII guard for a FreeRTOS critical section.
///
/// Disables interrupts on creation and re-enables them on drop. Use this
/// instead of manually calling `taskENTER_CRITICAL` / `taskEXIT_CRITICAL`.
///
/// # Example
///
/// ```rust,no_run
/// use freertos_api_rs::task::CriticalSection;
///
/// {
///     let _cs = CriticalSection::enter();
///     // Interrupts are disabled in this scope
///     // Access shared resources safely
/// } // Interrupts re-enabled here when _cs is dropped
/// ```
pub struct CriticalSection {
    _private: (), // Prevent construction outside of `enter()`
}

impl CriticalSection {
    /// Enters a critical section, disabling interrupts.
    ///
    /// Interrupts will be re-enabled when the returned guard is dropped.
    pub fn enter() -> Self {
        unsafe { freertos_rs_task_enter_critical() };
        Self { _private: () }
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        unsafe { freertos_rs_task_exit_critical() };
    }
}

// Safety: CriticalSection disables interrupts; it's Send because it can be
// created on any thread but should not be shared across threads.
unsafe impl Send for CriticalSection {}

/// RAII guard for a critical section entered from an ISR context.
///
/// Saves the interrupt state on creation and restores it on drop.
pub struct CriticalSectionFromIsr {
    saved_state: FreeRtosUBaseType,
}

impl CriticalSectionFromIsr {
    /// Enters a critical section from an ISR.
    pub fn enter() -> Self {
        let saved_state = unsafe { freertos_rs_task_enter_critical_from_isr() };
        Self { saved_state }
    }
}

impl Drop for CriticalSectionFromIsr {
    fn drop(&mut self) {
        unsafe { freertos_rs_task_exit_critical_from_isr(self.saved_state) };
    }
}

//===========================================================================
// SAFE WRAPPER - PREEMPTION GUARD RAII
//===========================================================================

/// RAII guard that disables preemption for a specific task.
///
/// Calls `vTaskPreemptionDisable()` on creation and `vTaskPreemptionEnable()`
/// on drop. This is used for cooperative scheduling within a preemptive system.
///
/// # Example
///
/// ```rust,no_run
/// use freertos_api_rs::task::PreemptionGuard;
/// use freertos_api_rs::task::freertos_rs_task_get_current_task_handle;
///
/// let task = unsafe { freertos_rs_task_get_current_task_handle() };
/// {
///     let _guard = PreemptionGuard::disable(task);
///     // This task will not be preempted in this scope
/// } // Preemption re-enabled on drop
/// ```
pub struct PreemptionGuard {
    task: FreeRtosTaskHandle,
}

impl PreemptionGuard {
    /// Disables preemption for the given task.
    ///
    /// Preemption will be re-enabled when the guard is dropped.
    ///
    /// # Safety
    /// `task` must be a valid, non-null task handle.
    pub unsafe fn disable(task: FreeRtosTaskHandle) -> Self {
        unsafe { freertos_rs_task_preemption_disable(task) };
        Self { task }
    }
}

impl Drop for PreemptionGuard {
    fn drop(&mut self) {
        unsafe { freertos_rs_task_preemption_enable(self.task) };
    }
}

// Safety: PreemptionGuard controls scheduling state, not shared data.
unsafe impl Send for PreemptionGuard {}

//===========================================================================
// SAFE WRAPPER - TASK
//===========================================================================

/// A spawned FreeRTOS task with RAII-managed lifetime.
///
/// Wraps a FreeRTOS task handle. When dropped, the task is deleted via
/// `vTaskDelete`. The handle may be null if the task was not created
/// successfully.
///
/// # Example
///
/// ```rust,no_run
/// use freertos_api_rs::task::Task;
/// use freertos_api_rs::base::TSK_IDLE_PRIORITY;
///
/// extern "C" fn my_task(_param: *mut core::ffi::c_void) {
///     loop {
///         unsafe { freertos_api_rs::task::freertos_rs_task_delay(100); }
///     }
/// }
///
/// let task = Task::spawn(
///     b"my_task\0".as_ptr(),
///     128,
///     my_task,
///     core::ptr::null_mut(),
///     TSK_IDLE_PRIORITY + 1,
/// ).expect("Failed to create task");
///
/// // task is deleted when `task` goes out of scope
/// ```
pub struct Task {
    handle: FreeRtosTaskHandle,
    /// If true, `vTaskDelete` is called on drop. Set to false for tasks
    /// that should outlive the handle (e.g., the current task).
    owns_task: bool,
}

impl Task {
    /// Spawns a new FreeRTOS task with dynamic memory allocation.
    ///
    /// # Safety
    /// `name` must be a valid null-terminated C string. `param` must be valid
    /// for the lifetime of the task or null.
    pub unsafe fn spawn(
        name: *const u8,
        stack_depth: FreeRtosConfigStackDepthType,
        entry: FreeRtosTaskFunction,
        param: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
    ) -> Result<Self, crate::base::FreeRtosError> {
        let mut handle: FreeRtosTaskHandle = core::ptr::null();
        let result = unsafe {
            freertos_rs_task_create(entry, name, stack_depth, param, priority, &mut handle)
        };
        if result == crate::base::PD_PASS && !handle.is_null() {
            Ok(Self { handle, owns_task: true })
        } else {
            Err(crate::base::FreeRtosError::OutOfMemory)
        }
    }

    /// Creates a `Task` from an existing handle without taking ownership.
    ///
    /// The task will NOT be deleted when this `Task` is dropped.
    pub fn from_handle(handle: FreeRtosTaskHandle) -> Self {
        Self { handle, owns_task: false }
    }

    /// Creates a `Task` from an existing handle, taking ownership.
    ///
    /// The task WILL be deleted when this `Task` is dropped.
    pub fn from_handle_owned(handle: FreeRtosTaskHandle) -> Self {
        Self { handle, owns_task: true }
    }

    /// Returns the raw task handle.
    pub fn handle(&self) -> FreeRtosTaskHandle {
        self.handle
    }

    /// Suspends this task.
    pub fn suspend(&self) {
        unsafe { freertos_rs_task_suspend(self.handle) }
    }

    /// Resumes this task.
    pub fn resume(&self) {
        unsafe { freertos_rs_task_resume(self.handle) }
    }

    /// Resumes this task from ISR context.
    /// Returns `true` if a context switch should be requested.
    pub fn resume_from_isr(&self) -> bool {
        unsafe { freertos_rs_task_resume_from_isr(self.handle) != 0 }
    }

    /// Gets the task priority.
    pub fn priority(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_task_priority_get(self.handle) }
    }

    /// Sets the task priority.
    pub fn set_priority(&self, priority: FreeRtosUBaseType) {
        unsafe { freertos_rs_task_priority_set(self.handle, priority) }
    }

    /// Gets the task name.
    pub fn name(&self) -> *const u8 {
        unsafe { freertos_rs_task_get_name(self.handle) }
    }

    /// Gets the stack high water mark (minimum free stack space).
    pub fn stack_high_water_mark(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_task_get_stack_high_water_mark(self.handle) }
    }

    /// Gets the current task state.
    pub fn state(&self) -> crate::base::FreeRtosTaskState {
        let s = unsafe { freertos_rs_task_get_state(self.handle) };
        match s {
            0 => crate::base::FreeRtosTaskState::Running,
            1 => crate::base::FreeRtosTaskState::Ready,
            2 => crate::base::FreeRtosTaskState::Blocked,
            3 => crate::base::FreeRtosTaskState::Suspended,
            4 => crate::base::FreeRtosTaskState::Deleted,
            _ => crate::base::FreeRtosTaskState::Running,
        }
    }

    /// Notifies this task with a value and action.
    /// Returns the previous notification value (before the action).
    pub fn notify(&self, value: u32, action: crate::base::FreeRtosNotifyAction) -> FreeRtosBaseType {
        unsafe { freertos_rs_task_notify(self.handle, value, action as u32) }
    }

    /// Gives a notification to this task (increments the notification value).
    pub fn notify_give(&self) -> FreeRtosBaseType {
        unsafe { freertos_rs_task_notify_give(self.handle) }
    }

    /// Aborts any delay on this task, making it ready to run.
    pub fn abort_delay(&self) -> FreeRtosBaseType {
        unsafe { freertos_rs_task_abort_delay(self.handle) }
    }

    /// Disables further automatic deletion on drop.
    /// Use this if you want the task to outlive this handle.
    pub fn detach(&mut self) {
        self.owns_task = false;
    }

    /// Gets the base priority (before priority inheritance).
    pub fn base_priority(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_task_base_priority_get(self.handle) }
    }

    /// Gets the base priority from ISR context.
    pub fn base_priority_from_isr(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_task_base_priority_get_from_isr(self.handle) }
    }

    /// Gets the priority from ISR context.
    pub fn priority_from_isr(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_task_priority_get_from_isr(self.handle) }
    }

    /// Sets the core affinity mask (SMP only).
    pub fn set_core_affinity(&self, mask: FreeRtosUBaseType) {
        unsafe { freertos_rs_task_core_affinity_set(self.handle, mask) }
    }

    /// Gets the core affinity mask (SMP only).
    pub fn core_affinity(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_task_core_affinity_get(self.handle) }
    }

    /// Gets the static buffers for this task (static allocation only).
    ///
    /// Returns `true` on success.
    ///
    /// # Safety
    /// The output pointers must be valid and properly aligned.
    pub unsafe fn get_static_buffers(
        &self,
        stack_buffer: *mut FreeRtosStackType,
        task_buffer: *mut FreeRtosStaticTask,
    ) -> bool {
        unsafe { freertos_rs_task_get_static_buffers(self.handle, stack_buffer, task_buffer) != 0 }
    }

    /// Gets the stack high water mark as `configSTACK_DEPTH_TYPE`.
    pub fn stack_high_water_mark2(&self) -> FreeRtosConfigStackDepthType {
        unsafe { freertos_rs_task_get_stack_high_water_mark2(self.handle) }
    }

    /// Sends an indexed notification to this task.
    ///
    /// # Safety
    /// `previous_value` must be a valid pointer or null.
    pub unsafe fn notify_indexed(
        &self,
        index: FreeRtosUBaseType,
        value: u32,
        action: crate::base::FreeRtosNotifyAction,
        previous_value: *mut u32,
    ) -> FreeRtosBaseType {
        unsafe {
            freertos_rs_task_generic_notify(self.handle, index, value, action as u32, previous_value)
        }
    }

    /// Gives an indexed notification (increment) to this task.
    pub fn notify_give_indexed(&self, index: FreeRtosUBaseType) -> FreeRtosBaseType {
        // Use generic notify with Increment action
        unsafe {
            freertos_rs_task_generic_notify(
                self.handle,
                index,
                0,
                crate::base::FreeRtosNotifyAction::Increment as u32,
                core::ptr::null_mut(),
            )
        }
    }

    /// Gets the task number (trace facility).
    pub fn task_number(&self) -> FreeRtosUBaseType {
        unsafe { freertos_rs_task_get_task_number(self.handle) }
    }

    /// Sets the task number (trace facility).
    pub fn set_task_number(&self, number: FreeRtosUBaseType) {
        unsafe { freertos_rs_task_set_task_number(self.handle, number) }
    }

    /// Sets a thread-local storage pointer for this task.
    ///
    /// # Safety
    /// `value` must be a valid pointer or null for the intended use.
    pub unsafe fn set_tls_pointer(&self, index: FreeRtosBaseType, value: FreeRtosVoidPtr) {
        unsafe { freertos_rs_task_set_thread_local_storage_pointer(self.handle, index, value) }
    }

    /// Gets a thread-local storage pointer for this task.
    pub fn get_tls_pointer(&self, index: FreeRtosBaseType) -> FreeRtosVoidPtr {
        unsafe { freertos_rs_task_get_thread_local_storage_pointer(self.handle, index) }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        if self.owns_task && !self.handle.is_null() {
            unsafe { freertos_rs_task_delete(self.handle) };
        }
    }
}

// Safety: Task handles can be sent between threads/cores.
unsafe impl Send for Task {}

//===========================================================================
// SAFE FUNCTIONS - TASK UTILITIES (no handle needed)
//===========================================================================

/// Delays the current task for the specified number of ticks.
pub fn delay(ticks: FreeRtosTickType) {
    unsafe { freertos_rs_task_delay(ticks) }
}

/// Returns the current tick count.
pub fn get_tick_count() -> FreeRtosTickType {
    unsafe { freertos_rs_task_get_tick_count() }
}

/// Returns the current tick count (ISR-safe).
pub fn get_tick_count_from_isr() -> FreeRtosTickType {
    unsafe { freertos_rs_task_get_tick_count_from_isr() }
}

/// Returns the handle of the currently running task.
pub fn get_current_task_handle() -> FreeRtosTaskHandle {
    unsafe { freertos_rs_task_get_current_task_handle() }
}

/// Returns the idle task handle.
pub fn get_idle_task_handle() -> FreeRtosTaskHandle {
    unsafe { freertos_rs_task_get_idle_task_handle() }
}

/// Returns the total number of tasks.
pub fn get_number_of_tasks() -> FreeRtosUBaseType {
    unsafe { freertos_rs_task_get_number_of_tasks() }
}

/// Starts the FreeRTOS scheduler.
pub fn start_scheduler() {
    unsafe { freertos_rs_task_start_scheduler() }
}

/// Ends the FreeRTOS scheduler.
pub fn end_scheduler() {
    unsafe { freertos_rs_task_end_scheduler() }
}

/// Suspends the scheduler (without disabling interrupts).
pub fn suspend_all() {
    unsafe { freertos_rs_task_suspend_all() }
}

/// Resumes the scheduler. Returns `true` if a context switch is needed.
pub fn resume_all() -> bool {
    unsafe { freertos_rs_task_resume_all() != 0 }
}

/// Waits for a task notification.
///
/// Returns the notification value. Clears bits as specified.
pub fn notify_wait(
    bits_to_clear_on_entry: u32,
    bits_to_clear_on_exit: u32,
    ticks_to_wait: FreeRtosTickType,
) -> Option<u32> {
    let mut value: u32 = 0;
    let result = unsafe {
        freertos_rs_task_notify_wait(
            bits_to_clear_on_entry,
            bits_to_clear_on_exit,
            &mut value,
            ticks_to_wait,
        )
    };
    if result == crate::base::PD_TRUE {
        Some(value)
    } else {
        None
    }
}

/// Takes a notification (ISR-safe).
pub fn notify_take(clear_on_exit: bool, ticks_to_wait: FreeRtosTickType) -> u32 {
    unsafe { freertos_rs_task_notify_take(if clear_on_exit { 1 } else { 0 }, ticks_to_wait) }
}

/// Delays the current task until an absolute wake time (for periodic tasks).
///
/// Returns `true` if the task was actually delayed.
/// Use with `get_tick_count()` to compute the next wake time.
pub fn delay_until(previous_wake_time: &mut FreeRtosTickType, time_increment: FreeRtosTickType) -> bool {
    unsafe { freertos_rs_task_delay_until(previous_wake_time, time_increment) != 0 }
}

/// Gets the scheduler state.
///
/// Returns one of: `TASK_SCHEDULER_NOT_STARTED`, `TASK_SCHEDULER_RUNNING`,
/// or `TASK_SCHEDULER_SUSPENDED` (from `projdefs` module).
pub fn get_scheduler_state() -> FreeRtosBaseType {
    unsafe { freertos_rs_task_get_scheduler_state() }
}

/// Gets the handle of a task by its name.
///
/// Returns `None` if no task with that name exists.
///
/// # Safety
/// `task_name` must be a valid null-terminated C string.
pub unsafe fn get_handle(task_name: *const u8) -> Option<FreeRtosTaskHandle> {
    let handle = unsafe { freertos_rs_task_get_handle(task_name) };
    if handle.is_null() { None } else { Some(handle) }
}

/// Gets the high water mark of the current task's stack.
pub fn get_current_stack_high_water_mark() -> FreeRtosUBaseType {
    unsafe { freertos_rs_task_get_stack_high_water_mark(core::ptr::null()) }
}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

// Send trait bounds
const _: () = {
    const fn assert_send<T: Send>() {}
    assert_send::<CriticalSection>();
    assert_send::<CriticalSectionFromIsr>();
    assert_send::<PreemptionGuard>();
};

// Constants
const _: () = assert!(crate::base::PD_PASS == 1);
const _: () = assert!(crate::base::PD_TRUE == 1);
const _: () = assert!(crate::base::PD_FALSE == 0);
const _: () = assert!(crate::base::PORT_MAX_DELAY == 0xFFFFFFFF);
