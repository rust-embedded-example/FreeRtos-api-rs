use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosTimerHandle,
    FreeRtosTimerCallback, FreeRtosVoidPtr, FreeRtosUBaseType
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER CREATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTimerCreate()
    /// Creates a software timer
    pub fn freertos_rs_timer_create(
        timer_name: *const u8,
        timer_period: FreeRtosTickType,
        auto_reload: FreeRtosBaseType,
        timer_id: FreeRtosVoidPtr,
        callback_function: FreeRtosTimerCallback
    ) -> FreeRtosTimerHandle;
    
    /// Wrapper for xTimerCreateStatic()
    /// Creates a software timer using statically allocated memory
    pub fn freertos_rs_timer_create_static(
        timer_name: *const u8,
        timer_period: FreeRtosTickType,
        auto_reload: FreeRtosBaseType,
        timer_id: FreeRtosVoidPtr,
        callback_function: FreeRtosTimerCallback,
        timer_buffer: FreeRtosVoidPtr
    ) -> FreeRtosTimerHandle;
    
    /// Wrapper for xTimerDelete()
    /// Deletes a software timer
    pub fn freertos_rs_timer_delete(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER CONTROL
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTimerStart()
    /// Starts a software timer
    pub fn freertos_rs_timer_start(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerStop()
    /// Stops a software timer
    pub fn freertos_rs_timer_stop(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerReset()
    /// Resets a software timer
    pub fn freertos_rs_timer_reset(
        timer: FreeRtosTimerHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerChangePeriod()
    /// Changes the period of a software timer
    pub fn freertos_rs_timer_change_period(
        timer: FreeRtosTimerHandle,
        new_period: FreeRtosTickType,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTimerStartFromISR()
    /// Starts a software timer from an ISR
    pub fn freertos_rs_timer_start_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerStopFromISR()
    /// Stops a software timer from an ISR
    pub fn freertos_rs_timer_stop_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerResetFromISR()
    /// Resets a software timer from an ISR
    pub fn freertos_rs_timer_reset_from_isr(
        timer: FreeRtosTimerHandle,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerChangePeriodFromISR()
    /// Changes the period of a software timer from an ISR
    pub fn freertos_rs_timer_change_period_from_isr(
        timer: FreeRtosTimerHandle,
        new_period: FreeRtosTickType,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - TIMER INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xTimerIsTimerActive()
    /// Checks if a timer is active
    pub fn freertos_rs_timer_is_timer_active(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xTimerGetTimerDaemonTaskHandle()
    /// Gets the handle of the timer daemon task
    pub fn freertos_rs_timer_get_timer_daemon_task_handle() -> FreeRtosVoidPtr;
    
    /// Wrapper for xTimerGetPeriod()
    /// Gets the period of a timer
    pub fn freertos_rs_timer_get_period(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosTickType;
    
    /// Wrapper for xTimerGetExpiryTime()
    /// Gets the expiry time of a timer
    pub fn freertos_rs_timer_get_expiry_time(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosTickType;
    
    /// Wrapper for pvTimerGetTimerID()
    /// Gets the ID of a timer
    pub fn freertos_rs_timer_get_timer_id(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosVoidPtr;
    
    /// Wrapper for vTimerSetTimerID()
    /// Sets the ID of a timer
    pub fn freertos_rs_timer_set_timer_id(
        timer: FreeRtosTimerHandle,
        new_id: FreeRtosVoidPtr
    );

    /// Wrapper for pcTimerGetName()
    /// Gets the name of a timer
    pub fn freertos_rs_timer_get_name(
        timer: FreeRtosTimerHandle
    ) -> *const u8;

    /// Wrapper for xTimerGetStaticBuffer()
    /// Gets the static buffer associated with a timer
    pub fn freertos_rs_timer_get_static_buffer(
        timer: FreeRtosTimerHandle,
        timer_buffer: *mut FreeRtosVoidPtr
    ) -> FreeRtosBaseType;

    /// Wrapper for uxTimerGetTimerNumber()
    /// Gets the timer number for tracing
    pub fn freertos_rs_timer_get_timer_number(
        timer: FreeRtosTimerHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for vTimerSetTimerNumber()
    /// Sets the timer number for tracing
    pub fn freertos_rs_timer_set_timer_number(
        timer: FreeRtosTimerHandle,
        timer_number: FreeRtosUBaseType
    );

    /// Wrapper for xTimerPendFunctionCall()
    /// Pends a function call to be executed by the timer daemon task
    pub fn freertos_rs_timer_pend_function_call(
        function_to_pend: FreeRtosVoidPtr,
        parameter1: FreeRtosVoidPtr,
        parameter2: u32,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;

    /// Wrapper for xTimerPendFunctionCallFromISR()
    /// Pends a function call from an ISR to be executed by the timer daemon task
    pub fn freertos_rs_timer_pend_function_call_from_isr(
        function_to_pend: FreeRtosVoidPtr,
        parameter1: FreeRtosVoidPtr,
        parameter2: u32,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
}
