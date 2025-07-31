use crate::base::{FreeRtosBaseType, FreeRtosTickType};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - UTILITY FUNCTIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for pdMS_TO_TICKS macro
    /// Converts milliseconds to ticks
    pub fn freertos_rs_ms_to_ticks(time_in_ms: FreeRtosTickType) -> FreeRtosTickType;
    
    /// Wrapper for pdTICKS_TO_MS macro
    /// Converts ticks to milliseconds
    pub fn freertos_rs_ticks_to_ms(time_in_ticks: FreeRtosTickType) -> FreeRtosTickType;
    
    /// Wrapper for pdTRUE constant
    /// Gets the pdTRUE constant value
    pub fn freertos_rs_get_pd_true() -> FreeRtosBaseType;
    
    /// Wrapper for pdFALSE constant
    /// Gets the pdFALSE constant value
    pub fn freertos_rs_get_pd_false() -> FreeRtosBaseType;
    
    /// Wrapper for pdPASS constant
    /// Gets the pdPASS constant value
    pub fn freertos_rs_get_pd_pass() -> FreeRtosBaseType;
    
    /// Wrapper for pdFAIL constant
    /// Gets the pdFAIL constant value
    pub fn freertos_rs_get_pd_fail() -> FreeRtosBaseType;

    /// Wrapper for portMAX_DELAY constant
    /// Gets the portMAX_DELAY constant value
    pub fn freertos_rs_get_port_max_delay() -> FreeRtosTickType;

    /// Wrapper for portTICK_PERIOD_MS constant
    /// Gets the portTICK_PERIOD_MS constant value
    pub fn freertos_rs_get_port_tick_period_ms() -> FreeRtosTickType;
}

//===========================================================================
// SCHEDULER STATE CONSTANTS
//===========================================================================

/// Scheduler has not been started (taskSCHEDULER_NOT_STARTED)
pub const TASK_SCHEDULER_NOT_STARTED: FreeRtosBaseType = 0;

/// Scheduler is running normally (taskSCHEDULER_RUNNING)
pub const TASK_SCHEDULER_RUNNING: FreeRtosBaseType = 1;

/// Scheduler is suspended (taskSCHEDULER_SUSPENDED)
pub const TASK_SCHEDULER_SUSPENDED: FreeRtosBaseType = 2;
