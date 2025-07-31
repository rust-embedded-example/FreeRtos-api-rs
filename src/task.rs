use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosTaskHandle, FreeRtosUBaseType,
    FreeRtosTaskFunction, FreeRtosConfigStackDepthType, FreeRtosStackType, FreeRtosStaticTask,
    FreeRtosVoidPtr, FreeRtosTimeOut
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TASK CREATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTaskCreate()
    /// Creates a new task and adds it to the list of tasks that are ready to run
    pub fn freertos_rs_task_create(
        task_code: FreeRtosTaskFunction,
        name: *const u8,
        stack_depth: FreeRtosConfigStackDepthType,
        parameters: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
        created_task: *mut FreeRtosTaskHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTaskCreateStatic()
    /// Creates a new task using statically allocated memory
    pub fn freertos_rs_task_create_static(
        task_code: FreeRtosTaskFunction,
        name: *const u8,
        stack_depth: u32,
        parameters: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
        stack_buffer: FreeRtosStackType,
        task_buffer: FreeRtosStaticTask
    ) -> FreeRtosTaskHandle;
    
    /// Wrapper for xTaskCreateRestricted()
    /// Creates a new restricted task for MPU systems
    pub fn freertos_rs_task_create_restricted(
        task_definition: *const FreeRtosVoidPtr,
        created_task: *mut FreeRtosTaskHandle
    ) -> FreeRtosBaseType;

    /// Wrapper for xTaskCreateRestrictedStatic()
    /// Creates a new restricted task using static allocation
    pub fn freertos_rs_task_create_restricted_static(
        task_definition: *const FreeRtosVoidPtr,
        created_task: *mut FreeRtosTaskHandle
    ) -> FreeRtosBaseType;

    /// Wrapper for xTaskCreateRestrictedStaticAffinitySet()
    /// Creates a new restricted task with affinity using static allocation
    pub fn freertos_rs_task_create_restricted_static_affinity_set(
        task_definition: *const FreeRtosVoidPtr,
        core_affinity_mask: FreeRtosUBaseType,
        created_task: *mut FreeRtosTaskHandle
    ) -> FreeRtosBaseType;

    /// Wrapper for xTaskCreateAffinitySet()
    /// Creates a new task with core affinity (multi-core systems)
    pub fn freertos_rs_task_create_affinity_set(
        task_code: FreeRtosTaskFunction,
        name: *const u8,
        stack_depth: FreeRtosConfigStackDepthType,
        parameters: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
        core_affinity_mask: FreeRtosUBaseType,
        created_task: *mut FreeRtosTaskHandle
    ) -> FreeRtosBaseType;

    /// Wrapper for xTaskCreateStaticAffinitySet()
    /// Creates a new task with core affinity using static allocation
    pub fn freertos_rs_task_create_static_affinity_set(
        task_code: FreeRtosTaskFunction,
        name: *const u8,
        stack_depth: FreeRtosConfigStackDepthType,
        parameters: FreeRtosVoidPtr,
        priority: FreeRtosUBaseType,
        stack_buffer: FreeRtosStackType,
        task_buffer: FreeRtosStaticTask,
        core_affinity_mask: FreeRtosUBaseType
    ) -> FreeRtosTaskHandle;

    /// Wrapper for vTaskCoreAffinitySet()
    /// Sets the core affinity of a task
    pub fn freertos_rs_task_core_affinity_set(
        task: FreeRtosTaskHandle,
        core_affinity_mask: FreeRtosUBaseType
    );

    /// Wrapper for uxTaskCoreAffinityGet()
    /// Gets the core affinity of a task
    pub fn freertos_rs_task_core_affinity_get(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - SCHEDULER CONTROL
//===========================================================================

unsafe extern "C" {
    /// Wrapper for vTaskStartScheduler()
    /// Starts the FreeRTOS scheduler
    pub fn freertos_rs_task_start_scheduler();
    
    /// Wrapper for vTaskEndScheduler()
    /// Ends the FreeRTOS scheduler
    pub fn freertos_rs_task_end_scheduler();
    
    /// Wrapper for vTaskSuspendAll()
    /// Suspends the scheduler without disabling interrupts
    pub fn freertos_rs_task_suspend_all();
    
    /// Wrapper for xTaskResumeAll()
    /// Resumes the scheduler after suspension
    pub fn freertos_rs_task_resume_all() -> FreeRtosBaseType;
    
    /// Wrapper for xTaskGetSchedulerState()
    /// Gets the current scheduler state
    pub fn freertos_rs_task_get_scheduler_state() -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TASK CONTROL
//===========================================================================

unsafe extern "C" {
    /// Wrapper for vTaskDelay()
    /// Delays the current task for a specified number of ticks
    pub fn freertos_rs_task_delay(ticks_to_delay: FreeRtosTickType);
    
    /// Wrapper for xTaskDelayUntil()
    /// Delays a task until a specified time
    pub fn freertos_rs_task_delay_until(
        previous_wake_time: *mut FreeRtosTickType,
        time_increment: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for vTaskDelete()
    /// Deletes a task
    pub fn freertos_rs_task_delete(task_to_delete: FreeRtosTaskHandle);
    
    /// Wrapper for vTaskSuspend()
    /// Suspends a task
    pub fn freertos_rs_task_suspend(task_to_suspend: FreeRtosTaskHandle);
    
    /// Wrapper for vTaskResume()
    /// Resumes a suspended task
    pub fn freertos_rs_task_resume(task_to_resume: FreeRtosTaskHandle);
    
    /// Wrapper for xTaskResumeFromISR()
    /// Resumes a suspended task from an ISR
    pub fn freertos_rs_task_resume_from_isr(task_to_resume: FreeRtosTaskHandle) -> FreeRtosBaseType;
    
    /// Wrapper for vTaskPrioritySet()
    /// Sets the priority of a task
    pub fn freertos_rs_task_priority_set(task: FreeRtosTaskHandle, new_priority: FreeRtosUBaseType);
    
    /// Wrapper for uxTaskPriorityGet()
    /// Gets the priority of a task
    pub fn freertos_rs_task_priority_get(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;

    /// Wrapper for uxTaskPriorityGetFromISR()
    /// Gets the priority of a task from an ISR
    pub fn freertos_rs_task_priority_get_from_isr(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;

    /// Wrapper for uxTaskBasePriorityGet()
    /// Gets the base priority of a task
    pub fn freertos_rs_task_base_priority_get(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;

    /// Wrapper for uxTaskBasePriorityGetFromISR()
    /// Gets the base priority of a task from an ISR
    pub fn freertos_rs_task_base_priority_get_from_isr(task: FreeRtosTaskHandle) -> FreeRtosUBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TICK FUNCTIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTaskGetTickCount()
    /// Gets the current tick count
    pub fn freertos_rs_task_get_tick_count() -> FreeRtosTickType;
    
    /// Wrapper for xTaskGetTickCountFromISR()
    /// Gets the current tick count from an ISR
    pub fn freertos_rs_task_get_tick_count_from_isr() -> FreeRtosTickType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TASK NOTIFICATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTaskNotify()
    /// Sends a notification to a task
    pub fn freertos_rs_task_notify(
        task_to_notify: FreeRtosTaskHandle,
        value: u32,
        action: u32
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTaskNotifyFromISR()
    /// Sends a notification to a task from an ISR
    pub fn freertos_rs_task_notify_from_isr(
        task_to_notify: FreeRtosTaskHandle,
        value: u32,
        action: u32,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTaskNotifyWait()
    /// Waits for a notification
    pub fn freertos_rs_task_notify_wait(
        bits_to_clear_on_entry: u32,
        bits_to_clear_on_exit: u32,
        notification_value: *mut u32,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTaskNotifyGive()
    /// Gives a notification (increment)
    pub fn freertos_rs_task_notify_give(task_to_notify: FreeRtosTaskHandle) -> FreeRtosBaseType;
    
    /// Wrapper for ulTaskNotifyTake()
    /// Takes a notification (decrement or clear)
    pub fn freertos_rs_task_notify_take(
        clear_count_on_exit: FreeRtosBaseType,
        ticks_to_wait: FreeRtosTickType
    ) -> u32;

    /// Wrapper for xTaskGenericNotify()
    /// Generic task notification function
    pub fn freertos_rs_task_generic_notify(
        task_to_notify: FreeRtosTaskHandle,
        index_to_notify: FreeRtosUBaseType,
        value: u32,
        action: u32,
        previous_notification_value: *mut u32
    ) -> FreeRtosBaseType;

    /// Wrapper for xTaskGenericNotifyFromISR()
    /// Generic task notification function from ISR
    pub fn freertos_rs_task_generic_notify_from_isr(
        task_to_notify: FreeRtosTaskHandle,
        index_to_notify: FreeRtosUBaseType,
        value: u32,
        action: u32,
        previous_notification_value: *mut u32,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;

    /// Wrapper for xTaskGenericNotifyWait()
    /// Generic task notification wait function
    pub fn freertos_rs_task_generic_notify_wait(
        index_to_wait_on: FreeRtosUBaseType,
        bits_to_clear_on_entry: u32,
        bits_to_clear_on_exit: u32,
        notification_value: *mut u32,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;

    /// Wrapper for xTaskGenericNotifyStateClear()
    /// Clears the notification state of a task
    pub fn freertos_rs_task_generic_notify_state_clear(
        task: FreeRtosTaskHandle,
        index_to_clear: FreeRtosUBaseType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TASK UTILITIES
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTaskGetApplicationTaskTag()
    /// Gets the application task tag
    pub fn freertos_rs_task_get_application_task_tag(
        task: FreeRtosTaskHandle
    ) -> FreeRtosVoidPtr;

    /// Wrapper for vTaskSetApplicationTaskTag()
    /// Sets the application task tag
    pub fn freertos_rs_task_set_application_task_tag(
        task: FreeRtosTaskHandle,
        tag_value: FreeRtosVoidPtr
    );

    /// Wrapper for xTaskCallApplicationTaskHook()
    /// Calls the application task hook
    pub fn freertos_rs_task_call_application_task_hook(
        task: FreeRtosTaskHandle,
        parameter: FreeRtosVoidPtr
    ) -> FreeRtosBaseType;

    /// Wrapper for pcTaskGetName()
    /// Gets the name of a task
    pub fn freertos_rs_task_get_name(
        task: FreeRtosTaskHandle
    ) -> *const u8;

    /// Wrapper for xTaskGetHandle()
    /// Gets the handle of a task by name
    pub fn freertos_rs_task_get_handle(
        task_name: *const u8
    ) -> FreeRtosTaskHandle;

    /// Wrapper for xTaskGetCurrentTaskHandle()
    /// Gets the handle of the currently running task
    pub fn freertos_rs_task_get_current_task_handle() -> FreeRtosTaskHandle;

    /// Wrapper for xTaskGetIdleTaskHandle()
    /// Gets the handle of the idle task
    pub fn freertos_rs_task_get_idle_task_handle() -> FreeRtosTaskHandle;

    /// Wrapper for uxTaskGetStackHighWaterMark()
    /// Gets the high water mark of a task's stack
    pub fn freertos_rs_task_get_stack_high_water_mark(
        task: FreeRtosTaskHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for uxTaskGetStackHighWaterMark2()
    /// Gets the high water mark of a task's stack (configSTACK_DEPTH_TYPE return type)
    pub fn freertos_rs_task_get_stack_high_water_mark2(
        task: FreeRtosTaskHandle
    ) -> FreeRtosConfigStackDepthType;

    /// Wrapper for xTaskGetStaticBuffers()
    /// Gets the static buffers associated with a task
    pub fn freertos_rs_task_get_static_buffers(
        task: FreeRtosTaskHandle,
        stack_buffer: *mut FreeRtosStackType,
        task_buffer: *mut FreeRtosStaticTask
    ) -> FreeRtosBaseType;

    /// Wrapper for ulTaskGetRunTimeCounter()
    /// Gets the run time counter for a task
    pub fn freertos_rs_task_get_run_time_counter(task: FreeRtosTaskHandle) -> u32;

    /// Wrapper for ulTaskGetRunTimePercent()
    /// Gets the run time percentage for a task
    pub fn freertos_rs_task_get_run_time_percent(task: FreeRtosTaskHandle) -> u32;

    /// Wrapper for eTaskGetState()
    /// Gets the state of a task
    pub fn freertos_rs_task_get_state(
        task: FreeRtosTaskHandle
    ) -> u32;

    /// Wrapper for vTaskList()
    /// Generates a human readable table of task states
    pub fn freertos_rs_task_list(
        write_buffer: *mut u8
    );

    /// Wrapper for vTaskGetRunTimeStats()
    /// Generates a human readable table of run time stats
    pub fn freertos_rs_task_get_run_time_stats(
        write_buffer: *mut u8
    );

    /// Wrapper for uxTaskGetNumberOfTasks()
    /// Gets the number of tasks in the system
    pub fn freertos_rs_task_get_number_of_tasks() -> FreeRtosUBaseType;

    /// Wrapper for uxTaskGetSystemState()
    /// Gets detailed task information
    pub fn freertos_rs_task_get_system_state(
        task_status_array: FreeRtosVoidPtr,
        array_size: FreeRtosUBaseType,
        total_run_time: *mut u32
    ) -> FreeRtosUBaseType;

    /// Wrapper for vTaskGetInfo()
    /// Gets information about a specific task
    pub fn freertos_rs_task_get_info(
        task: FreeRtosTaskHandle,
        task_status: FreeRtosVoidPtr,
        get_free_stack_space: FreeRtosBaseType,
        state: u32
    );

    /// Wrapper for vTaskSetThreadLocalStoragePointer()
    /// Sets a thread local storage pointer
    pub fn freertos_rs_task_set_thread_local_storage_pointer(
        task: FreeRtosTaskHandle,
        index: FreeRtosBaseType,
        value: FreeRtosVoidPtr
    );

    /// Wrapper for pvTaskGetThreadLocalStoragePointer()
    /// Gets a thread local storage pointer
    pub fn freertos_rs_task_get_thread_local_storage_pointer(
        task: FreeRtosTaskHandle,
        index: FreeRtosBaseType
    ) -> FreeRtosVoidPtr;

    /// Wrapper for xTaskAbortDelay()
    /// Aborts the delay of a task
    pub fn freertos_rs_task_abort_delay(
        task: FreeRtosTaskHandle
    ) -> FreeRtosBaseType;

    /// Wrapper for vTaskSetTimeOutState()
    /// Sets timeout state
    pub fn freertos_rs_task_set_time_out_state(
        time_out: *mut FreeRtosTimeOut
    );

    /// Wrapper for xTaskCheckForTimeOut()
    /// Checks for timeout
    pub fn freertos_rs_task_check_for_time_out(
        time_out: *mut FreeRtosTimeOut,
        ticks_to_wait: *mut FreeRtosTickType
    ) -> FreeRtosBaseType;

    /// Wrapper for xTaskCatchUpTicks()
    /// Catches up ticks after low power mode
    pub fn freertos_rs_task_catch_up_ticks(
        ticks_to_catch_up: FreeRtosTickType
    ) -> FreeRtosBaseType;

    /// Wrapper for vTaskResetState()
    /// Resets the task state
    pub fn freertos_rs_task_reset_state();

    /// Wrapper for ulTaskGenericNotifyValueClear()
    /// Clears specific bits in a task notification value
    pub fn freertos_rs_task_generic_notify_value_clear(
        task: FreeRtosTaskHandle,
        index_to_clear: FreeRtosUBaseType,
        bits_to_clear: u32
    ) -> u32;

    /// Wrapper for vTaskListTasks()
    /// Generates a human readable table of task states with buffer length
    pub fn freertos_rs_task_list_tasks(
        write_buffer: *mut u8,
        buffer_length: usize
    );

    /// Wrapper for vTaskGetRunTimeStatistics()
    /// Generates a human readable table of run time stats with buffer length
    pub fn freertos_rs_task_get_run_time_statistics(
        write_buffer: *mut u8,
        buffer_length: usize
    );
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - CRITICAL SECTIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for taskENTER_CRITICAL()
    /// Enters a critical section
    pub fn freertos_rs_task_enter_critical();

    /// Wrapper for taskEXIT_CRITICAL()
    /// Exits a critical section
    pub fn freertos_rs_task_exit_critical();

    /// Wrapper for taskENTER_CRITICAL_FROM_ISR()
    /// Enters a critical section from ISR
    pub fn freertos_rs_task_enter_critical_from_isr() -> FreeRtosUBaseType;

    /// Wrapper for taskEXIT_CRITICAL_FROM_ISR()
    /// Exits a critical section from ISR
    pub fn freertos_rs_task_exit_critical_from_isr(saved_interrupt_status: FreeRtosUBaseType);

    /// Wrapper for taskDISABLE_INTERRUPTS()
    /// Disables interrupts
    pub fn freertos_rs_task_disable_interrupts();

    /// Wrapper for taskENABLE_INTERRUPTS()
    /// Enables interrupts
    pub fn freertos_rs_task_enable_interrupts();

    /// Wrapper for vTaskAllocateMPURegions()
    /// Allocates MPU regions to a task
    pub fn freertos_rs_task_allocate_mpu_regions(
        task_to_modify: FreeRtosTaskHandle,
        regions: *const FreeRtosVoidPtr
    );

    /// Wrapper for xTaskIncrementTick()
    /// Increments the tick count (called by tick interrupt)
    pub fn freertos_rs_task_increment_tick() -> FreeRtosBaseType;

    /// Wrapper for vTaskStepTick()
    /// Steps the tick count forward by specified amount
    pub fn freertos_rs_task_step_tick(ticks_to_jump: FreeRtosTickType);

    /// Wrapper for vTaskPreemptionDisable()
    /// Disables preemption for a task
    pub fn freertos_rs_task_preemption_disable(task: FreeRtosTaskHandle);

    /// Wrapper for vTaskPreemptionEnable()
    /// Enables preemption for a task
    pub fn freertos_rs_task_preemption_enable(task: FreeRtosTaskHandle);
}
