/*
 * FreeRTOS Rust API Wrapper - C Layer
 *
 * This file provides C wrapper functions for FreeRTOS APIs to be called from Rust.
 * All wrapper functions follow the naming convention: freertos_rs_<function_name>
 *
 * Copyright (c) 2024
 * SPDX-License-Identifier: MIT
 */

#include "FreeRTOS.h"
#include "task.h"
#include "portable.h"
#include "projdefs.h"
#include "queue.h"
#include "semphr.h"
#include "event_groups.h"
#include "timers.h"
#include "stream_buffer.h"
#include "message_buffer.h"
#include "atomic.h"
#include "list.h"

/*===========================================================================
 * TASK CREATION FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for xTaskCreate()
 * Creates a new task and adds it to the list of tasks that are ready to run
 * @param pxTaskCode Pointer to the task entry function
 * @param pcName Descriptive name for the task
 * @param usStackDepth Stack depth in words
 * @param pvParameters Pointer passed as parameter to the task
 * @param uxPriority Priority of the task
 * @param pxCreatedTask Handle to the created task
 * @return BaseType_t - pdPASS if successful, errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY if failed
 */
BaseType_t freertos_rs_task_create(
    TaskFunction_t pxTaskCode,
    const char * const pcName,
    const configSTACK_DEPTH_TYPE usStackDepth,
    void * const pvParameters,
    UBaseType_t uxPriority,
    TaskHandle_t * const pxCreatedTask)
{
    return xTaskCreate(pxTaskCode, pcName, usStackDepth, pvParameters, uxPriority, pxCreatedTask);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xTaskCreateStatic()
 * Creates a new task using statically allocated memory
 * @param pxTaskCode Pointer to the task entry function
 * @param pcName Descriptive name for the task
 * @param ulStackDepth Stack depth in words
 * @param pvParameters Pointer passed as parameter to the task
 * @param uxPriority Priority of the task
 * @param puxStackBuffer Pointer to stack buffer
 * @param pxTaskBuffer Pointer to task buffer
 * @return TaskHandle_t - Handle to the created task
 */
TaskHandle_t freertos_rs_task_create_static(
    TaskFunction_t pxTaskCode,
    const char * const pcName,
    const uint32_t ulStackDepth,
    void * const pvParameters,
    UBaseType_t uxPriority,
    StackType_t * const puxStackBuffer,
    StaticTask_t * const pxTaskBuffer)
{
    return xTaskCreateStatic(pxTaskCode, pcName, ulStackDepth, pvParameters, uxPriority, puxStackBuffer, pxTaskBuffer);
}
#endif

#if (portUSING_MPU_WRAPPERS == 1)
/**
 * @brief Wrapper for xTaskCreateRestricted()
 * Creates a new restricted task for MPU systems
 * @param pxTaskDefinition Pointer to task parameters structure
 * @param pxCreatedTask Handle to the created task
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_create_restricted(
    const TaskParameters_t * const pxTaskDefinition,
    TaskHandle_t * pxCreatedTask)
{
    return xTaskCreateRestricted(pxTaskDefinition, pxCreatedTask);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xTaskCreateRestrictedStatic()
 * Creates a new restricted task using static allocation
 * @param pxTaskDefinition Pointer to task parameters structure
 * @param pxCreatedTask Handle to the created task
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_create_restricted_static(
    const TaskParameters_t * const pxTaskDefinition,
    TaskHandle_t * pxCreatedTask)
{
    return xTaskCreateRestrictedStatic(pxTaskDefinition, pxCreatedTask);
}

#if ((configNUMBER_OF_CORES > 1) && (configUSE_CORE_AFFINITY == 1))
/**
 * @brief Wrapper for xTaskCreateRestrictedStaticAffinitySet()
 * Creates a new restricted task with affinity using static allocation
 * @param pxTaskDefinition Pointer to task parameters structure
 * @param uxCoreAffinityMask Core affinity mask
 * @param pxCreatedTask Handle to the created task
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_create_restricted_static_affinity_set(
    const TaskParameters_t * const pxTaskDefinition,
    UBaseType_t uxCoreAffinityMask,
    TaskHandle_t * pxCreatedTask)
{
    return xTaskCreateRestrictedStaticAffinitySet(pxTaskDefinition, uxCoreAffinityMask, pxCreatedTask);
}
#endif
#endif
#endif

/*===========================================================================
 * TASK MANAGEMENT FUNCTIONS
 *===========================================================================*/

/*===========================================================================
 * SCHEDULER CONTROL FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for vTaskStartScheduler()
 * Starts the FreeRTOS scheduler
 */
void freertos_rs_task_start_scheduler(void)
{
    vTaskStartScheduler();
}

/**
 * @brief Wrapper for vTaskEndScheduler()
 * Ends the FreeRTOS scheduler
 */
void freertos_rs_task_end_scheduler(void)
{
    vTaskEndScheduler();
}

/**
 * @brief Wrapper for vTaskSuspendAll()
 * Suspends the scheduler without disabling interrupts
 */
void freertos_rs_task_suspend_all(void)
{
    vTaskSuspendAll();
}

/**
 * @brief Wrapper for xTaskResumeAll()
 * Resumes the scheduler after suspension
 * @return BaseType_t - pdTRUE if a context switch is required
 */
BaseType_t freertos_rs_task_resume_all(void)
{
    return xTaskResumeAll();
}

/**
 * @brief Wrapper for xTaskGetSchedulerState()
 * Gets the current scheduler state
 * @return BaseType_t - Scheduler state (running, not started, or suspended)
 */
BaseType_t freertos_rs_task_get_scheduler_state(void)
{
    return xTaskGetSchedulerState();
}

/*===========================================================================
 * TASK CONTROL FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for vTaskDelay()
 * Delays the current task for a specified number of ticks
 * @param xTicksToDelay Number of ticks to delay
 */
void freertos_rs_task_delay(const TickType_t xTicksToDelay)
{
    vTaskDelay(xTicksToDelay);
}

/**
 * @brief Wrapper for xTaskDelayUntil()
 * Delays a task until a specified time
 * @param pxPreviousWakeTime Pointer to previous wake time
 * @param xTimeIncrement Time increment
 * @return BaseType_t - pdTRUE if the task was delayed
 */
BaseType_t freertos_rs_task_delay_until(TickType_t * const pxPreviousWakeTime, const TickType_t xTimeIncrement)
{
    return xTaskDelayUntil(pxPreviousWakeTime, xTimeIncrement);
}

/**
 * @brief Wrapper for vTaskDelete()
 * Deletes a task
 * @param xTaskToDelete Handle of task to delete (NULL for current task)
 */
void freertos_rs_task_delete(TaskHandle_t xTaskToDelete)
{
    vTaskDelete(xTaskToDelete);
}

/**
 * @brief Wrapper for vTaskSuspend()
 * Suspends a task
 * @param xTaskToSuspend Handle of task to suspend (NULL for current task)
 */
void freertos_rs_task_suspend(TaskHandle_t xTaskToSuspend)
{
    vTaskSuspend(xTaskToSuspend);
}

/**
 * @brief Wrapper for vTaskResume()
 * Resumes a suspended task
 * @param xTaskToResume Handle of task to resume
 */
void freertos_rs_task_resume(TaskHandle_t xTaskToResume)
{
    vTaskResume(xTaskToResume);
}

/**
 * @brief Wrapper for xTaskResumeFromISR()
 * Resumes a suspended task from an ISR
 * @param xTaskToResume Handle of task to resume
 * @return BaseType_t - pdTRUE if a context switch is required
 */
BaseType_t freertos_rs_task_resume_from_isr(TaskHandle_t xTaskToResume)
{
    return xTaskResumeFromISR(xTaskToResume);
}

/**
 * @brief Wrapper for vTaskPrioritySet()
 * Sets the priority of a task
 * @param xTask Handle of task (NULL for current task)
 * @param uxNewPriority New priority for the task
 */
void freertos_rs_task_priority_set(TaskHandle_t xTask, UBaseType_t uxNewPriority)
{
    vTaskPrioritySet(xTask, uxNewPriority);
}

/**
 * @brief Wrapper for uxTaskPriorityGet()
 * Gets the priority of a task
 * @param xTask Handle of task (NULL for current task)
 * @return UBaseType_t - Task priority
 */
UBaseType_t freertos_rs_task_priority_get(TaskHandle_t xTask)
{
    return uxTaskPriorityGet(xTask);
}

/**
 * @brief Wrapper for uxTaskPriorityGetFromISR()
 * Gets the priority of a task from an ISR
 * @param xTask Handle of task
 * @return UBaseType_t - Task priority
 */
UBaseType_t freertos_rs_task_priority_get_from_isr(TaskHandle_t xTask)
{
    return uxTaskPriorityGetFromISR(xTask);
}

/**
 * @brief Wrapper for uxTaskBasePriorityGet()
 * Gets the base priority of a task
 * @param xTask Handle of the task to query
 * @return UBaseType_t - Base priority of the task
 */
UBaseType_t freertos_rs_task_base_priority_get(TaskHandle_t xTask)
{
    return uxTaskBasePriorityGet(xTask);
}

/**
 * @brief Wrapper for uxTaskBasePriorityGetFromISR()
 * Gets the base priority of a task from an ISR
 * @param xTask Handle of the task to query
 * @return UBaseType_t - Base priority of the task
 */
UBaseType_t freertos_rs_task_base_priority_get_from_isr(TaskHandle_t xTask)
{
    return uxTaskBasePriorityGetFromISR(xTask);
}

#if ((configSUPPORT_DYNAMIC_ALLOCATION == 1) && (configNUMBER_OF_CORES > 1) && (configUSE_CORE_AFFINITY == 1))
/**
 * @brief Wrapper for xTaskCreateAffinitySet()
 * Creates a new task with core affinity
 * @param pxTaskCode Pointer to the task entry function
 * @param pcName Descriptive name for the task
 * @param usStackDepth Stack depth in words
 * @param pvParameters Pointer passed as parameter to the task
 * @param uxPriority Priority of the task
 * @param uxCoreAffinityMask Core affinity mask
 * @param pxCreatedTask Handle to the created task
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_create_affinity_set(
    TaskFunction_t pxTaskCode,
    const char * const pcName,
    const configSTACK_DEPTH_TYPE usStackDepth,
    void * const pvParameters,
    UBaseType_t uxPriority,
    UBaseType_t uxCoreAffinityMask,
    TaskHandle_t * const pxCreatedTask)
{
    return xTaskCreateAffinitySet(pxTaskCode, pcName, usStackDepth, pvParameters, uxPriority, uxCoreAffinityMask, pxCreatedTask);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xTaskCreateStaticAffinitySet()
 * Creates a new task with core affinity using static allocation
 * @param pxTaskCode Pointer to the task entry function
 * @param pcName Descriptive name for the task
 * @param ulStackDepth Stack depth in words
 * @param pvParameters Pointer passed as parameter to the task
 * @param uxPriority Priority of the task
 * @param puxStackBuffer Pointer to stack buffer
 * @param pxTaskBuffer Pointer to task buffer
 * @param uxCoreAffinityMask Core affinity mask
 * @return TaskHandle_t - Handle to the created task
 */
TaskHandle_t freertos_rs_task_create_static_affinity_set(
    TaskFunction_t pxTaskCode,
    const char * const pcName,
    const configSTACK_DEPTH_TYPE ulStackDepth,
    void * const pvParameters,
    UBaseType_t uxPriority,
    StackType_t * const puxStackBuffer,
    StaticTask_t * const pxTaskBuffer,
    UBaseType_t uxCoreAffinityMask)
{
    return xTaskCreateStaticAffinitySet(pxTaskCode, pcName, ulStackDepth, pvParameters, uxPriority, puxStackBuffer, pxTaskBuffer, uxCoreAffinityMask);
}
#endif

/**
 * @brief Wrapper for vTaskCoreAffinitySet()
 * Sets the core affinity of a task
 * @param xTask Handle of the task
 * @param uxCoreAffinityMask Core affinity mask
 */
void freertos_rs_task_core_affinity_set(TaskHandle_t xTask, UBaseType_t uxCoreAffinityMask)
{
    vTaskCoreAffinitySet(xTask, uxCoreAffinityMask);
}

/**
 * @brief Wrapper for uxTaskCoreAffinityGet()
 * Gets the core affinity of a task
 * @param xTask Handle of the task
 * @return UBaseType_t - Core affinity mask
 */
UBaseType_t freertos_rs_task_core_affinity_get(TaskHandle_t xTask)
{
    return uxTaskCoreAffinityGet(xTask);
}
#endif

#if (portUSING_MPU_WRAPPERS == 1)
/**
 * @brief Wrapper for vTaskAllocateMPURegions()
 * Allocates MPU regions to a task
 * @param xTaskToModify Handle of task to modify
 * @param pxRegions Pointer to memory regions
 */
void freertos_rs_task_allocate_mpu_regions(TaskHandle_t xTaskToModify, const MemoryRegion_t * const pxRegions)
{
    vTaskAllocateMPURegions(xTaskToModify, pxRegions);
}
#endif

/**
 * @brief Wrapper for xTaskIncrementTick()
 * Increments the tick count (called by tick interrupt)
 * @return BaseType_t - pdTRUE if context switch required
 */
BaseType_t freertos_rs_task_increment_tick(void)
{
    return xTaskIncrementTick();
}

#if (configUSE_TICKLESS_IDLE != 0)
/**
 * @brief Wrapper for vTaskStepTick()
 * Steps the tick count forward by specified amount
 * @param xTicksToJump Number of ticks to jump
 */
void freertos_rs_task_step_tick(TickType_t xTicksToJump)
{
    vTaskStepTick(xTicksToJump);
}
#endif

#if (configUSE_PREEMPTION == 0)
/**
 * @brief Wrapper for vTaskPreemptionDisable()
 * Disables preemption for a task
 * @param xTask Handle of task (NULL for current task)
 */
void freertos_rs_task_preemption_disable(TaskHandle_t xTask)
{
    vTaskPreemptionDisable(xTask);
}

/**
 * @brief Wrapper for vTaskPreemptionEnable()
 * Enables preemption for a task
 * @param xTask Handle of task (NULL for current task)
 */
void freertos_rs_task_preemption_enable(TaskHandle_t xTask)
{
    vTaskPreemptionEnable(xTask);
}
#endif

/*===========================================================================
 * TICK FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for xTaskGetTickCount()
 * Gets the current tick count
 * @return TickType_t - Current tick count
 */
TickType_t freertos_rs_task_get_tick_count(void)
{
    return xTaskGetTickCount();
}

/**
 * @brief Wrapper for xTaskGetTickCountFromISR()
 * Gets the current tick count from an ISR
 * @return TickType_t - Current tick count
 */
TickType_t freertos_rs_task_get_tick_count_from_isr(void)
{
    return xTaskGetTickCountFromISR();
}

/*===========================================================================
 * TASK NOTIFICATION FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for xTaskNotify()
 * Sends a notification to a task
 * @param xTaskToNotify Handle of task to notify
 * @param ulValue Value to send
 * @param eAction Action to perform
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_notify(TaskHandle_t xTaskToNotify, uint32_t ulValue, uint32_t eAction)
{
    return xTaskGenericNotify(xTaskToNotify, tskDEFAULT_INDEX_TO_NOTIFY, ulValue, (eNotifyAction)eAction, NULL);
}

/**
 * @brief Wrapper for xTaskNotifyFromISR()
 * Sends a notification to a task from an ISR
 * @param xTaskToNotify Handle of task to notify
 * @param ulValue Value to send
 * @param eAction Action to perform
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_notify_from_isr(TaskHandle_t xTaskToNotify, uint32_t ulValue, uint32_t eAction, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xTaskGenericNotifyFromISR(xTaskToNotify, tskDEFAULT_INDEX_TO_NOTIFY, ulValue, (eNotifyAction)eAction, NULL, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xTaskNotifyWait()
 * Waits for a notification
 * @param ulBitsToClearOnEntry Bits to clear on entry
 * @param ulBitsToClearOnExit Bits to clear on exit
 * @param pulNotificationValue Pointer to notification value
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_notify_wait(uint32_t ulBitsToClearOnEntry, uint32_t ulBitsToClearOnExit, uint32_t *pulNotificationValue, TickType_t xTicksToWait)
{
    return xTaskGenericNotifyWait(tskDEFAULT_INDEX_TO_NOTIFY, ulBitsToClearOnEntry, ulBitsToClearOnExit, pulNotificationValue, xTicksToWait);
}

/**
 * @brief Wrapper for xTaskNotifyGive()
 * Gives a notification (increment)
 * @param xTaskToNotify Handle of task to notify
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_notify_give(TaskHandle_t xTaskToNotify)
{
    return xTaskGenericNotify(xTaskToNotify, tskDEFAULT_INDEX_TO_NOTIFY, 0, eIncrement, NULL);
}

/**
 * @brief Wrapper for ulTaskNotifyTake()
 * Takes a notification (decrement or clear)
 * @param xClearCountOnExit Clear count on exit flag
 * @param xTicksToWait Ticks to wait
 * @return uint32_t - Notification value
 */
uint32_t freertos_rs_task_notify_take(BaseType_t xClearCountOnExit, TickType_t xTicksToWait)
{
    return ulTaskGenericNotifyTake(tskDEFAULT_INDEX_TO_NOTIFY, xClearCountOnExit, xTicksToWait);
}

/**
 * @brief Wrapper for ulTaskGenericNotifyTake()
 * Generic indexed notification take
 * @param uxIndexToWaitOn Notification array index
 * @param xClearCountOnExit Clear count on exit flag
 * @param xTicksToWait Ticks to wait
 * @return uint32_t - Notification value
 */
uint32_t freertos_rs_task_generic_notify_take(UBaseType_t uxIndexToWaitOn, BaseType_t xClearCountOnExit, TickType_t xTicksToWait)
{
    return ulTaskGenericNotifyTake(uxIndexToWaitOn, xClearCountOnExit, xTicksToWait);
}

/**
 * @brief Wrapper for xTaskGenericNotify()
 * Generic task notification function
 * @param xTaskToNotify Handle of task to notify
 * @param uxIndexToNotify Index of notification array
 * @param ulValue Value to send
 * @param eAction Action to perform
 * @param pulPreviousNotificationValue Pointer to previous notification value
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_generic_notify(TaskHandle_t xTaskToNotify, UBaseType_t uxIndexToNotify, uint32_t ulValue, uint32_t eAction, uint32_t *pulPreviousNotificationValue)
{
    return xTaskGenericNotify(xTaskToNotify, uxIndexToNotify, ulValue, (eNotifyAction)eAction, pulPreviousNotificationValue);
}

/**
 * @brief Wrapper for xTaskGenericNotifyFromISR()
 * Generic task notification function from ISR
 * @param xTaskToNotify Handle of task to notify
 * @param uxIndexToNotify Index of notification array
 * @param ulValue Value to send
 * @param eAction Action to perform
 * @param pulPreviousNotificationValue Pointer to previous notification value
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_generic_notify_from_isr(TaskHandle_t xTaskToNotify, UBaseType_t uxIndexToNotify, uint32_t ulValue, uint32_t eAction, uint32_t *pulPreviousNotificationValue, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xTaskGenericNotifyFromISR(xTaskToNotify, uxIndexToNotify, ulValue, (eNotifyAction)eAction, pulPreviousNotificationValue, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xTaskGenericNotifyWait()
 * Generic task notification wait function
 * @param uxIndexToWaitOn Index to wait on
 * @param ulBitsToClearOnEntry Bits to clear on entry
 * @param ulBitsToClearOnExit Bits to clear on exit
 * @param pulNotificationValue Pointer to notification value
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_generic_notify_wait(UBaseType_t uxIndexToWaitOn, uint32_t ulBitsToClearOnEntry, uint32_t ulBitsToClearOnExit, uint32_t *pulNotificationValue, TickType_t xTicksToWait)
{
    return xTaskGenericNotifyWait(uxIndexToWaitOn, ulBitsToClearOnEntry, ulBitsToClearOnExit, pulNotificationValue, xTicksToWait);
}

/**
 * @brief Wrapper for xTaskGenericNotifyStateClear()
 * Clears the notification state of a task
 * @param xTask Handle of task
 * @param uxIndexToClear Index to clear
 * @return BaseType_t - pdTRUE if notification was pending
 */
BaseType_t freertos_rs_task_generic_notify_state_clear(TaskHandle_t xTask, UBaseType_t uxIndexToClear)
{
    return xTaskGenericNotifyStateClear(xTask, uxIndexToClear);
}

/*===========================================================================
 * TASK UTILITY FUNCTIONS
 *===========================================================================*/

#if (configUSE_APPLICATION_TASK_TAG == 1)
/**
 * @brief Wrapper for xTaskGetApplicationTaskTag()
 * Gets the application task tag
 * @param xTask Handle of task (NULL for current task)
 * @return void* - Task tag value
 */
void* freertos_rs_task_get_application_task_tag(TaskHandle_t xTask)
{
    return xTaskGetApplicationTaskTag(xTask);
}

/**
 * @brief Wrapper for xTaskGetApplicationTaskTagFromISR()
 * Gets the application task tag from ISR context
 * @param xTask Handle of task
 * @return void* - Task tag value
 */
void* freertos_rs_task_get_application_task_tag_from_isr(TaskHandle_t xTask)
{
    return (void*)xTaskGetApplicationTaskTagFromISR(xTask);
}

/**
 * @brief Wrapper for vTaskSetApplicationTaskTag()
 * Sets the application task tag
 * @param xTask Handle of task (NULL for current task)
 * @param pxHookFunction Tag value to set
 */
void freertos_rs_task_set_application_task_tag(TaskHandle_t xTask, void* pxHookFunction)
{
    vTaskSetApplicationTaskTag(xTask, (TaskHookFunction_t)pxHookFunction);
}

/**
 * @brief Wrapper for xTaskCallApplicationTaskHook()
 * Calls the application task hook
 * @param xTask Handle of task
 * @param pvParameter Parameter to pass to hook
 * @return BaseType_t - Return value from hook
 */
BaseType_t freertos_rs_task_call_application_task_hook(TaskHandle_t xTask, void* pvParameter)
{
    return xTaskCallApplicationTaskHook(xTask, pvParameter);
}
#endif

/**
 * @brief Wrapper for pcTaskGetName()
 * Gets the name of a task
 * @param xTaskToQuery Handle of task (NULL for current task)
 * @return const char* - Task name
 */
const char* freertos_rs_task_get_name(TaskHandle_t xTaskToQuery)
{
    return pcTaskGetName(xTaskToQuery);
}

/**
 * @brief Wrapper for xTaskGetHandle()
 * Gets the handle of a task by name
 * @param pcNameToQuery Name of task to find
 * @return TaskHandle_t - Task handle or NULL if not found
 */
TaskHandle_t freertos_rs_task_get_handle(const char* pcNameToQuery)
{
    return xTaskGetHandle(pcNameToQuery);
}

/**
 * @brief Wrapper for xTaskGetCurrentTaskHandle()
 * Gets the handle of the currently running task
 * @return TaskHandle_t - Current task handle
 */
TaskHandle_t freertos_rs_task_get_current_task_handle(void)
{
    return xTaskGetCurrentTaskHandle();
}

/**
 * @brief Wrapper for xTaskGetIdleTaskHandle()
 * Gets the handle of the idle task
 * @return TaskHandle_t - Idle task handle
 */
TaskHandle_t freertos_rs_task_get_idle_task_handle(void)
{
    return xTaskGetIdleTaskHandle();
}

/**
 * @brief Wrapper for uxTaskGetStackHighWaterMark()
 * Gets the high water mark of a task's stack
 * @param xTask Handle of task (NULL for current task)
 * @return UBaseType_t - High water mark in words
 */
UBaseType_t freertos_rs_task_get_stack_high_water_mark(TaskHandle_t xTask)
{
    return uxTaskGetStackHighWaterMark(xTask);
}

#if (INCLUDE_uxTaskGetStackHighWaterMark2 == 1)
/**
 * @brief Wrapper for uxTaskGetStackHighWaterMark2()
 * Gets the high water mark of a task's stack (configSTACK_DEPTH_TYPE return type)
 * @param xTask Handle of task (NULL for current task)
 * @return configSTACK_DEPTH_TYPE - High water mark in stack depth type
 */
configSTACK_DEPTH_TYPE freertos_rs_task_get_stack_high_water_mark2(TaskHandle_t xTask)
{
    return uxTaskGetStackHighWaterMark2(xTask);
}
#endif

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xTaskGetStaticBuffers()
 * Gets the static buffers associated with a task
 * @param xTask Handle of the task
 * @param ppuxStackBuffer Pointer to receive stack buffer pointer
 * @param ppxTaskBuffer Pointer to receive task buffer pointer
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_task_get_static_buffers(TaskHandle_t xTask, StackType_t **ppuxStackBuffer, StaticTask_t **ppxTaskBuffer)
{
    return xTaskGetStaticBuffers(xTask, ppuxStackBuffer, ppxTaskBuffer);
}
#endif

#if (configGENERATE_RUN_TIME_STATS == 1)
/**
 * @brief Wrapper for ulTaskGetRunTimeCounter()
 * Gets the run time counter for a task
 * @param xTask Handle of the task
 * @return configRUN_TIME_COUNTER_TYPE - Run time counter value
 */
configRUN_TIME_COUNTER_TYPE freertos_rs_task_get_run_time_counter(TaskHandle_t xTask)
{
    return ulTaskGetRunTimeCounter(xTask);
}

/**
 * @brief Wrapper for ulTaskGetRunTimePercent()
 * Gets the run time percentage for a task
 * @param xTask Handle of the task
 * @return configRUN_TIME_COUNTER_TYPE - Run time percentage
 */
configRUN_TIME_COUNTER_TYPE freertos_rs_task_get_run_time_percent(TaskHandle_t xTask)
{
    return ulTaskGetRunTimePercent(xTask);
}
#endif

/**
 * @brief Wrapper for eTaskGetState()
 * Gets the state of a task
 * @param xTask Handle of task
 * @return uint32_t - Task state
 */
uint32_t freertos_rs_task_get_state(TaskHandle_t xTask)
{
    return (uint32_t)eTaskGetState(xTask);
}

#if ((configUSE_TRACE_FACILITY == 1) && (configUSE_STATS_FORMATTING_FUNCTIONS > 0))
/**
 * @brief Wrapper for vTaskList()
 * Generates a human readable table of task states
 * @param pcWriteBuffer Buffer to write the table to
 */
void freertos_rs_task_list(char* pcWriteBuffer)
{
    vTaskList(pcWriteBuffer);
}
#endif

#if ((configGENERATE_RUN_TIME_STATS == 1) && (configUSE_STATS_FORMATTING_FUNCTIONS > 0) && (configUSE_TRACE_FACILITY == 1))
/**
 * @brief Wrapper for vTaskGetRunTimeStats()
 * Generates a human readable table of run time stats
 * @param pcWriteBuffer Buffer to write the table to
 */
void freertos_rs_task_get_run_time_stats(char* pcWriteBuffer)
{
    vTaskGetRunTimeStats(pcWriteBuffer);
}
#endif

/**
 * @brief Wrapper for uxTaskGetNumberOfTasks()
 * Gets the number of tasks in the system
 * @return UBaseType_t - Number of tasks
 */
UBaseType_t freertos_rs_task_get_number_of_tasks(void)
{
    return uxTaskGetNumberOfTasks();
}

#if (configUSE_TRACE_FACILITY == 1)
/**
 * @brief Wrapper for uxTaskGetSystemState()
 * Gets detailed task information
 * @param pxTaskStatusArray Array to fill with task status
 * @param uxArraySize Size of the array
 * @param pulTotalRunTime Pointer to total run time
 * @return UBaseType_t - Number of tasks returned
 */
UBaseType_t freertos_rs_task_get_system_state(TaskStatus_t* pxTaskStatusArray, UBaseType_t uxArraySize, uint32_t* pulTotalRunTime)
{
    return uxTaskGetSystemState(pxTaskStatusArray, uxArraySize, pulTotalRunTime);
}

/**
 * @brief Wrapper for vTaskGetInfo()
 * Gets information about a specific task
 * @param xTask Handle of task to query
 * @param pxTaskStatus Pointer to TaskStatus_t structure to fill
 * @param xGetFreeStackSpace Include free stack space calculation
 * @param eState Task state to use if task is deleted
 */
void freertos_rs_task_get_info(TaskHandle_t xTask, TaskStatus_t* pxTaskStatus, BaseType_t xGetFreeStackSpace, uint32_t eState)
{
    vTaskGetInfo(xTask, pxTaskStatus, xGetFreeStackSpace, (eTaskState)eState);
}
#endif

#if (configNUM_THREAD_LOCAL_STORAGE_POINTERS > 0)
/**
 * @brief Wrapper for vTaskSetThreadLocalStoragePointer()
 * Sets a thread local storage pointer
 * @param xTaskToSet Handle of task (NULL for current task)
 * @param xIndex Index of storage pointer
 * @param pvValue Value to set
 */
void freertos_rs_task_set_thread_local_storage_pointer(TaskHandle_t xTaskToSet, BaseType_t xIndex, void* pvValue)
{
    vTaskSetThreadLocalStoragePointer(xTaskToSet, xIndex, pvValue);
}

/**
 * @brief Wrapper for pvTaskGetThreadLocalStoragePointer()
 * Gets a thread local storage pointer
 * @param xTaskToQuery Handle of task (NULL for current task)
 * @param xIndex Index of storage pointer
 * @return void* - Storage pointer value
 */
void* freertos_rs_task_get_thread_local_storage_pointer(TaskHandle_t xTaskToQuery, BaseType_t xIndex)
{
    return pvTaskGetThreadLocalStoragePointer(xTaskToQuery, xIndex);
}
#endif

#if (INCLUDE_xTaskAbortDelay == 1)
/**
 * @brief Wrapper for xTaskAbortDelay()
 * Aborts the delay of a task
 * @param xTask Handle of task
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_task_abort_delay(TaskHandle_t xTask)
{
    return xTaskAbortDelay(xTask);
}
#endif

/**
 * @brief Wrapper for vTaskSetTimeOutState()
 * Sets timeout state
 * @param pxTimeOut Pointer to timeout state structure
 */
void freertos_rs_task_set_time_out_state(TimeOut_t* pxTimeOut)
{
    vTaskSetTimeOutState(pxTimeOut);
}

/**
 * @brief Wrapper for xTaskCheckForTimeOut()
 * Checks for timeout
 * @param pxTimeOut Pointer to timeout state structure
 * @param pxTicksToWait Pointer to ticks to wait
 * @return BaseType_t - pdTRUE if timeout occurred
 */
BaseType_t freertos_rs_task_check_for_time_out(TimeOut_t* pxTimeOut, TickType_t* pxTicksToWait)
{
    return xTaskCheckForTimeOut(pxTimeOut, pxTicksToWait);
}

#if (configUSE_TICKLESS_IDLE != 0)
/**
 * @brief Wrapper for xTaskCatchUpTicks()
 * Catches up ticks after low power mode
 * @param xTicksToCatchUp Number of ticks to catch up
 * @return BaseType_t - pdTRUE if catch up was successful
 */
BaseType_t freertos_rs_task_catch_up_ticks(TickType_t xTicksToCatchUp)
{
    return xTaskCatchUpTicks(xTicksToCatchUp);
}
#endif

/**
 * @brief Wrapper for vTaskResetState()
 * Resets the task state
 */
void freertos_rs_task_reset_state(void)
{
    vTaskResetState();
}

/**
 * @brief Wrapper for ulTaskGenericNotifyValueClear()
 * Clears specific bits in a task notification value
 * @param xTask Handle of the task
 * @param uxIndexToClear Index of the notification to clear
 * @param ulBitsToClear Bits to clear
 * @return uint32_t - Previous notification value
 */
uint32_t freertos_rs_task_generic_notify_value_clear(TaskHandle_t xTask, UBaseType_t uxIndexToClear, uint32_t ulBitsToClear)
{
    return ulTaskGenericNotifyValueClear(xTask, uxIndexToClear, ulBitsToClear);
}

#if ((configUSE_STATS_FORMATTING_FUNCTIONS > 0) && (configUSE_TRACE_FACILITY == 1))
/**
 * @brief Wrapper for vTaskListTasks()
 * Generates a human readable table of task states with buffer length
 * @param pcWriteBuffer Buffer to write the table to
 * @param uxBufferLength Length of the buffer
 */
void freertos_rs_task_list_tasks(char *pcWriteBuffer, size_t uxBufferLength)
{
    vTaskListTasks(pcWriteBuffer, uxBufferLength);
}
#endif

#if ((configGENERATE_RUN_TIME_STATS == 1) && (configUSE_STATS_FORMATTING_FUNCTIONS > 0) && (configUSE_TRACE_FACILITY == 1))
/**
 * @brief Wrapper for vTaskGetRunTimeStatistics()
 * Generates a human readable table of run time stats with buffer length
 * @param pcWriteBuffer Buffer to write the table to
 * @param uxBufferLength Length of the buffer
 */
void freertos_rs_task_get_run_time_statistics(char *pcWriteBuffer, size_t uxBufferLength)
{
    vTaskGetRunTimeStatistics(pcWriteBuffer, uxBufferLength);
}
#endif

/*===========================================================================
 * CRITICAL SECTION FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for taskENTER_CRITICAL()
 * Enters a critical section
 */
void freertos_rs_task_enter_critical(void)
{
    taskENTER_CRITICAL();
}

/**
 * @brief Wrapper for taskEXIT_CRITICAL()
 * Exits a critical section
 */
void freertos_rs_task_exit_critical(void)
{
    taskEXIT_CRITICAL();
}

/**
 * @brief Wrapper for taskENTER_CRITICAL_FROM_ISR()
 * Enters a critical section from ISR
 * @return UBaseType_t - Saved interrupt status
 */
UBaseType_t freertos_rs_task_enter_critical_from_isr(void)
{
    return taskENTER_CRITICAL_FROM_ISR();
}

/**
 * @brief Wrapper for taskEXIT_CRITICAL_FROM_ISR()
 * Exits a critical section from ISR
 * @param uxSavedInterruptStatus Saved interrupt status
 */
void freertos_rs_task_exit_critical_from_isr(UBaseType_t uxSavedInterruptStatus)
{
    taskEXIT_CRITICAL_FROM_ISR(uxSavedInterruptStatus);
}

/**
 * @brief Wrapper for taskDISABLE_INTERRUPTS()
 * Disables interrupts
 */
void freertos_rs_task_disable_interrupts(void)
{
    taskDISABLE_INTERRUPTS();
}

/**
 * @brief Wrapper for taskENABLE_INTERRUPTS()
 * Enables interrupts
 */
void freertos_rs_task_enable_interrupts(void)
{
    taskENABLE_INTERRUPTS();
}

/*===========================================================================
 * MEMORY MANAGEMENT FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for pvPortMalloc()
 * Allocates memory from the FreeRTOS heap
 * @param xWantedSize Size of memory to allocate in bytes
 * @return void* - Pointer to allocated memory, or NULL if allocation failed
 */
void* freertos_rs_port_malloc(size_t xWantedSize)
{
    return pvPortMalloc(xWantedSize);
}

/**
 * @brief Wrapper for vPortFree()
 * Frees memory previously allocated with pvPortMalloc
 * @param pv Pointer to memory to free
 */
void freertos_rs_port_free(void* pv)
{
    vPortFree(pv);
}

/**
 * @brief Wrapper for xPortGetFreeHeapSize()
 * Gets the amount of free heap space available
 * @return size_t - Number of free bytes in the heap
 */
size_t freertos_rs_port_get_free_heap_size(void)
{
    return xPortGetFreeHeapSize();
}

/**
 * @brief Wrapper for xPortGetMinimumEverFreeHeapSize()
 * Gets the minimum amount of free heap space that has ever existed
 * @return size_t - Minimum number of free bytes that has ever existed
 */
size_t freertos_rs_port_get_minimum_ever_free_heap_size(void)
{
    return xPortGetMinimumEverFreeHeapSize();
}

/*===========================================================================
 * SYSTEM UTILITY FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for pdMS_TO_TICKS macro
 * Converts milliseconds to ticks
 * @param xTimeInMs Time in milliseconds
 * @return TickType_t - Time in ticks
 */
TickType_t freertos_rs_ms_to_ticks(const TickType_t xTimeInMs)
{
    return pdMS_TO_TICKS(xTimeInMs);
}

/**
 * @brief Wrapper for pdTICKS_TO_MS macro
 * Converts ticks to milliseconds
 * @param xTimeInTicks Time in ticks
 * @return TickType_t - Time in milliseconds
 */
TickType_t freertos_rs_ticks_to_ms(const TickType_t xTimeInTicks)
{
    return pdTICKS_TO_MS(xTimeInTicks);
}

/**
 * @brief Get pdTRUE constant value
 * @return BaseType_t - pdTRUE value
 */
BaseType_t freertos_rs_get_pd_true(void)
{
    return pdTRUE;
}

/**
 * @brief Get pdFALSE constant value
 * @return BaseType_t - pdFALSE value
 */
BaseType_t freertos_rs_get_pd_false(void)
{
    return pdFALSE;
}

/**
 * @brief Get pdPASS constant value
 * @return BaseType_t - pdPASS value
 */
BaseType_t freertos_rs_get_pd_pass(void)
{
    return pdPASS;
}

/**
 * @brief Get pdFAIL constant value
 * @return BaseType_t - pdFAIL value
 */
BaseType_t freertos_rs_get_pd_fail(void)
{
    return pdFAIL;
}

/**
 * @brief Wrapper for portMAX_DELAY constant
 * Gets the portMAX_DELAY constant value
 * @return TickType_t - portMAX_DELAY value
 */
TickType_t freertos_rs_get_port_max_delay(void)
{
    return portMAX_DELAY;
}

/**
 * @brief Wrapper for portTICK_PERIOD_MS constant
 * Gets the portTICK_PERIOD_MS constant value
 * @return TickType_t - portTICK_PERIOD_MS value
 */
TickType_t freertos_rs_get_port_tick_period_ms(void)
{
    return portTICK_PERIOD_MS;
}

/*===========================================================================
 * QUEUE MANAGEMENT FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for xQueueCreate()
 * Creates a new queue
 * @param uxQueueLength Maximum number of items the queue can hold
 * @param uxItemSize Size of each item in bytes
 * @return QueueHandle_t - Queue handle or NULL if failed
 */
QueueHandle_t freertos_rs_queue_create(UBaseType_t uxQueueLength, UBaseType_t uxItemSize)
{
    return xQueueCreate(uxQueueLength, uxItemSize);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xQueueCreateStatic()
 * Creates a new queue using statically allocated memory
 * @param uxQueueLength Maximum number of items the queue can hold
 * @param uxItemSize Size of each item in bytes
 * @param pucQueueStorageBuffer Storage buffer for queue items
 * @param pxQueueBuffer Queue control block buffer
 * @return QueueHandle_t - Queue handle
 */
QueueHandle_t freertos_rs_queue_create_static(UBaseType_t uxQueueLength, UBaseType_t uxItemSize, uint8_t* pucQueueStorageBuffer, void* pxQueueBuffer)
{
    return xQueueCreateStatic(uxQueueLength, uxItemSize, pucQueueStorageBuffer, (StaticQueue_t*)pxQueueBuffer);
}
#endif

/**
 * @brief Wrapper for vQueueDelete()
 * Deletes a queue
 * @param xQueue Queue handle
 */
void freertos_rs_queue_delete(QueueHandle_t xQueue)
{
    vQueueDelete(xQueue);
}

/**
 * @brief Wrapper for xQueueSend()
 * Sends an item to the back of a queue (backwards-compatible API)
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param xTicksToWait Ticks to wait if queue is full
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_send(QueueHandle_t xQueue, const void* pvItemToQueue, TickType_t xTicksToWait)
{
    return xQueueSend(xQueue, pvItemToQueue, xTicksToWait);
}

/**
 * @brief Wrapper for xQueueSendToFront()
 * Sends an item to the front of a queue
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param xTicksToWait Ticks to wait if queue is full
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_send_to_front(QueueHandle_t xQueue, const void* pvItemToQueue, TickType_t xTicksToWait)
{
    return xQueueSendToFront(xQueue, pvItemToQueue, xTicksToWait);
}

/**
 * @brief Wrapper for xQueueSendToBack()
 * Sends an item to the back of a queue
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param xTicksToWait Ticks to wait if queue is full
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_send_to_back(QueueHandle_t xQueue, const void* pvItemToQueue, TickType_t xTicksToWait)
{
    return xQueueSendToBack(xQueue, pvItemToQueue, xTicksToWait);
}

/**
 * @brief Wrapper for xQueueReceive()
 * Receives an item from a queue
 * @param xQueue Queue handle
 * @param pvBuffer Buffer to receive item into
 * @param xTicksToWait Ticks to wait if queue is empty
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_receive(QueueHandle_t xQueue, void* pvBuffer, TickType_t xTicksToWait)
{
    return xQueueReceive(xQueue, pvBuffer, xTicksToWait);
}

/**
 * @brief Wrapper for xQueuePeek()
 * Peeks at an item in a queue without removing it
 * @param xQueue Queue handle
 * @param pvBuffer Buffer to copy item into
 * @param xTicksToWait Ticks to wait if queue is empty
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_peek(QueueHandle_t xQueue, void* pvBuffer, TickType_t xTicksToWait)
{
    return xQueuePeek(xQueue, pvBuffer, xTicksToWait);
}

/**
 * @brief Wrapper for xQueueSendFromISR()
 * Sends an item to the back of a queue from an ISR (backwards-compatible API)
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_send_from_isr(QueueHandle_t xQueue, const void* pvItemToQueue, BaseType_t* pxHigherPriorityTaskWoken)
{
    return xQueueSendFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xQueueSendToFrontFromISR()
 * Sends an item to the front of a queue from an ISR
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_send_to_front_from_isr(QueueHandle_t xQueue, const void* pvItemToQueue, BaseType_t* pxHigherPriorityTaskWoken)
{
    return xQueueSendToFrontFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xQueueSendToBackFromISR()
 * Sends an item to the back of a queue from an ISR
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_send_to_back_from_isr(QueueHandle_t xQueue, const void* pvItemToQueue, BaseType_t* pxHigherPriorityTaskWoken)
{
    return xQueueSendToBackFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xQueueGiveFromISR()
 * Gives to a queue from an ISR without copying data (semaphore use case)
 * @param xQueue Queue handle (must have item size 0, i.e. used as semaphore)
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdTRUE if a context switch is needed
 */
BaseType_t freertos_rs_queue_give_from_isr(QueueHandle_t xQueue, BaseType_t* pxHigherPriorityTaskWoken)
{
    return xQueueGiveFromISR(xQueue, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xQueueReceiveFromISR()
 * Receives an item from a queue from an ISR
 * @param xQueue Queue handle
 * @param pvBuffer Buffer to receive item into
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_receive_from_isr(QueueHandle_t xQueue, void* pvBuffer, BaseType_t* pxHigherPriorityTaskWoken)
{
    return xQueueReceiveFromISR(xQueue, pvBuffer, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for uxQueueMessagesWaiting()
 * Returns the number of messages waiting in a queue
 * @param xQueue Queue handle
 * @return UBaseType_t - Number of messages waiting
 */
UBaseType_t freertos_rs_queue_messages_waiting(QueueHandle_t xQueue)
{
    return uxQueueMessagesWaiting(xQueue);
}

/**
 * @brief Wrapper for uxQueueMessagesWaitingFromISR()
 * Returns the number of messages waiting in a queue from an ISR
 * @param xQueue Queue handle
 * @return UBaseType_t - Number of messages waiting
 */
UBaseType_t freertos_rs_queue_messages_waiting_from_isr(QueueHandle_t xQueue)
{
    return uxQueueMessagesWaitingFromISR(xQueue);
}

/**
 * @brief Wrapper for xQueueIsQueueEmptyFromISR()
 * Checks if a queue is empty from an ISR
 * @param xQueue Queue handle
 * @return BaseType_t - pdTRUE if queue is empty
 */
BaseType_t freertos_rs_queue_is_queue_empty_from_isr(QueueHandle_t xQueue)
{
    return xQueueIsQueueEmptyFromISR(xQueue);
}

/**
 * @brief Wrapper for xQueueIsQueueFullFromISR()
 * Checks if a queue is full from an ISR
 * @param xQueue Queue handle
 * @return BaseType_t - pdTRUE if queue is full
 */
BaseType_t freertos_rs_queue_is_queue_full_from_isr(QueueHandle_t xQueue)
{
    return xQueueIsQueueFullFromISR(xQueue);
}

/**
 * @brief Wrapper for uxQueueSpacesAvailable()
 * Returns the number of free spaces in a queue
 * @param xQueue Queue handle
 * @return UBaseType_t - Number of free spaces
 */
UBaseType_t freertos_rs_queue_spaces_available(QueueHandle_t xQueue)
{
    return uxQueueSpacesAvailable(xQueue);
}

/**
 * @brief Wrapper for xQueueReset()
 * Resets a queue to its empty state
 * @param xQueue Queue handle
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_reset(QueueHandle_t xQueue)
{
    return xQueueReset(xQueue);
}

/**
 * @brief Wrapper for xQueueOverwrite()
 * Overwrites an item in a queue (queue must have length 1)
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to overwrite with
 * @return BaseType_t - Always pdPASS
 */
BaseType_t freertos_rs_queue_overwrite(QueueHandle_t xQueue, const void* pvItemToQueue)
{
    return xQueueOverwrite(xQueue, pvItemToQueue);
}

/**
 * @brief Wrapper for xQueueOverwriteFromISR()
 * Overwrites an item in a queue from an ISR
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to overwrite with
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - Always pdPASS
 */
BaseType_t freertos_rs_queue_overwrite_from_isr(QueueHandle_t xQueue, const void* pvItemToQueue, BaseType_t* pxHigherPriorityTaskWoken)
{
    return xQueueOverwriteFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xQueueGenericSend()
 * Generic queue send function
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param xTicksToWait Ticks to wait if queue is full
 * @param xCopyPosition Position to copy to (front, back, or overwrite)
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_generic_send(QueueHandle_t xQueue, const void* pvItemToQueue, TickType_t xTicksToWait, BaseType_t xCopyPosition)
{
    return xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, xCopyPosition);
}

/**
 * @brief Wrapper for xQueueGenericSendFromISR()
 * Generic queue send function from ISR
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @param xCopyPosition Position to copy to (front, back, or overwrite)
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_generic_send_from_isr(QueueHandle_t xQueue, const void* pvItemToQueue, BaseType_t* pxHigherPriorityTaskWoken, BaseType_t xCopyPosition)
{
    return xQueueGenericSendFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken, xCopyPosition);
}

#if (configUSE_MUTEXES == 1)
/**
 * @brief Wrapper for xQueueCreateMutex()
 * Creates a mutex (internal function)
 * @param ucQueueType Queue type
 * @return QueueHandle_t - Mutex handle or NULL if failed
 */
QueueHandle_t freertos_rs_queue_create_mutex(const uint8_t ucQueueType)
{
    return xQueueCreateMutex(ucQueueType);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xQueueCreateMutexStatic()
 * Creates a mutex using static allocation (internal function)
 * @param ucQueueType Queue type
 * @param pxStaticQueue Static queue buffer
 * @return QueueHandle_t - Mutex handle
 */
QueueHandle_t freertos_rs_queue_create_mutex_static(const uint8_t ucQueueType, void* pxStaticQueue)
{
    return xQueueCreateMutexStatic(ucQueueType, (StaticQueue_t*)pxStaticQueue);
}
#endif

#if (INCLUDE_xSemaphoreGetMutexHolder == 1)
/**
 * @brief Wrapper for xQueueGetMutexHolder()
 * Gets the holder of a mutex
 * @param xSemaphore Mutex handle
 * @return TaskHandle_t - Task handle of mutex holder
 */
TaskHandle_t freertos_rs_queue_get_mutex_holder(QueueHandle_t xSemaphore)
{
    return xQueueGetMutexHolder(xSemaphore);
}

/**
 * @brief Wrapper for xQueueGetMutexHolderFromISR()
 * Gets the holder of a mutex from ISR
 * @param xSemaphore Mutex handle
 * @return TaskHandle_t - Task handle of mutex holder
 */
TaskHandle_t freertos_rs_queue_get_mutex_holder_from_isr(QueueHandle_t xSemaphore)
{
    return xQueueGetMutexHolderFromISR(xSemaphore);
}
#endif
#endif

/**
 * @brief Wrapper for xQueuePeekFromISR()
 * Peeks at an item in a queue from an ISR
 * @param xQueue Queue handle
 * @param pvBuffer Buffer to receive the item
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_queue_peek_from_isr(QueueHandle_t xQueue, void *pvBuffer)
{
    return xQueuePeekFromISR(xQueue, pvBuffer);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xQueueGetStaticBuffers()
 * Gets the static buffers associated with a queue
 * @param xQueue Queue handle
 * @param ppucQueueStorage Pointer to receive queue storage buffer pointer
 * @param ppxStaticQueue Pointer to receive static queue structure pointer
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_queue_get_static_buffers(QueueHandle_t xQueue, uint8_t **ppucQueueStorage, void **ppxStaticQueue)
{
    return xQueueGenericGetStaticBuffers(xQueue, ppucQueueStorage, (StaticQueue_t**)ppxStaticQueue);
}
#endif

/**
 * @brief Wrapper for uxQueueGetQueueItemSize()
 * Gets the size of items in a queue
 * @param xQueue Queue handle
 * @return UBaseType_t - Size of queue items in bytes
 */
UBaseType_t freertos_rs_queue_get_queue_item_size(QueueHandle_t xQueue)
{
    return uxQueueGetQueueItemSize(xQueue);
}

/**
 * @brief Wrapper for uxQueueGetQueueLength()
 * Gets the length of a queue
 * @param xQueue Queue handle
 * @return UBaseType_t - Maximum number of items the queue can hold
 */
UBaseType_t freertos_rs_queue_get_queue_length(QueueHandle_t xQueue)
{
    return uxQueueGetQueueLength(xQueue);
}

#if (configQUEUE_REGISTRY_SIZE > 0)
/**
 * @brief Wrapper for vQueueAddToRegistry()
 * Adds a queue to the registry
 * @param xQueue Handle of the queue
 * @param pcQueueName Name to assign to the queue
 */
void freertos_rs_queue_add_to_registry(QueueHandle_t xQueue, const char *pcQueueName)
{
    vQueueAddToRegistry(xQueue, pcQueueName);
}

/**
 * @brief Wrapper for vQueueUnregisterQueue()
 * Removes a queue from the registry
 * @param xQueue Handle of the queue
 */
void freertos_rs_queue_unregister_queue(QueueHandle_t xQueue)
{
    vQueueUnregisterQueue(xQueue);
}

/**
 * @brief Wrapper for pcQueueGetName()
 * Gets the name of a queue
 * @param xQueue Handle of the queue
 * @return const char* - Name of the queue
 */
const char* freertos_rs_queue_get_name(QueueHandle_t xQueue)
{
    return pcQueueGetName(xQueue);
}
#endif

#if (configUSE_TRACE_FACILITY == 1)
/**
 * @brief Wrapper for vQueueSetQueueNumber()
 * Sets the queue number for tracing
 * @param xQueue Queue handle
 * @param uxQueueNumber Queue number to set
 */
void freertos_rs_queue_set_queue_number(QueueHandle_t xQueue, UBaseType_t uxQueueNumber)
{
    vQueueSetQueueNumber(xQueue, uxQueueNumber);
}

/**
 * @brief Wrapper for uxQueueGetQueueNumber()
 * Gets the queue number for tracing
 * @param xQueue Queue handle
 * @return UBaseType_t - Queue number
 */
UBaseType_t freertos_rs_queue_get_queue_number(QueueHandle_t xQueue)
{
    return uxQueueGetQueueNumber(xQueue);
}

/**
 * @brief Wrapper for ucQueueGetQueueType()
 * Gets the type of a queue
 * @param xQueue Queue handle
 * @return uint8_t - Queue type
 */
uint8_t freertos_rs_queue_get_queue_type(QueueHandle_t xQueue)
{
    return ucQueueGetQueueType(xQueue);
}
#endif

/*===========================================================================
 * SEMAPHORE AND MUTEX FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for xSemaphoreCreateBinary()
 * Creates a binary semaphore
 * @return SemaphoreHandle_t - Semaphore handle or NULL if failed
 */
SemaphoreHandle_t freertos_rs_semaphore_create_binary(void)
{
    return xSemaphoreCreateBinary();
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xSemaphoreCreateBinaryStatic()
 * Creates a binary semaphore using statically allocated memory
 * @param pxSemaphoreBuffer Semaphore buffer
 * @return SemaphoreHandle_t - Semaphore handle
 */
SemaphoreHandle_t freertos_rs_semaphore_create_binary_static(void* pxSemaphoreBuffer)
{
    return xSemaphoreCreateBinaryStatic((StaticSemaphore_t*)pxSemaphoreBuffer);
}
#endif

/**
 * @brief Wrapper for vSemaphoreDelete()
 * Deletes a semaphore
 * @param xSemaphore Semaphore handle
 */
void freertos_rs_semaphore_delete(SemaphoreHandle_t xSemaphore)
{
    vSemaphoreDelete(xSemaphore);
}

#if (configUSE_COUNTING_SEMAPHORES == 1)
/**
 * @brief Wrapper for xSemaphoreCreateCounting()
 * Creates a counting semaphore
 * @param uxMaxCount Maximum count value
 * @param uxInitialCount Initial count value
 * @return SemaphoreHandle_t - Semaphore handle or NULL if failed
 */
SemaphoreHandle_t freertos_rs_semaphore_create_counting(UBaseType_t uxMaxCount, UBaseType_t uxInitialCount)
{
    return xSemaphoreCreateCounting(uxMaxCount, uxInitialCount);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xSemaphoreCreateCountingStatic()
 * Creates a counting semaphore using statically allocated memory
 * @param uxMaxCount Maximum count value
 * @param uxInitialCount Initial count value
 * @param pxSemaphoreBuffer Semaphore buffer
 * @return SemaphoreHandle_t - Semaphore handle
 */
SemaphoreHandle_t freertos_rs_semaphore_create_counting_static(UBaseType_t uxMaxCount, UBaseType_t uxInitialCount, void* pxSemaphoreBuffer)
{
    return xSemaphoreCreateCountingStatic(uxMaxCount, uxInitialCount, (StaticSemaphore_t*)pxSemaphoreBuffer);
}
#endif
#endif

#if (configUSE_MUTEXES == 1)
/**
 * @brief Wrapper for xSemaphoreCreateMutex()
 * Creates a mutex
 * @return SemaphoreHandle_t - Mutex handle or NULL if failed
 */
SemaphoreHandle_t freertos_rs_semaphore_create_mutex(void)
{
    return xSemaphoreCreateMutex();
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xSemaphoreCreateMutexStatic()
 * Creates a mutex using statically allocated memory
 * @param pxMutexBuffer Mutex buffer
 * @return SemaphoreHandle_t - Mutex handle
 */
SemaphoreHandle_t freertos_rs_semaphore_create_mutex_static(void* pxMutexBuffer)
{
    return xSemaphoreCreateMutexStatic((StaticSemaphore_t*)pxMutexBuffer);
}
#endif

#if (configUSE_RECURSIVE_MUTEXES == 1)
/**
 * @brief Wrapper for xSemaphoreCreateRecursiveMutex()
 * Creates a recursive mutex
 * @return SemaphoreHandle_t - Mutex handle or NULL if failed
 */
SemaphoreHandle_t freertos_rs_semaphore_create_recursive_mutex(void)
{
    return xSemaphoreCreateRecursiveMutex();
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xSemaphoreCreateRecursiveMutexStatic()
 * Creates a recursive mutex using statically allocated memory
 * @param pxMutexBuffer Mutex buffer
 * @return SemaphoreHandle_t - Mutex handle
 */
SemaphoreHandle_t freertos_rs_semaphore_create_recursive_mutex_static(void* pxMutexBuffer)
{
    return xSemaphoreCreateRecursiveMutexStatic((StaticSemaphore_t*)pxMutexBuffer);
}
#endif
#endif
#endif

/**
 * @brief Wrapper for xSemaphoreTake()
 * Takes (acquires) a semaphore
 * @param xSemaphore Semaphore handle
 * @param xTicksToWait Ticks to wait if semaphore not available
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_semaphore_take(SemaphoreHandle_t xSemaphore, TickType_t xTicksToWait)
{
    return xSemaphoreTake(xSemaphore, xTicksToWait);
}

/**
 * @brief Wrapper for xSemaphoreGive()
 * Gives (releases) a semaphore
 * @param xSemaphore Semaphore handle
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_semaphore_give(SemaphoreHandle_t xSemaphore)
{
    return xSemaphoreGive(xSemaphore);
}

#if (configUSE_RECURSIVE_MUTEXES == 1)
/**
 * @brief Wrapper for xSemaphoreTakeRecursive()
 * Takes a recursive mutex
 * @param xMutex Mutex handle
 * @param xTicksToWait Ticks to wait if mutex not available
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_semaphore_take_recursive(SemaphoreHandle_t xMutex, TickType_t xTicksToWait)
{
    return xSemaphoreTakeRecursive(xMutex, xTicksToWait);
}

/**
 * @brief Wrapper for xSemaphoreGiveRecursive()
 * Gives a recursive mutex
 * @param xMutex Mutex handle
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_semaphore_give_recursive(SemaphoreHandle_t xMutex)
{
    return xSemaphoreGiveRecursive(xMutex);
}
#endif

/**
 * @brief Wrapper for xSemaphoreTakeFromISR()
 * Takes a semaphore from an ISR
 * @param xSemaphore Semaphore handle
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_semaphore_take_from_isr(SemaphoreHandle_t xSemaphore, BaseType_t* pxHigherPriorityTaskWoken)
{
    return xSemaphoreTakeFromISR(xSemaphore, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xSemaphoreGiveFromISR()
 * Gives a semaphore from an ISR
 * @param xSemaphore Semaphore handle
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_semaphore_give_from_isr(SemaphoreHandle_t xSemaphore, BaseType_t* pxHigherPriorityTaskWoken)
{
    return xSemaphoreGiveFromISR(xSemaphore, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for uxSemaphoreGetCount()
 * Gets the count value of a semaphore
 * @param xSemaphore Semaphore handle
 * @return UBaseType_t - Count value
 */
UBaseType_t freertos_rs_semaphore_get_count(SemaphoreHandle_t xSemaphore)
{
    return uxSemaphoreGetCount(xSemaphore);
}

/**
 * @brief Wrapper for uxSemaphoreGetCountFromISR()
 * Gets the count value of a semaphore from an ISR
 * @param xSemaphore Semaphore handle
 * @return UBaseType_t - Count value
 */
UBaseType_t freertos_rs_semaphore_get_count_from_isr(SemaphoreHandle_t xSemaphore)
{
    return uxSemaphoreGetCountFromISR(xSemaphore);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xSemaphoreGetStaticBuffer()
 * Gets the static buffer associated with a semaphore
 * @param xSemaphore Semaphore handle
 * @param ppxSemaphoreBuffer Pointer to receive semaphore buffer pointer
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_semaphore_get_static_buffer(SemaphoreHandle_t xSemaphore, StaticSemaphore_t **ppxSemaphoreBuffer)
{
    return xSemaphoreGetStaticBuffer(xSemaphore, ppxSemaphoreBuffer);
}
#endif

#if (INCLUDE_xSemaphoreGetMutexHolder == 1)
/**
 * @brief Wrapper for xSemaphoreGetMutexHolder()
 * Gets the task that currently holds a mutex
 * @param xSemaphore Mutex handle
 * @return TaskHandle_t - Handle of the task holding the mutex, or NULL
 */
TaskHandle_t freertos_rs_semaphore_get_mutex_holder(SemaphoreHandle_t xSemaphore)
{
    return xSemaphoreGetMutexHolder(xSemaphore);
}

/**
 * @brief Wrapper for xSemaphoreGetMutexHolderFromISR()
 * Gets the task that currently holds a mutex from ISR
 * @param xSemaphore Mutex handle
 * @return TaskHandle_t - Handle of the task holding the mutex, or NULL
 */
TaskHandle_t freertos_rs_semaphore_get_mutex_holder_from_isr(SemaphoreHandle_t xSemaphore)
{
    return xSemaphoreGetMutexHolderFromISR(xSemaphore);
}
#endif

/*===========================================================================
 * EVENT GROUP FUNCTIONS
 *===========================================================================*/

#if (configSUPPORT_DYNAMIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xEventGroupCreate()
 * Creates an event group
 * @return EventGroupHandle_t - Handle to the created event group
 */
EventGroupHandle_t freertos_rs_event_group_create(void)
{
    return xEventGroupCreate();
}
#endif

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xEventGroupCreateStatic()
 * Creates an event group using statically allocated memory
 * @param pxEventGroupBuffer Pointer to event group buffer
 * @return EventGroupHandle_t - Handle to the created event group
 */
EventGroupHandle_t freertos_rs_event_group_create_static(StaticEventGroup_t *pxEventGroupBuffer)
{
    return xEventGroupCreateStatic(pxEventGroupBuffer);
}
#endif

/**
 * @brief Wrapper for vEventGroupDelete()
 * Deletes an event group
 * @param xEventGroup Event group handle
 */
void freertos_rs_event_group_delete(EventGroupHandle_t xEventGroup)
{
    vEventGroupDelete(xEventGroup);
}

/**
 * @brief Wrapper for xEventGroupSetBits()
 * Sets bits in an event group
 * @param xEventGroup Event group handle
 * @param uxBitsToSet Bits to set
 * @return EventBits_t - Value of event bits before setting
 */
EventBits_t freertos_rs_event_group_set_bits(EventGroupHandle_t xEventGroup, const EventBits_t uxBitsToSet)
{
    return xEventGroupSetBits(xEventGroup, uxBitsToSet);
}

/**
 * @brief Wrapper for xEventGroupClearBits()
 * Clears bits in an event group
 * @param xEventGroup Event group handle
 * @param uxBitsToClear Bits to clear
 * @return EventBits_t - Value of event bits before clearing
 */
EventBits_t freertos_rs_event_group_clear_bits(EventGroupHandle_t xEventGroup, const EventBits_t uxBitsToClear)
{
    return xEventGroupClearBits(xEventGroup, uxBitsToClear);
}

/**
 * @brief Wrapper for xEventGroupGetBits()
 * Gets the current value of the event group bits
 * @param xEventGroup Event group handle
 * @return EventBits_t - Current value of event bits
 */
EventBits_t freertos_rs_event_group_get_bits(EventGroupHandle_t xEventGroup)
{
    return xEventGroupGetBits(xEventGroup);
}

/**
 * @brief Wrapper for xEventGroupWaitBits()
 * Waits for bits to be set in an event group
 * @param xEventGroup Event group handle
 * @param uxBitsToWaitFor Bits to wait for
 * @param xClearOnExit Clear bits on exit
 * @param xWaitForAllBits Wait for all bits or any bit
 * @param xTicksToWait Ticks to wait
 * @return EventBits_t - Value of event bits when condition was met
 */
EventBits_t freertos_rs_event_group_wait_bits(EventGroupHandle_t xEventGroup, const EventBits_t uxBitsToWaitFor, BaseType_t xClearOnExit, BaseType_t xWaitForAllBits, TickType_t xTicksToWait)
{
    return xEventGroupWaitBits(xEventGroup, uxBitsToWaitFor, xClearOnExit, xWaitForAllBits, xTicksToWait);
}

/**
 * @brief Wrapper for xEventGroupSync()
 * Synchronizes tasks using an event group
 * @param xEventGroup Event group handle
 * @param uxBitsToSet Bits to set
 * @param uxBitsToWaitFor Bits to wait for
 * @param xTicksToWait Ticks to wait
 * @return EventBits_t - Value of event bits when condition was met
 */
EventBits_t freertos_rs_event_group_sync(EventGroupHandle_t xEventGroup, const EventBits_t uxBitsToSet, const EventBits_t uxBitsToWaitFor, TickType_t xTicksToWait)
{
    return xEventGroupSync(xEventGroup, uxBitsToSet, uxBitsToWaitFor, xTicksToWait);
}

#if (configUSE_TRACE_FACILITY == 1)
/**
 * @brief Wrapper for xEventGroupSetBitsFromISR()
 * Sets bits in an event group from an ISR
 * @param xEventGroup Event group handle
 * @param uxBitsToSet Bits to set
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_event_group_set_bits_from_isr(EventGroupHandle_t xEventGroup, const EventBits_t uxBitsToSet, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xEventGroupSetBitsFromISR(xEventGroup, uxBitsToSet, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xEventGroupClearBitsFromISR()
 * Clears bits in an event group from an ISR
 * @param xEventGroup Event group handle
 * @param uxBitsToClear Bits to clear
 * @return EventBits_t - Value of event bits before clearing
 */
EventBits_t freertos_rs_event_group_clear_bits_from_isr(EventGroupHandle_t xEventGroup, const EventBits_t uxBitsToClear)
{
    return xEventGroupClearBitsFromISR(xEventGroup, uxBitsToClear);
}
#endif

/**
 * @brief Wrapper for xEventGroupGetBitsFromISR()
 * Gets the current value of the event group bits from an ISR
 * @param xEventGroup Event group handle
 * @return EventBits_t - Current value of event bits
 */
EventBits_t freertos_rs_event_group_get_bits_from_isr(EventGroupHandle_t xEventGroup)
{
    return xEventGroupGetBitsFromISR(xEventGroup);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xEventGroupGetStaticBuffer()
 * Gets the static buffer associated with an event group
 * @param xEventGroup Event group handle
 * @param ppxEventGroupBuffer Pointer to receive event group buffer pointer
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_event_group_get_static_buffer(EventGroupHandle_t xEventGroup, StaticEventGroup_t **ppxEventGroupBuffer)
{
    return xEventGroupGetStaticBuffer(xEventGroup, ppxEventGroupBuffer);
}
#endif

#if (configUSE_TRACE_FACILITY == 1)
/**
 * @brief Wrapper for uxEventGroupGetNumber()
 * Gets the event group number for tracing
 * @param xEventGroup Event group handle
 * @return UBaseType_t - Event group number
 */
UBaseType_t freertos_rs_event_group_get_number(EventGroupHandle_t xEventGroup)
{
    return uxEventGroupGetNumber(xEventGroup);
}

/**
 * @brief Wrapper for vEventGroupSetNumber()
 * Sets the event group number for tracing
 * @param xEventGroup Event group handle
 * @param uxEventGroupNumber Event group number to set
 */
void freertos_rs_event_group_set_number(EventGroupHandle_t xEventGroup, UBaseType_t uxEventGroupNumber)
{
    vEventGroupSetNumber(xEventGroup, uxEventGroupNumber);
}
#endif

/*===========================================================================
 * TIMER FUNCTIONS
 *===========================================================================*/

#if (configUSE_TIMERS == 1)

#if (configSUPPORT_DYNAMIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xTimerCreate()
 * Creates a software timer
 * @param pcTimerName Name for the timer
 * @param xTimerPeriod Timer period in ticks
 * @param uxAutoReload Auto-reload flag
 * @param pvTimerID Timer ID
 * @param pxCallbackFunction Callback function
 * @return TimerHandle_t - Handle to the created timer
 */
TimerHandle_t freertos_rs_timer_create(const char * const pcTimerName, const TickType_t xTimerPeriod, const UBaseType_t uxAutoReload, void * const pvTimerID, TimerCallbackFunction_t pxCallbackFunction)
{
    return xTimerCreate(pcTimerName, xTimerPeriod, uxAutoReload, pvTimerID, pxCallbackFunction);
}
#endif

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xTimerCreateStatic()
 * Creates a software timer using statically allocated memory
 * @param pcTimerName Name for the timer
 * @param xTimerPeriod Timer period in ticks
 * @param uxAutoReload Auto-reload flag
 * @param pvTimerID Timer ID
 * @param pxCallbackFunction Callback function
 * @param pxTimerBuffer Timer buffer
 * @return TimerHandle_t - Handle to the created timer
 */
TimerHandle_t freertos_rs_timer_create_static(const char * const pcTimerName, const TickType_t xTimerPeriod, const UBaseType_t uxAutoReload, void * const pvTimerID, TimerCallbackFunction_t pxCallbackFunction, StaticTimer_t *pxTimerBuffer)
{
    return xTimerCreateStatic(pcTimerName, xTimerPeriod, uxAutoReload, pvTimerID, pxCallbackFunction, pxTimerBuffer);
}
#endif

/**
 * @brief Wrapper for xTimerDelete()
 * Deletes a software timer
 * @param xTimer Timer handle
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_delete(TimerHandle_t xTimer, TickType_t xTicksToWait)
{
    return xTimerDelete(xTimer, xTicksToWait);
}

/**
 * @brief Wrapper for xTimerStart()
 * Starts a software timer
 * @param xTimer Timer handle
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_start(TimerHandle_t xTimer, TickType_t xTicksToWait)
{
    return xTimerStart(xTimer, xTicksToWait);
}

/**
 * @brief Wrapper for xTimerStop()
 * Stops a software timer
 * @param xTimer Timer handle
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_stop(TimerHandle_t xTimer, TickType_t xTicksToWait)
{
    return xTimerStop(xTimer, xTicksToWait);
}

/**
 * @brief Wrapper for xTimerReset()
 * Resets a software timer
 * @param xTimer Timer handle
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_reset(TimerHandle_t xTimer, TickType_t xTicksToWait)
{
    return xTimerReset(xTimer, xTicksToWait);
}

/**
 * @brief Wrapper for xTimerChangePeriod()
 * Changes the period of a software timer
 * @param xTimer Timer handle
 * @param xNewPeriod New period in ticks
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_change_period(TimerHandle_t xTimer, TickType_t xNewPeriod, TickType_t xTicksToWait)
{
    return xTimerChangePeriod(xTimer, xNewPeriod, xTicksToWait);
}

/**
 * @brief Wrapper for xTimerStartFromISR()
 * Starts a software timer from an ISR
 * @param xTimer Timer handle
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_start_from_isr(TimerHandle_t xTimer, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xTimerStartFromISR(xTimer, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xTimerStopFromISR()
 * Stops a software timer from an ISR
 * @param xTimer Timer handle
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_stop_from_isr(TimerHandle_t xTimer, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xTimerStopFromISR(xTimer, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xTimerResetFromISR()
 * Resets a software timer from an ISR
 * @param xTimer Timer handle
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_reset_from_isr(TimerHandle_t xTimer, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xTimerResetFromISR(xTimer, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xTimerChangePeriodFromISR()
 * Changes the period of a software timer from an ISR
 * @param xTimer Timer handle
 * @param xNewPeriod New period in ticks
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_change_period_from_isr(TimerHandle_t xTimer, TickType_t xNewPeriod, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xTimerChangePeriodFromISR(xTimer, xNewPeriod, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xTimerIsTimerActive()
 * Checks if a timer is active
 * @param xTimer Timer handle
 * @return BaseType_t - pdTRUE if active
 */
BaseType_t freertos_rs_timer_is_timer_active(TimerHandle_t xTimer)
{
    return xTimerIsTimerActive(xTimer);
}

/**
 * @brief Wrapper for xTimerGetTimerDaemonTaskHandle()
 * Gets the handle of the timer daemon task
 * @return TaskHandle_t - Handle of the timer daemon task
 */
TaskHandle_t freertos_rs_timer_get_timer_daemon_task_handle(void)
{
    return xTimerGetTimerDaemonTaskHandle();
}

/**
 * @brief Wrapper for xTimerGetPeriod()
 * Gets the period of a timer
 * @param xTimer Timer handle
 * @return TickType_t - Timer period
 */
TickType_t freertos_rs_timer_get_period(TimerHandle_t xTimer)
{
    return xTimerGetPeriod(xTimer);
}

/**
 * @brief Wrapper for xTimerGetExpiryTime()
 * Gets the expiry time of a timer
 * @param xTimer Timer handle
 * @return TickType_t - Timer expiry time
 */
TickType_t freertos_rs_timer_get_expiry_time(TimerHandle_t xTimer)
{
    return xTimerGetExpiryTime(xTimer);
}

/**
 * @brief Wrapper for pcTimerGetName()
 * Gets the name of a timer
 * @param xTimer Timer handle
 * @return const char* - Timer name
 */
const char* freertos_rs_timer_get_name(TimerHandle_t xTimer)
{
    return pcTimerGetName(xTimer);
}

/**
 * @brief Wrapper for pvTimerGetTimerID()
 * Gets the ID of a timer
 * @param xTimer Timer handle
 * @return void* - Timer ID
 */
void* freertos_rs_timer_get_timer_id(TimerHandle_t xTimer)
{
    return pvTimerGetTimerID(xTimer);
}

/**
 * @brief Wrapper for vTimerSetTimerID()
 * Sets the ID of a timer
 * @param xTimer Timer handle
 * @param pvNewID New timer ID
 */
void freertos_rs_timer_set_timer_id(TimerHandle_t xTimer, void *pvNewID)
{
    vTimerSetTimerID(xTimer, pvNewID);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xTimerGetStaticBuffer()
 * Gets the static buffer associated with a timer
 * @param xTimer Timer handle
 * @param ppxTimerBuffer Pointer to receive timer buffer pointer
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_timer_get_static_buffer(TimerHandle_t xTimer, StaticTimer_t **ppxTimerBuffer)
{
    return xTimerGetStaticBuffer(xTimer, ppxTimerBuffer);
}
#endif

#if (configUSE_TRACE_FACILITY == 1)
/**
 * @brief Wrapper for uxTimerGetTimerNumber()
 * Gets the timer number for tracing
 * @param xTimer Timer handle
 * @return UBaseType_t - Timer number
 */
UBaseType_t freertos_rs_timer_get_timer_number(TimerHandle_t xTimer)
{
    return uxTimerGetTimerNumber(xTimer);
}

/**
 * @brief Wrapper for vTimerSetTimerNumber()
 * Sets the timer number for tracing
 * @param xTimer Timer handle
 * @param uxTimerNumber Timer number to set
 */
void freertos_rs_timer_set_timer_number(TimerHandle_t xTimer, UBaseType_t uxTimerNumber)
{
    vTimerSetTimerNumber(xTimer, uxTimerNumber);
}
#endif

/**
 * @brief Wrapper for xTimerPendFunctionCall()
 * Pends a function call to be executed by the timer daemon task
 * @param xFunctionToPend Function to execute
 * @param pvParameter1 First parameter
 * @param ulParameter2 Second parameter
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_pend_function_call(PendedFunction_t xFunctionToPend, void *pvParameter1, uint32_t ulParameter2, TickType_t xTicksToWait)
{
    return xTimerPendFunctionCall(xFunctionToPend, pvParameter1, ulParameter2, xTicksToWait);
}

/**
 * @brief Wrapper for xTimerPendFunctionCallFromISR()
 * Pends a function call from an ISR to be executed by the timer daemon task
 * @param xFunctionToPend Function to execute
 * @param pvParameter1 First parameter
 * @param ulParameter2 Second parameter
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_pend_function_call_from_isr(PendedFunction_t xFunctionToPend, void *pvParameter1, uint32_t ulParameter2, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xTimerPendFunctionCallFromISR(xFunctionToPend, pvParameter1, ulParameter2, pxHigherPriorityTaskWoken);
}

#endif /* configUSE_TIMERS */

/*===========================================================================
 * STREAM BUFFER FUNCTIONS
 *===========================================================================*/

#if (configUSE_STREAM_BUFFERS == 1)

#if (configSUPPORT_DYNAMIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xStreamBufferCreate()
 * Creates a stream buffer
 * @param xBufferSizeBytes Size of the buffer in bytes
 * @param xTriggerLevelBytes Trigger level in bytes
 * @return StreamBufferHandle_t - Handle to the created stream buffer
 */
StreamBufferHandle_t freertos_rs_stream_buffer_create(size_t xBufferSizeBytes, size_t xTriggerLevelBytes)
{
    return xStreamBufferCreate(xBufferSizeBytes, xTriggerLevelBytes);
}
#endif

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xStreamBufferCreateStatic()
 * Creates a stream buffer using statically allocated memory
 * @param xBufferSizeBytes Size of the buffer in bytes
 * @param xTriggerLevelBytes Trigger level in bytes
 * @param pucStreamBufferStorageArea Storage area for the buffer
 * @param pxStaticStreamBuffer Static stream buffer structure
 * @return StreamBufferHandle_t - Handle to the created stream buffer
 */
StreamBufferHandle_t freertos_rs_stream_buffer_create_static(size_t xBufferSizeBytes, size_t xTriggerLevelBytes, uint8_t *pucStreamBufferStorageArea, StaticStreamBuffer_t *pxStaticStreamBuffer)
{
    return xStreamBufferCreateStatic(xBufferSizeBytes, xTriggerLevelBytes, pucStreamBufferStorageArea, pxStaticStreamBuffer);
}
#endif

/**
 * @brief Wrapper for vStreamBufferDelete()
 * Deletes a stream buffer
 * @param xStreamBuffer Stream buffer handle
 */
void freertos_rs_stream_buffer_delete(StreamBufferHandle_t xStreamBuffer)
{
    vStreamBufferDelete(xStreamBuffer);
}

/**
 * @brief Wrapper for xStreamBufferSend()
 * Sends data to a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @param pvTxData Data to send
 * @param xDataLengthBytes Length of data in bytes
 * @param xTicksToWait Ticks to wait
 * @return size_t - Number of bytes sent
 */
size_t freertos_rs_stream_buffer_send(StreamBufferHandle_t xStreamBuffer, const void *pvTxData, size_t xDataLengthBytes, TickType_t xTicksToWait)
{
    return xStreamBufferSend(xStreamBuffer, pvTxData, xDataLengthBytes, xTicksToWait);
}

/**
 * @brief Wrapper for xStreamBufferReceive()
 * Receives data from a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @param pvRxData Buffer to receive data
 * @param xBufferLengthBytes Length of receive buffer
 * @param xTicksToWait Ticks to wait
 * @return size_t - Number of bytes received
 */
size_t freertos_rs_stream_buffer_receive(StreamBufferHandle_t xStreamBuffer, void *pvRxData, size_t xBufferLengthBytes, TickType_t xTicksToWait)
{
    return xStreamBufferReceive(xStreamBuffer, pvRxData, xBufferLengthBytes, xTicksToWait);
}

/**
 * @brief Wrapper for xStreamBufferReset()
 * Resets a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_stream_buffer_reset(StreamBufferHandle_t xStreamBuffer)
{
    return xStreamBufferReset(xStreamBuffer);
}

/**
 * @brief Wrapper for xStreamBufferSendFromISR()
 * Sends data to a stream buffer from an ISR
 * @param xStreamBuffer Stream buffer handle
 * @param pvTxData Data to send
 * @param xDataLengthBytes Length of data in bytes
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return size_t - Number of bytes sent
 */
size_t freertos_rs_stream_buffer_send_from_isr(StreamBufferHandle_t xStreamBuffer, const void *pvTxData, size_t xDataLengthBytes, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xStreamBufferSendFromISR(xStreamBuffer, pvTxData, xDataLengthBytes, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xStreamBufferReceiveFromISR()
 * Receives data from a stream buffer from an ISR
 * @param xStreamBuffer Stream buffer handle
 * @param pvRxData Buffer to receive data
 * @param xBufferLengthBytes Length of receive buffer
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return size_t - Number of bytes received
 */
size_t freertos_rs_stream_buffer_receive_from_isr(StreamBufferHandle_t xStreamBuffer, void *pvRxData, size_t xBufferLengthBytes, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xStreamBufferReceiveFromISR(xStreamBuffer, pvRxData, xBufferLengthBytes, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xStreamBufferIsEmpty()
 * Checks if a stream buffer is empty
 * @param xStreamBuffer Stream buffer handle
 * @return BaseType_t - pdTRUE if empty
 */
BaseType_t freertos_rs_stream_buffer_is_empty(StreamBufferHandle_t xStreamBuffer)
{
    return xStreamBufferIsEmpty(xStreamBuffer);
}

/**
 * @brief Wrapper for xStreamBufferIsFull()
 * Checks if a stream buffer is full
 * @param xStreamBuffer Stream buffer handle
 * @return BaseType_t - pdTRUE if full
 */
BaseType_t freertos_rs_stream_buffer_is_full(StreamBufferHandle_t xStreamBuffer)
{
    return xStreamBufferIsFull(xStreamBuffer);
}

/**
 * @brief Wrapper for xStreamBufferBytesAvailable()
 * Gets the number of bytes available in a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @return size_t - Number of bytes available
 */
size_t freertos_rs_stream_buffer_bytes_available(StreamBufferHandle_t xStreamBuffer)
{
    return xStreamBufferBytesAvailable(xStreamBuffer);
}

/**
 * @brief Wrapper for xStreamBufferSpacesAvailable()
 * Gets the number of free spaces available in a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @return size_t - Number of free spaces available
 */
size_t freertos_rs_stream_buffer_spaces_available(StreamBufferHandle_t xStreamBuffer)
{
    return xStreamBufferSpacesAvailable(xStreamBuffer);
}

/**
 * @brief Wrapper for xStreamBufferSetTriggerLevel()
 * Sets the trigger level of a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @param xTriggerLevel New trigger level
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_stream_buffer_set_trigger_level(StreamBufferHandle_t xStreamBuffer, size_t xTriggerLevel)
{
    return xStreamBufferSetTriggerLevel(xStreamBuffer, xTriggerLevel);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xStreamBufferGetStaticBuffers()
 * Gets the static buffers associated with a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @param ppucStreamBufferStorageArea Pointer to receive storage area pointer
 * @param ppxStaticStreamBuffer Pointer to receive static stream buffer pointer
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_stream_buffer_get_static_buffers(StreamBufferHandle_t xStreamBuffer, uint8_t **ppucStreamBufferStorageArea, StaticStreamBuffer_t **ppxStaticStreamBuffer)
{
    return xStreamBufferGetStaticBuffers(xStreamBuffer, ppucStreamBufferStorageArea, ppxStaticStreamBuffer);
}
#endif

#endif /* configUSE_STREAM_BUFFERS */

/*===========================================================================
 * MESSAGE BUFFER FUNCTIONS
 *===========================================================================*/

#if (configUSE_MESSAGE_BUFFERS == 1)

#if (configSUPPORT_DYNAMIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xMessageBufferCreate()
 * Creates a message buffer
 * @param xBufferSizeBytes Size of the buffer in bytes
 * @return MessageBufferHandle_t - Handle to the created message buffer
 */
MessageBufferHandle_t freertos_rs_message_buffer_create(size_t xBufferSizeBytes)
{
    return xMessageBufferCreate(xBufferSizeBytes);
}
#endif

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xMessageBufferCreateStatic()
 * Creates a message buffer using statically allocated memory
 * @param xBufferSizeBytes Size of the buffer in bytes
 * @param pucMessageBufferStorageArea Storage area for the buffer
 * @param pxStaticMessageBuffer Static message buffer structure
 * @return MessageBufferHandle_t - Handle to the created message buffer
 */
MessageBufferHandle_t freertos_rs_message_buffer_create_static(size_t xBufferSizeBytes, uint8_t *pucMessageBufferStorageArea, StaticMessageBuffer_t *pxStaticMessageBuffer)
{
    return xMessageBufferCreateStatic(xBufferSizeBytes, pucMessageBufferStorageArea, pxStaticMessageBuffer);
}
#endif

/**
 * @brief Wrapper for vMessageBufferDelete()
 * Deletes a message buffer
 * @param xMessageBuffer Message buffer handle
 */
void freertos_rs_message_buffer_delete(MessageBufferHandle_t xMessageBuffer)
{
    vMessageBufferDelete(xMessageBuffer);
}

/**
 * @brief Wrapper for xMessageBufferSend()
 * Sends a message to a message buffer
 * @param xMessageBuffer Message buffer handle
 * @param pvTxData Message to send
 * @param xDataLengthBytes Length of message in bytes
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_message_buffer_send(MessageBufferHandle_t xMessageBuffer, const void *pvTxData, size_t xDataLengthBytes, TickType_t xTicksToWait)
{
    return xMessageBufferSend(xMessageBuffer, pvTxData, xDataLengthBytes, xTicksToWait);
}

/**
 * @brief Wrapper for xMessageBufferReceive()
 * Receives a message from a message buffer
 * @param xMessageBuffer Message buffer handle
 * @param pvRxData Buffer to receive message
 * @param xBufferLengthBytes Length of receive buffer
 * @param xTicksToWait Ticks to wait
 * @return size_t - Length of received message
 */
size_t freertos_rs_message_buffer_receive(MessageBufferHandle_t xMessageBuffer, void *pvRxData, size_t xBufferLengthBytes, TickType_t xTicksToWait)
{
    return xMessageBufferReceive(xMessageBuffer, pvRxData, xBufferLengthBytes, xTicksToWait);
}

/**
 * @brief Wrapper for xMessageBufferReset()
 * Resets a message buffer
 * @param xMessageBuffer Message buffer handle
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_message_buffer_reset(MessageBufferHandle_t xMessageBuffer)
{
    return xMessageBufferReset(xMessageBuffer);
}

/**
 * @brief Wrapper for xMessageBufferSendFromISR()
 * Sends a message to a message buffer from an ISR
 * @param xMessageBuffer Message buffer handle
 * @param pvTxData Message to send
 * @param xDataLengthBytes Length of message in bytes
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_message_buffer_send_from_isr(MessageBufferHandle_t xMessageBuffer, const void *pvTxData, size_t xDataLengthBytes, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xMessageBufferSendFromISR(xMessageBuffer, pvTxData, xDataLengthBytes, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xMessageBufferReceiveFromISR()
 * Receives a message from a message buffer from an ISR
 * @param xMessageBuffer Message buffer handle
 * @param pvRxData Buffer to receive message
 * @param xBufferLengthBytes Length of receive buffer
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return size_t - Length of received message
 */
size_t freertos_rs_message_buffer_receive_from_isr(MessageBufferHandle_t xMessageBuffer, void *pvRxData, size_t xBufferLengthBytes, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xMessageBufferReceiveFromISR(xMessageBuffer, pvRxData, xBufferLengthBytes, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xMessageBufferIsEmpty()
 * Checks if a message buffer is empty
 * @param xMessageBuffer Message buffer handle
 * @return BaseType_t - pdTRUE if empty
 */
BaseType_t freertos_rs_message_buffer_is_empty(MessageBufferHandle_t xMessageBuffer)
{
    return xMessageBufferIsEmpty(xMessageBuffer);
}

/**
 * @brief Wrapper for xMessageBufferIsFull()
 * Checks if a message buffer is full
 * @param xMessageBuffer Message buffer handle
 * @return BaseType_t - pdTRUE if full
 */
BaseType_t freertos_rs_message_buffer_is_full(MessageBufferHandle_t xMessageBuffer)
{
    return xMessageBufferIsFull(xMessageBuffer);
}

/**
 * @brief Wrapper for xMessageBufferSpacesAvailable()
 * Gets the number of free spaces available in a message buffer
 * @param xMessageBuffer Message buffer handle
 * @return size_t - Number of free spaces available
 */
size_t freertos_rs_message_buffer_spaces_available(MessageBufferHandle_t xMessageBuffer)
{
    return xMessageBufferSpacesAvailable(xMessageBuffer);
}

/**
 * @brief Wrapper for xMessageBufferNextLengthBytes()
 * Gets the length of the next message in a message buffer
 * @param xMessageBuffer Message buffer handle
 * @return size_t - Length of next message
 */
size_t freertos_rs_message_buffer_next_length_bytes(MessageBufferHandle_t xMessageBuffer)
{
    return xMessageBufferNextLengthBytes(xMessageBuffer);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xMessageBufferGetStaticBuffers()
 * Gets the static buffers associated with a message buffer
 * @param xMessageBuffer Message buffer handle
 * @param ppucMessageBufferStorageArea Pointer to receive storage area pointer
 * @param ppxStaticMessageBuffer Pointer to receive static message buffer pointer
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_message_buffer_get_static_buffers(MessageBufferHandle_t xMessageBuffer, uint8_t **ppucMessageBufferStorageArea, StaticMessageBuffer_t **ppxStaticMessageBuffer)
{
    return xMessageBufferGetStaticBuffers(xMessageBuffer, ppucMessageBufferStorageArea, ppxStaticMessageBuffer);
}
#endif

#endif /* configUSE_MESSAGE_BUFFERS */

/*===========================================================================
 * ATOMIC OPERATION FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for Atomic_Add_u32()
 * Atomically adds to a 32-bit unsigned value
 * @param pAddend Pointer to the value to add to
 * @param ulCount Value to add
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_add_u32(uint32_t *pAddend, uint32_t ulCount)
{
    return Atomic_Add_u32(pAddend, ulCount);
}

/**
 * @brief Wrapper for Atomic_Subtract_u32()
 * Atomically subtracts from a 32-bit unsigned value
 * @param pMinuend Pointer to the value to subtract from
 * @param ulCount Value to subtract
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_subtract_u32(uint32_t *pMinuend, uint32_t ulCount)
{
    return Atomic_Subtract_u32(pMinuend, ulCount);
}

/**
 * @brief Wrapper for Atomic_Increment_u32()
 * Atomically increments a 32-bit unsigned value
 * @param pAddend Pointer to the value to increment
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_increment_u32(uint32_t *pAddend)
{
    return Atomic_Increment_u32(pAddend);
}

/**
 * @brief Wrapper for Atomic_Decrement_u32()
 * Atomically decrements a 32-bit unsigned value
 * @param pMinuend Pointer to the value to decrement
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_decrement_u32(uint32_t *pMinuend)
{
    return Atomic_Decrement_u32(pMinuend);
}

/**
 * @brief Wrapper for Atomic_OR_u32()
 * Atomically performs bitwise OR on a 32-bit unsigned value
 * @param pDestination Pointer to the destination value
 * @param ulValue Value to OR with
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_or_u32(uint32_t *pDestination, uint32_t ulValue)
{
    return Atomic_OR_u32(pDestination, ulValue);
}

/**
 * @brief Wrapper for Atomic_AND_u32()
 * Atomically performs bitwise AND on a 32-bit unsigned value
 * @param pDestination Pointer to the destination value
 * @param ulValue Value to AND with
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_and_u32(uint32_t *pDestination, uint32_t ulValue)
{
    return Atomic_AND_u32(pDestination, ulValue);
}

/**
 * @brief Wrapper for Atomic_NAND_u32()
 * Atomically performs bitwise NAND on a 32-bit unsigned value
 * @param pDestination Pointer to the destination value
 * @param ulValue Value to NAND with
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_nand_u32(uint32_t *pDestination, uint32_t ulValue)
{
    return Atomic_NAND_u32(pDestination, ulValue);
}

/**
 * @brief Wrapper for Atomic_XOR_u32()
 * Atomically performs bitwise XOR on a 32-bit unsigned value
 * @param pDestination Pointer to the destination value
 * @param ulValue Value to XOR with
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_xor_u32(uint32_t *pDestination, uint32_t ulValue)
{
    return Atomic_XOR_u32(pDestination, ulValue);
}

/**
 * @brief Wrapper for Atomic_CompareAndSwap_u32()
 * Atomically compares and swaps a 32-bit unsigned value
 * @param pDestination Pointer to the destination value
 * @param ulExchange Value to exchange with
 * @param ulComparand Value to compare against
 * @return uint32_t - Previous value
 */
uint32_t freertos_rs_atomic_compare_and_swap_u32(uint32_t *pDestination, uint32_t ulExchange, uint32_t ulComparand)
{
    return Atomic_CompareAndSwap_u32(pDestination, ulExchange, ulComparand);
}

/**
 * @brief Wrapper for Atomic_SwapPointers_p32()
 * Atomically swaps two 32-bit pointers
 * @param ppvDestination Pointer to the destination pointer
 * @param pvExchange Pointer to exchange with
 * @return void* - Previous pointer value
 */
void* freertos_rs_atomic_swap_pointers_p32(void **ppvDestination, void *pvExchange)
{
    return Atomic_SwapPointers_p32(ppvDestination, pvExchange);
}

/**
 * @brief Wrapper for Atomic_CompareAndSwapPointers_p32()
 * Atomically compares and swaps two 32-bit pointers
 * @param ppvDestination Pointer to the destination pointer
 * @param pvExchange Pointer to exchange with
 * @param pvComparand Pointer to compare against
 * @return void* - Previous pointer value
 */
void* freertos_rs_atomic_compare_and_swap_pointers_p32(void **ppvDestination, void *pvExchange, void *pvComparand)
{
    return Atomic_CompareAndSwapPointers_p32(ppvDestination, pvExchange, pvComparand);
}

/*===========================================================================
 * LIST OPERATION FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for vListInitialise()
 * Initializes a list
 * @param pxList Pointer to the list to initialize
 */
void freertos_rs_list_initialise(List_t *pxList)
{
    vListInitialise(pxList);
}

/**
 * @brief Wrapper for vListInitialiseItem()
 * Initializes a list item
 * @param pxItem Pointer to the list item to initialize
 */
void freertos_rs_list_initialise_item(ListItem_t *pxItem)
{
    vListInitialiseItem(pxItem);
}

/**
 * @brief Wrapper for vListInsert()
 * Inserts a list item into a list in priority order
 * @param pxList Pointer to the list
 * @param pxNewListItem Pointer to the list item to insert
 */
void freertos_rs_list_insert(List_t *pxList, ListItem_t *pxNewListItem)
{
    vListInsert(pxList, pxNewListItem);
}

/**
 * @brief Wrapper for vListInsertEnd()
 * Inserts a list item at the end of a list
 * @param pxList Pointer to the list
 * @param pxNewListItem Pointer to the list item to insert
 */
void freertos_rs_list_insert_end(List_t *pxList, ListItem_t *pxNewListItem)
{
    vListInsertEnd(pxList, pxNewListItem);
}

/**
 * @brief Wrapper for uxListRemove()
 * Removes a list item from a list
 * @param pxItemToRemove Pointer to the list item to remove
 * @return UBaseType_t - Number of items remaining in the list
 */
UBaseType_t freertos_rs_list_remove(ListItem_t *pxItemToRemove)
{
    return uxListRemove(pxItemToRemove);
}

/**
 * @brief Wrapper for listGET_OWNER_OF_NEXT_ENTRY()
 * Gets the owner of the next entry in a list
 * @param pxTCB Pointer to current TCB
 * @param pxList Pointer to the list
 * @return void* - Owner of the next entry
 */
void* freertos_rs_list_get_owner_of_next_entry(void *pxTCB, List_t *pxList)
{
    return listGET_OWNER_OF_NEXT_ENTRY(pxTCB, pxList);
}

/**
 * @brief Wrapper for listGET_OWNER_OF_HEAD_ENTRY()
 * Gets the owner of the head entry in a list
 * @param pxList Pointer to the list
 * @return void* - Owner of the head entry
 */
void* freertos_rs_list_get_owner_of_head_entry(List_t *pxList)
{
    return listGET_OWNER_OF_HEAD_ENTRY(pxList);
}

/**
 * @brief Wrapper for listIS_EMPTY()
 * Checks if a list is empty
 * @param pxList Pointer to the list
 * @return BaseType_t - pdTRUE if empty
 */
BaseType_t freertos_rs_list_is_empty(List_t *pxList)
{
    return listIS_EMPTY(pxList);
}

/**
 * @brief Wrapper for listCURRENT_LIST_LENGTH()
 * Gets the current length of a list
 * @param pxList Pointer to the list
 * @return UBaseType_t - Current list length
 */
UBaseType_t freertos_rs_list_current_list_length(List_t *pxList)
{
    return listCURRENT_LIST_LENGTH(pxList);
}

/**
 * @brief Wrapper for listGET_ITEM_VALUE_OF_HEAD_ENTRY()
 * Gets the item value of the head entry
 * @param pxList Pointer to the list
 * @return TickType_t - Item value of head entry
 */
TickType_t freertos_rs_list_get_item_value_of_head_entry(List_t *pxList)
{
    return listGET_ITEM_VALUE_OF_HEAD_ENTRY(pxList);
}

/**
 * @brief Wrapper for listSET_LIST_ITEM_OWNER()
 * Sets the owner of a list item
 * @param pxListItem Pointer to the list item
 * @param pxOwner Pointer to the owner
 */
void freertos_rs_list_set_list_item_owner(ListItem_t *pxListItem, void *pxOwner)
{
    listSET_LIST_ITEM_OWNER(pxListItem, pxOwner);
}

/**
 * @brief Wrapper for listGET_LIST_ITEM_OWNER()
 * Gets the owner of a list item
 * @param pxListItem Pointer to the list item
 * @return void* - Owner of the list item
 */
void* freertos_rs_list_get_list_item_owner(ListItem_t *pxListItem)
{
    return listGET_LIST_ITEM_OWNER(pxListItem);
}

/**
 * @brief Wrapper for listSET_LIST_ITEM_VALUE()
 * Sets the value of a list item
 * @param pxListItem Pointer to the list item
 * @param xValue Value to set
 */
void freertos_rs_list_set_list_item_value(ListItem_t *pxListItem, TickType_t xValue)
{
    listSET_LIST_ITEM_VALUE(pxListItem, xValue);
}

/**
 * @brief Wrapper for listGET_LIST_ITEM_VALUE()
 * Gets the value of a list item
 * @param pxListItem Pointer to the list item
 * @return TickType_t - Value of the list item
 */
TickType_t freertos_rs_list_get_list_item_value(ListItem_t *pxListItem)
{
    return listGET_LIST_ITEM_VALUE(pxListItem);
}

/**
 * @brief Wrapper for listGET_HEAD_ENTRY()
 * Gets the head entry of a list
 * @param pxList Pointer to the list
 * @return ListItem_t* - Pointer to the head entry
 */
ListItem_t* freertos_rs_list_get_head_entry(List_t *pxList)
{
    return listGET_HEAD_ENTRY(pxList);
}

/**
 * @brief Wrapper for listGET_NEXT()
 * Gets the next item in a list
 * @param pxListItem Pointer to the current list item
 * @return ListItem_t* - Pointer to the next list item
 */
ListItem_t* freertos_rs_list_get_next(ListItem_t *pxListItem)
{
    return listGET_NEXT(pxListItem);
}

/**
 * @brief Wrapper for listLIST_ITEM_CONTAINER()
 * Gets the container list of a list item
 * @param pxListItem Pointer to the list item
 * @return List_t* - Pointer to the container list
 */
List_t* freertos_rs_list_list_item_container(ListItem_t *pxListItem)
{
    return listLIST_ITEM_CONTAINER(pxListItem);
}

/*===========================================================================
 * CO-ROUTINE FUNCTIONS
 *===========================================================================*/

#if (configUSE_CO_ROUTINES == 1)

/**
 * @brief Wrapper for xCoRoutineCreate()
 * Creates a co-routine
 * @param pxCoRoutineCode Pointer to the co-routine function
 * @param uxPriority Priority of the co-routine
 * @param uxIndex Index of the co-routine
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_co_routine_create(crCOROUTINE_CODE pxCoRoutineCode, UBaseType_t uxPriority, UBaseType_t uxIndex)
{
    return xCoRoutineCreate(pxCoRoutineCode, uxPriority, uxIndex);
}

/**
 * @brief Wrapper for vCoRoutineSchedule()
 * Schedules co-routines
 */
void freertos_rs_co_routine_schedule(void)
{
    vCoRoutineSchedule();
}

/**
 * @brief Wrapper for vCoRoutineAddToDelayedList()
 * Adds a co-routine to the delayed list
 * @param xTicksToDelay Number of ticks to delay
 * @param pxEventList Pointer to event list (can be NULL)
 */
void freertos_rs_co_routine_add_to_delayed_list(TickType_t xTicksToDelay, List_t *pxEventList)
{
    vCoRoutineAddToDelayedList(xTicksToDelay, pxEventList);
}

/**
 * @brief Wrapper for xCoRoutineRemoveFromEventList()
 * Removes a co-routine from an event list
 * @param pxEventList Pointer to the event list
 * @return BaseType_t - pdTRUE if a co-routine was removed
 */
BaseType_t freertos_rs_co_routine_remove_from_event_list(const List_t *pxEventList)
{
    return xCoRoutineRemoveFromEventList(pxEventList);
}

/**
 * @brief Wrapper for vCoRoutineResetState()
 * Resets the co-routine state
 */
void freertos_rs_co_routine_reset_state(void)
{
    vCoRoutineResetState();
}

#endif /* configUSE_CO_ROUTINES */

/*===========================================================================
 * ADDITIONAL TASK FUNCTIONS
 *===========================================================================*/

/**
 * @brief Wrapper for vTaskGenericNotifyGiveFromISR()
 * Gives a notification from ISR (increment)
 * @param xTaskToNotify Handle of task to notify
 * @param uxIndexToNotify Index of notification
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 */
void freertos_rs_task_generic_notify_give_from_isr(TaskHandle_t xTaskToNotify, UBaseType_t uxIndexToNotify, BaseType_t *pxHigherPriorityTaskWoken)
{
    vTaskGenericNotifyGiveFromISR(xTaskToNotify, uxIndexToNotify, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for vTaskMissedYield()
 * Indicates that a context switch is required
 */
void freertos_rs_task_missed_yield(void)
{
    vTaskMissedYield();
}

/**
 * @brief Wrapper for xTaskPriorityInherit()
 * Inherits priority from a task holding a mutex
 * @param pxMutexHolder Handle of the mutex holder task
 * @return BaseType_t - pdTRUE if priority was inherited
 */
BaseType_t freertos_rs_task_priority_inherit(TaskHandle_t const pxMutexHolder)
{
    return xTaskPriorityInherit(pxMutexHolder);
}

/**
 * @brief Wrapper for xTaskPriorityDisinherit()
 * Disinherits priority from a task releasing a mutex
 * @param pxMutexHolder Handle of the mutex holder task
 * @return BaseType_t - pdTRUE if a context switch is needed
 */
BaseType_t freertos_rs_task_priority_disinherit(TaskHandle_t const pxMutexHolder)
{
    return xTaskPriorityDisinherit(pxMutexHolder);
}

/**
 * @brief Wrapper for vTaskPriorityDisinheritAfterTimeout()
 * Disinherits priority after a timeout on a mutex
 * @param pxMutexHolder Handle of the mutex holder task
 * @param uxHighestPriorityWaitingTask Highest priority of waiting tasks
 */
void freertos_rs_task_priority_disinherit_after_timeout(TaskHandle_t const pxMutexHolder, UBaseType_t uxHighestPriorityWaitingTask)
{
    vTaskPriorityDisinheritAfterTimeout(pxMutexHolder, uxHighestPriorityWaitingTask);
}

/**
 * @brief Wrapper for xTaskRemoveFromEventList()
 * Removes a task from an event list
 * @param pxEventList Pointer to the event list
 * @return BaseType_t - pdTRUE if the removed task has higher priority
 */
BaseType_t freertos_rs_task_remove_from_event_list(const List_t * const pxEventList)
{
    return xTaskRemoveFromEventList(pxEventList);
}

/**
 * @brief Wrapper for uxTaskResetEventItemValue()
 * Resets the event item value of the current task
 * @return UBaseType_t - Previous event item value
 */
UBaseType_t freertos_rs_task_reset_event_item_value(void)
{
    return uxTaskResetEventItemValue();
}

/**
 * @brief Wrapper for pvTaskIncrementMutexHeldCount()
 * Increments the mutex held count of the current task
 * @return void* - Handle of the current task
 */
void* freertos_rs_task_increment_mutex_held_count(void)
{
    return pvTaskIncrementMutexHeldCount();
}

#if (configNUMBER_OF_CORES > 1)
/**
 * @brief Wrapper for xTaskGetCurrentTaskHandleForCore()
 * Gets the handle of the currently running task on a specific core
 * @param xCoreID Core ID
 * @return TaskHandle_t - Current task handle for the specified core
 */
TaskHandle_t freertos_rs_task_get_current_task_handle_for_core(BaseType_t xCoreID)
{
    return xTaskGetCurrentTaskHandleForCore(xCoreID);
}

/**
 * @brief Wrapper for xTaskGetIdleTaskHandleForCore()
 * Gets the handle of the idle task for a specific core
 * @param xCoreID Core ID
 * @return TaskHandle_t - Idle task handle for the specified core
 */
TaskHandle_t freertos_rs_task_get_idle_task_handle_for_core(BaseType_t xCoreID)
{
    return xTaskGetIdleTaskHandleForCore(xCoreID);
}
#endif

/**
 * @brief Wrapper for uxTaskGetTaskNumber()
 * Gets the task number
 * @param xTask Handle of the task
 * @return UBaseType_t - Task number
 */
UBaseType_t freertos_rs_task_get_task_number(TaskHandle_t xTask)
{
    return uxTaskGetTaskNumber(xTask);
}

/**
 * @brief Wrapper for vTaskSetTaskNumber()
 * Sets the task number
 * @param xTask Handle of the task
 * @param uxHandle Task number to set
 */
void freertos_rs_task_set_task_number(TaskHandle_t xTask, const UBaseType_t uxHandle)
{
    vTaskSetTaskNumber(xTask, uxHandle);
}

/**
 * @brief Wrapper for eTaskConfirmSleepModeStatus()
 * Checks if the system can enter sleep mode
 * @return eSleepModeStatus - Sleep mode status
 */
BaseType_t freertos_rs_task_confirm_sleep_mode_status(void)
{
    return (BaseType_t)eTaskConfirmSleepModeStatus();
}

/*===========================================================================
 * ADDITIONAL TIMER FUNCTIONS
 *===========================================================================*/

#if (configUSE_TIMERS == 1)

/**
 * @brief Wrapper for vTimerSetReloadMode()
 * Sets the reload mode of a timer
 * @param xTimer Timer handle
 * @param xAutoReload Auto-reload flag (pdTRUE for auto-reload, pdFALSE for one-shot)
 */
void freertos_rs_timer_set_reload_mode(TimerHandle_t xTimer, const BaseType_t xAutoReload)
{
    vTimerSetReloadMode(xTimer, xAutoReload);
}

/**
 * @brief Wrapper for xTimerGetReloadMode()
 * Gets the reload mode of a timer
 * @param xTimer Timer handle
 * @return BaseType_t - pdTRUE if auto-reload, pdFALSE if one-shot
 */
BaseType_t freertos_rs_timer_get_reload_mode(TimerHandle_t xTimer)
{
    return xTimerGetReloadMode(xTimer);
}

/**
 * @brief Wrapper for uxTimerGetReloadMode()
 * Gets the reload mode of a timer as UBaseType_t
 * @param xTimer Timer handle
 * @return UBaseType_t - 1 if auto-reload, 0 if one-shot
 */
UBaseType_t freertos_rs_ux_timer_get_reload_mode(TimerHandle_t xTimer)
{
    return uxTimerGetReloadMode(xTimer);
}

/**
 * @brief Wrapper for xTimerCreateTimerTask()
 * Creates the timer daemon task
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_timer_create_timer_task(void)
{
    return xTimerCreateTimerTask();
}

/**
 * @brief Wrapper for vTimerResetState()
 * Resets the timer state
 */
void freertos_rs_timer_reset_state(void)
{
    vTimerResetState();
}

#endif /* configUSE_TIMERS */

/*===========================================================================
 * ADDITIONAL QUEUE FUNCTIONS
 *===========================================================================*/

#if (configUSE_QUEUE_SETS == 1)

#if (configSUPPORT_DYNAMIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xQueueCreateSet()
 * Creates a queue set with dynamic allocation
 * @param uxEventQueueLength Maximum number of items in the set
 * @return QueueSetHandle_t - Handle to the created queue set
 */
QueueSetHandle_t freertos_rs_queue_create_set(const UBaseType_t uxEventQueueLength)
{
    return xQueueCreateSet(uxEventQueueLength);
}
#endif

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xQueueCreateSetStatic()
 * Creates a queue set using static allocation
 * @param uxEventQueueLength Maximum number of items in the set
 * @param pucQueueStorage Storage buffer for the queue set
 * @param pxStaticQueue Static queue structure
 * @return QueueSetHandle_t - Handle to the created queue set
 */
QueueSetHandle_t freertos_rs_queue_create_set_static(const UBaseType_t uxEventQueueLength, uint8_t *pucQueueStorage, StaticQueue_t *pxStaticQueue)
{
    return xQueueCreateSetStatic(uxEventQueueLength, pucQueueStorage, pxStaticQueue);
}
#endif

/**
 * @brief Wrapper for xQueueAddToSet()
 * Adds a queue or semaphore to a queue set
 * @param xQueueOrSemaphore Handle of the queue or semaphore to add
 * @param xQueueSet Handle of the queue set
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_add_to_set(QueueSetMemberHandle_t xQueueOrSemaphore, QueueSetHandle_t xQueueSet)
{
    return xQueueAddToSet(xQueueOrSemaphore, xQueueSet);
}

/**
 * @brief Wrapper for xQueueRemoveFromSet()
 * Removes a queue or semaphore from a queue set
 * @param xQueueOrSemaphore Handle of the queue or semaphore to remove
 * @param xQueueSet Handle of the queue set
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_remove_from_set(QueueSetMemberHandle_t xQueueOrSemaphore, QueueSetHandle_t xQueueSet)
{
    return xQueueRemoveFromSet(xQueueOrSemaphore, xQueueSet);
}

/**
 * @brief Wrapper for xQueueSelectFromSet()
 * Selects from a queue set (blocks until a member is ready)
 * @param xQueueSet Handle of the queue set
 * @param xTicksToWait Ticks to wait
 * @return QueueSetMemberHandle_t - Handle to the queue/semaphore that contains data
 */
QueueSetMemberHandle_t freertos_rs_queue_select_from_set(QueueSetHandle_t xQueueSet, TickType_t xTicksToWait)
{
    return xQueueSelectFromSet(xQueueSet, xTicksToWait);
}

/**
 * @brief Wrapper for xQueueSelectFromSetFromISR()
 * Selects from a queue set from an ISR
 * @param xQueueSet Queue set handle
 * @return QueueSetMemberHandle_t - Handle to the queue/semaphore that contains data
 */
QueueSetMemberHandle_t freertos_rs_queue_select_from_set_from_isr(QueueSetHandle_t xQueueSet)
{
    return xQueueSelectFromSetFromISR(xQueueSet);
}

#endif /* configUSE_QUEUE_SETS */

/**
 * @brief Wrapper for xQueueSemaphoreTake()
 * Takes a semaphore (binary or counting)
 * @param xQueue Queue/semaphore handle
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_semaphore_take(QueueHandle_t xQueue, TickType_t xTicksToWait)
{
    return xQueueSemaphoreTake(xQueue, xTicksToWait);
}

#if (configUSE_RECURSIVE_MUTEXES == 1)
/**
 * @brief Wrapper for xQueueTakeMutexRecursive()
 * Takes a recursive mutex
 * @param xMutex Mutex handle
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_take_mutex_recursive(QueueHandle_t xMutex, TickType_t xTicksToWait)
{
    return xQueueTakeMutexRecursive(xMutex, xTicksToWait);
}

/**
 * @brief Wrapper for xQueueGiveMutexRecursive()
 * Gives a recursive mutex
 * @param xMutex Mutex handle
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_give_mutex_recursive(QueueHandle_t xMutex)
{
    return xQueueGiveMutexRecursive(xMutex);
}
#endif

/**
 * @brief Wrapper for xQueueGenericReset()
 * Resets a queue to its empty state
 * @param xQueue Queue handle
 * @param xNewQueue pdTRUE to create a new queue, pdFALSE to reset existing
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_queue_generic_reset(QueueHandle_t xQueue, BaseType_t xNewQueue)
{
    return xQueueGenericReset(xQueue, xNewQueue);
}

/*===========================================================================
 * ADDITIONAL STREAM BUFFER FUNCTIONS
 *===========================================================================*/

#if (configUSE_STREAM_BUFFERS == 1)

/**
 * @brief Wrapper for xStreamBufferResetFromISR()
 * Resets a stream buffer from an ISR
 * @param xStreamBuffer Stream buffer handle
 * @return BaseType_t - pdPASS if successful
 */
BaseType_t freertos_rs_stream_buffer_reset_from_isr(StreamBufferHandle_t xStreamBuffer)
{
    return xStreamBufferResetFromISR(xStreamBuffer);
}

/**
 * @brief Wrapper for xStreamBufferSendCompletedFromISR()
 * Called from ISR when a send operation completes
 * @param xStreamBuffer Stream buffer handle
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdTRUE if a task was woken
 */
BaseType_t freertos_rs_stream_buffer_send_completed_from_isr(StreamBufferHandle_t xStreamBuffer, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xStreamBufferSendCompletedFromISR(xStreamBuffer, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xStreamBufferReceiveCompletedFromISR()
 * Called from ISR when a receive operation completes
 * @param xStreamBuffer Stream buffer handle
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdTRUE if a task was woken
 */
BaseType_t freertos_rs_stream_buffer_receive_completed_from_isr(StreamBufferHandle_t xStreamBuffer, BaseType_t *pxHigherPriorityTaskWoken)
{
    return xStreamBufferReceiveCompletedFromISR(xStreamBuffer, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xStreamBufferNextMessageLengthBytes()
 * Gets the length of the next message in a stream buffer used for batching
 * @param xStreamBuffer Stream buffer handle
 * @return size_t - Length of the next message in bytes
 */
size_t freertos_rs_stream_buffer_next_message_length_bytes(StreamBufferHandle_t xStreamBuffer)
{
    return xStreamBufferNextMessageLengthBytes(xStreamBuffer);
}

/**
 * @brief Wrapper for uxStreamBufferGetStreamBufferNotificationIndex()
 * Gets the notification index used by a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @return UBaseType_t - Notification index
 */
UBaseType_t freertos_rs_stream_buffer_get_notification_index(StreamBufferHandle_t xStreamBuffer)
{
    return uxStreamBufferGetStreamBufferNotificationIndex(xStreamBuffer);
}

/**
 * @brief Wrapper for vStreamBufferSetStreamBufferNotificationIndex()
 * Sets the notification index for a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @param uxNotificationIndex Notification index to set
 */
void freertos_rs_stream_buffer_set_notification_index(StreamBufferHandle_t xStreamBuffer, UBaseType_t uxNotificationIndex)
{
    vStreamBufferSetStreamBufferNotificationIndex(xStreamBuffer, uxNotificationIndex);
}

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for xStreamBufferGetStaticBuffers()
 * Gets the static buffers associated with a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @param ppucStreamBufferStorageArea Pointer to receive storage area pointer
 * @param ppxStaticStreamBuffer Pointer to receive static stream buffer pointer
 * @return BaseType_t - pdTRUE if successful
 */
BaseType_t freertos_rs_stream_buffer_get_static_buffers(StreamBufferHandle_t xStreamBuffer, uint8_t **ppucStreamBufferStorageArea, StaticStreamBuffer_t **ppxStaticStreamBuffer)
{
    return xStreamBufferGetStaticBuffers(xStreamBuffer, ppucStreamBufferStorageArea, ppxStaticStreamBuffer);
}
#endif

#if (configUSE_TRACE_FACILITY == 1)
/**
 * @brief Wrapper for vStreamBufferSetStreamBufferNumber()
 * Sets the stream buffer number for tracing
 * @param xStreamBuffer Stream buffer handle
 * @param uxStreamBufferNumber Stream buffer number to set
 */
void freertos_rs_stream_buffer_set_stream_buffer_number(StreamBufferHandle_t xStreamBuffer, UBaseType_t uxStreamBufferNumber)
{
    vStreamBufferSetStreamBufferNumber(xStreamBuffer, uxStreamBufferNumber);
}

/**
 * @brief Wrapper for uxStreamBufferGetStreamBufferNumber()
 * Gets the stream buffer number for tracing
 * @param xStreamBuffer Stream buffer handle
 * @return UBaseType_t - Stream buffer number
 */
UBaseType_t freertos_rs_stream_buffer_get_stream_buffer_number(StreamBufferHandle_t xStreamBuffer)
{
    return uxStreamBufferGetStreamBufferNumber(xStreamBuffer);
}

/**
 * @brief Wrapper for ucStreamBufferGetStreamBufferType()
 * Gets the type of a stream buffer
 * @param xStreamBuffer Stream buffer handle
 * @return uint8_t - Stream buffer type
 */
uint8_t freertos_rs_stream_buffer_get_stream_buffer_type(StreamBufferHandle_t xStreamBuffer)
{
    return ucStreamBufferGetStreamBufferType(xStreamBuffer);
}
#endif

#endif /* configUSE_STREAM_BUFFERS */

/*===========================================================================
 * ADDITIONAL PORTABLE/MEMORY FUNCTIONS
 *===========================================================================*/

#if (configSUPPORT_DYNAMIC_ALLOCATION == 1)
/**
 * @brief Wrapper for pvPortCalloc()
 * Allocates and zeros memory from the FreeRTOS heap
 * @param xNum Number of elements
 * @param xSize Size of each element
 * @return void* - Pointer to allocated memory, or NULL if allocation failed
 */
void* freertos_rs_port_calloc(size_t xNum, size_t xSize)
{
    return pvPortCalloc(xNum, xSize);
}
#endif

/**
 * @brief Wrapper for vPortInitialiseBlocks()
 * Initializes memory blocks (heap_3 only)
 */
void freertos_rs_port_initialise_blocks(void)
{
    vPortInitialiseBlocks();
}

/**
 * @brief Wrapper for xPortResetHeapMinimumEverFreeHeapSize()
 * Resets the minimum ever free heap size counter
 */
void freertos_rs_port_reset_heap_minimum_ever_free_heap_size(void)
{
    xPortResetHeapMinimumEverFreeHeapSize();
}

#if (configUSE_HEAP_STRUCT_SELECTION == 1)
/**
 * @brief Wrapper for vPortGetHeapStats()
 * Gets heap statistics
 * @param pxHeapStats Pointer to heap statistics structure to fill
 */
void freertos_rs_port_get_heap_stats(void *pxHeapStats)
{
    vPortGetHeapStats((HeapStats_t *)pxHeapStats);
}
#endif

#if (configAPPLICATION_ALLOCATED_HEAP == 1)
/**
 * @brief Wrapper for vPortDefineHeapRegions()
 * Defines heap regions (heap_5 only)
 * @param pxHeapRegions Pointer to array of heap regions (terminated with NULL entry)
 */
void freertos_rs_port_define_heap_regions(const void * const pxHeapRegions)
{
    vPortDefineHeapRegions((const HeapRegion_t * const)pxHeapRegions);
}
#endif

/*===========================================================================
 * MISSING API ADDITIONS - GAP ANALYSIS COMPLETION
 *===========================================================================*/

/*===========================================================================
 * QUEUE - CO-ROUTINE QUEUE OPERATIONS
 *===========================================================================*/

#if (configUSE_CO_ROUTINES == 1)
/**
 * @brief Wrapper for xQueueCRSend()
 * Sends an item to a queue from a co-routine
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS or errQUEUE_FULL
 */
BaseType_t freertos_rs_queue_cr_send(QueueHandle_t xQueue, const void * const pvItemToQueue, TickType_t xTicksToWait)
{
    return xQueueCRSend(xQueue, pvItemToQueue, xTicksToWait);
}

/**
 * @brief Wrapper for xQueueCRReceive()
 * Receives an item from a queue in a co-routine
 * @param xQueue Queue handle
 * @param pvBuffer Pointer to buffer for received item
 * @param xTicksToWait Ticks to wait
 * @return BaseType_t - pdPASS or errQUEUE_EMPTY
 */
BaseType_t freertos_rs_queue_cr_receive(QueueHandle_t xQueue, void * const pvBuffer, TickType_t xTicksToWait)
{
    return xQueueCRReceive(xQueue, pvBuffer, xTicksToWait);
}

/**
 * @brief Wrapper for xQueueCRSendFromISR()
 * Sends an item to a queue from a co-routine ISR
 * @param xQueue Queue handle
 * @param pvItemToQueue Pointer to item to send
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS or errQUEUE_FULL
 */
BaseType_t freertos_rs_queue_cr_send_from_isr(QueueHandle_t xQueue, const void * const pvItemToQueue, BaseType_t * const pxHigherPriorityTaskWoken)
{
    return xQueueCRSendFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xQueueCRReceiveFromISR()
 * Receives an item from a queue in a co-routine ISR
 * @param xQueue Queue handle
 * @param pvBuffer Pointer to buffer for received item
 * @param pxHigherPriorityTaskWoken Pointer to higher priority task woken flag
 * @return BaseType_t - pdPASS or errQUEUE_EMPTY
 */
BaseType_t freertos_rs_queue_cr_receive_from_isr(QueueHandle_t xQueue, void * const pvBuffer, BaseType_t * const pxHigherPriorityTaskWoken)
{
    return xQueueCRReceiveFromISR(xQueue, pvBuffer, pxHigherPriorityTaskWoken);
}
#endif /* configUSE_CO_ROUTINES */

/*===========================================================================
 * QUEUE - RESTRICTED WAIT
 *===========================================================================*/

/**
 * @brief Wrapper for vQueueWaitForMessageRestricted()
 * Waits for a message with restricted permissions (MPU)
 * @param xQueue Queue handle
 * @param xTicksToWait Ticks to wait
 * @param xWaitIndefinitely Whether to wait indefinitely
 */
void freertos_rs_queue_wait_for_message_restricted(QueueHandle_t xQueue, TickType_t xTicksToWait, BaseType_t xWaitIndefinitely)
{
    vQueueWaitForMessageRestricted(xQueue, xTicksToWait, xWaitIndefinitely);
}

/*===========================================================================
 * EVENT GROUP - CALLBACK FUNCTIONS
 *===========================================================================*/

#if (configUSE_EVENT_GROUPS == 1)
/**
 * @brief Wrapper for vEventGroupSetBitsCallback()
 * Timer callback to set bits in an event group
 * @param pvEventGroup Event group handle (cast to void*)
 * @param ulBitsToSet Bits to set
 */
void freertos_rs_event_group_set_bits_callback(void *pvEventGroup, uint32_t ulBitsToSet)
{
    vEventGroupSetBitsCallback((EventGroupHandle_t)pvEventGroup, ulBitsToSet);
}

/**
 * @brief Wrapper for vEventGroupClearBitsCallback()
 * Timer callback to clear bits in an event group
 * @param pvEventGroup Event group handle (cast to void*)
 * @param ulBitsToClear Bits to clear
 */
void freertos_rs_event_group_clear_bits_callback(void *pvEventGroup, uint32_t ulBitsToClear)
{
    vEventGroupClearBitsCallback((EventGroupHandle_t)pvEventGroup, ulBitsToClear);
}
#endif /* configUSE_EVENT_GROUPS */

/*===========================================================================
 * STREAM BUFFER - WITH CALLBACK CREATION
 *===========================================================================*/

#if (configUSE_STREAM_BUFFERS == 1)
/**
 * @brief Wrapper for xStreamBufferCreateWithCallback()
 * Creates a stream buffer with send/receive callbacks
 * @param xBufferSizeBytes Buffer size
 * @param xTriggerLevelBytes Trigger level
 * @param pxSendCompletedCallback Send completion callback
 * @param pxReceiveCompletedCallback Receive completion callback
 * @return StreamBufferHandle_t - Handle or NULL
 */
StreamBufferHandle_t freertos_rs_stream_buffer_create_with_callback(
    size_t xBufferSizeBytes,
    size_t xTriggerLevelBytes,
    StreamBufferCallbackFunction_t pxSendCompletedCallback,
    StreamBufferCallbackFunction_t pxReceiveCompletedCallback)
{
    return xStreamBufferCreateWithCallback(xBufferSizeBytes, xTriggerLevelBytes,
        pxSendCompletedCallback, pxReceiveCompletedCallback);
}

/**
 * @brief Wrapper for xStreamBufferCreateStaticWithCallback()
 * Creates a static stream buffer with send/receive callbacks
 * @param xBufferSizeBytes Buffer size
 * @param xTriggerLevelBytes Trigger level
 * @param pucStreamBufferStorageArea Storage area
 * @param pxStaticStreamBuffer Static buffer structure
 * @param pxSendCompletedCallback Send completion callback
 * @param pxReceiveCompletedCallback Receive completion callback
 * @return StreamBufferHandle_t - Handle or NULL
 */
StreamBufferHandle_t freertos_rs_stream_buffer_create_static_with_callback(
    size_t xBufferSizeBytes,
    size_t xTriggerLevelBytes,
    uint8_t *pucStreamBufferStorageArea,
    StaticStreamBuffer_t *pxStaticStreamBuffer,
    StreamBufferCallbackFunction_t pxSendCompletedCallback,
    StreamBufferCallbackFunction_t pxReceiveCompletedCallback)
{
    return xStreamBufferCreateStaticWithCallback(xBufferSizeBytes, xTriggerLevelBytes,
        pucStreamBufferStorageArea, pxStaticStreamBuffer,
        pxSendCompletedCallback, pxReceiveCompletedCallback);
}

/*===========================================================================
 * STREAM BATCHING BUFFER
 *===========================================================================*/

#if (configUSE_STREAM_BUFFERS == 1)
/**
 * @brief Wrapper for xStreamBatchingBufferCreate()
 * Creates a batching stream buffer
 * @param xBufferSizeBytes Buffer size
 * @param xTriggerLevelBytes Trigger level
 * @return StreamBufferHandle_t - Handle or NULL
 */
StreamBufferHandle_t freertos_rs_stream_batching_buffer_create(
    size_t xBufferSizeBytes,
    size_t xTriggerLevelBytes)
{
    return xStreamBatchingBufferCreate(xBufferSizeBytes, xTriggerLevelBytes);
}

/**
 * @brief Wrapper for xStreamBatchingBufferCreateWithCallback()
 * Creates a batching stream buffer with callbacks
 * @param xBufferSizeBytes Buffer size
 * @param xTriggerLevelBytes Trigger level
 * @param pxSendCompletedCallback Send completion callback
 * @param pxReceiveCompletedCallback Receive completion callback
 * @return StreamBufferHandle_t - Handle or NULL
 */
StreamBufferHandle_t freertos_rs_stream_batching_buffer_create_with_callback(
    size_t xBufferSizeBytes,
    size_t xTriggerLevelBytes,
    StreamBufferCallbackFunction_t pxSendCompletedCallback,
    StreamBufferCallbackFunction_t pxReceiveCompletedCallback)
{
    return xStreamBatchingBufferCreateWithCallback(xBufferSizeBytes, xTriggerLevelBytes,
        pxSendCompletedCallback, pxReceiveCompletedCallback);
}

/**
 * @brief Wrapper for xStreamBatchingBufferCreateStatic()
 * Creates a static batching stream buffer
 * @param xBufferSizeBytes Buffer size
 * @param xTriggerLevelBytes Trigger level
 * @param pucStreamBufferStorageArea Storage area
 * @param pxStaticStreamBuffer Static buffer structure
 * @return StreamBufferHandle_t - Handle or NULL
 */
StreamBufferHandle_t freertos_rs_stream_batching_buffer_create_static(
    size_t xBufferSizeBytes,
    size_t xTriggerLevelBytes,
    uint8_t *pucStreamBufferStorageArea,
    StaticStreamBuffer_t *pxStaticStreamBuffer)
{
    return xStreamBatchingBufferCreateStatic(xBufferSizeBytes, xTriggerLevelBytes,
        pucStreamBufferStorageArea, pxStaticStreamBuffer);
}

/**
 * @brief Wrapper for xStreamBatchingBufferCreateStaticWithCallback()
 * Creates a static batching stream buffer with callbacks
 * @param xBufferSizeBytes Buffer size
 * @param xTriggerLevelBytes Trigger level
 * @param pucStreamBufferStorageArea Storage area
 * @param pxStaticStreamBuffer Static buffer structure
 * @param pxSendCompletedCallback Send completion callback
 * @param pxReceiveCompletedCallback Receive completion callback
 * @return StreamBufferHandle_t - Handle or NULL
 */
StreamBufferHandle_t freertos_rs_stream_batching_buffer_create_static_with_callback(
    size_t xBufferSizeBytes,
    size_t xTriggerLevelBytes,
    uint8_t *pucStreamBufferStorageArea,
    StaticStreamBuffer_t *pxStaticStreamBuffer,
    StreamBufferCallbackFunction_t pxSendCompletedCallback,
    StreamBufferCallbackFunction_t pxReceiveCompletedCallback)
{
    return xStreamBatchingBufferCreateStaticWithCallback(xBufferSizeBytes, xTriggerLevelBytes,
        pucStreamBufferStorageArea, pxStaticStreamBuffer,
        pxSendCompletedCallback, pxReceiveCompletedCallback);
}
#endif /* configUSE_STREAM_BUFFERS */
#endif /* configUSE_STREAM_BUFFERS */

/*===========================================================================
 * MESSAGE BUFFER - WITH CALLBACK + ISR RESET
 *===========================================================================*/

#if (configUSE_STREAM_BUFFERS == 1)
/**
 * @brief Wrapper for xMessageBufferCreateWithCallback()
 * Creates a message buffer with send/receive callbacks
 * @param xBufferSizeBytes Buffer size
 * @param pxSendCompletedCallback Send completion callback
 * @param pxReceiveCompletedCallback Receive completion callback
 * @return MessageBufferHandle_t - Handle or NULL
 */
MessageBufferHandle_t freertos_rs_message_buffer_create_with_callback(
    size_t xBufferSizeBytes,
    StreamBufferCallbackFunction_t pxSendCompletedCallback,
    StreamBufferCallbackFunction_t pxReceiveCompletedCallback)
{
    return xMessageBufferCreateWithCallback(xBufferSizeBytes,
        pxSendCompletedCallback, pxReceiveCompletedCallback);
}

/**
 * @brief Wrapper for xMessageBufferCreateStaticWithCallback()
 * Creates a static message buffer with send/receive callbacks
 * @param xBufferSizeBytes Buffer size
 * @param pucStreamBufferStorageArea Storage area
 * @param pxStaticStreamBuffer Static buffer structure
 * @param pxSendCompletedCallback Send completion callback
 * @param pxReceiveCompletedCallback Receive completion callback
 * @return MessageBufferHandle_t - Handle or NULL
 */
MessageBufferHandle_t freertos_rs_message_buffer_create_static_with_callback(
    size_t xBufferSizeBytes,
    uint8_t *pucStreamBufferStorageArea,
    StaticStreamBuffer_t *pxStaticStreamBuffer,
    StreamBufferCallbackFunction_t pxSendCompletedCallback,
    StreamBufferCallbackFunction_t pxReceiveCompletedCallback)
{
    return xMessageBufferCreateStaticWithCallback(xBufferSizeBytes,
        pucStreamBufferStorageArea, pxStaticStreamBuffer,
        pxSendCompletedCallback, pxReceiveCompletedCallback);
}

/**
 * @brief Wrapper for xMessageBufferResetFromISR()
 * Resets a message buffer from an ISR
 * @param xMessageBuffer Message buffer handle
 * @return BaseType_t - pdPASS or errINVALID_BUFFER
 */
BaseType_t freertos_rs_message_buffer_reset_from_isr(MessageBufferHandle_t xMessageBuffer)
{
    return xMessageBufferResetFromISR(xMessageBuffer);
}
#endif /* configUSE_STREAM_BUFFERS */

/*===========================================================================
 * PORTABLE - YIELD
 *===========================================================================*/

/**
 * @brief Wrapper for portYIELD()
 * Yields the current task, requesting a context switch
 */
void freertos_rs_port_yield(void)
{
    portYIELD();
}

/*===========================================================================
 * TASK - IDLE TASK MEMORY (STATIC ALLOCATION)
 *===========================================================================*/

#if (configSUPPORT_STATIC_ALLOCATION == 1)
/**
 * @brief Wrapper for vTaskGetIdleTaskMemory()
 * Gets the static memory for the idle task
 * @param ppxIdleTaskTCBBuffer Pointer to TCB buffer pointer
 * @param ppxIdleTaskStackBuffer Pointer to stack buffer pointer
 * @param pulIdleTaskStackSize Pointer to stack size
 */
void freertos_rs_task_get_idle_task_memory(
    StaticTask_t **ppxIdleTaskTCBBuffer,
    StackType_t **ppxIdleTaskStackBuffer,
    uint32_t *pulIdleTaskStackSize)
{
    vTaskGetIdleTaskMemory(ppxIdleTaskTCBBuffer, ppxIdleTaskStackBuffer, pulIdleTaskStackSize);
}

/**
 * @brief Wrapper for vTaskGetPassiveIdleTaskMemory()
 * Gets the static memory for the passive idle task (SMP)
 * @param ppxIdleTaskTCBBuffer Pointer to TCB buffer pointer
 * @param ppxIdleTaskStackBuffer Pointer to stack buffer pointer
 * @param pulIdleTaskStackSize Pointer to stack size
 * @param xCoreID Core ID for SMP
 */
void freertos_rs_task_get_passive_idle_task_memory(
    StaticTask_t **ppxIdleTaskTCBBuffer,
    StackType_t **ppxIdleTaskStackBuffer,
    uint32_t *pulIdleTaskStackSize,
    BaseType_t xCoreID)
{
    vTaskGetPassiveIdleTaskMemory(ppxIdleTaskTCBBuffer, ppxIdleTaskStackBuffer, pulIdleTaskStackSize, xCoreID);
}
#endif /* configSUPPORT_STATIC_ALLOCATION */

/*===========================================================================
 * TASK - IDLE RUN TIME STATS
 *===========================================================================*/

#if (configGENERATE_RUN_TIME_STATS == 1)
/**
 * @brief Wrapper for ulTaskGetIdleRunTimeCounter()
 * Returns the run time counter of the idle task
 * @return configRUN_TIME_COUNTER_TYPE - idle task run time counter
 */
uint32_t freertos_rs_task_get_idle_run_time_counter(void)
{
    return (uint32_t)ulTaskGetIdleRunTimeCounter();
}

/**
 * @brief Wrapper for ulTaskGetIdleRunTimePercent()
 * Returns the percentage of CPU time used by the idle task
 * @return configRUN_TIME_COUNTER_TYPE - idle task run time percentage
 */
uint32_t freertos_rs_task_get_idle_run_time_percent(void)
{
    return (uint32_t)ulTaskGetIdleRunTimePercent();
}
#endif /* configGENERATE_RUN_TIME_STATS */

/*===========================================================================
 * TASK - MPU RESTRICTED AFFINITY SET
 *===========================================================================*/

#if (portUSING_MPU_WRAPPERS == 1)
/**
 * @brief Wrapper for xTaskCreateRestrictedAffinitySet()
 * Creates a restricted task with core affinity
 * @param pxTaskDefinition Task parameters
 * @param uxCoreAffinityMask Core affinity mask
 * @param pxCreatedTask Handle of created task
 * @return BaseType_t - pdPASS or errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY
 */
BaseType_t freertos_rs_task_create_restricted_affinity_set(
    const void * const pxTaskDefinition,
    UBaseType_t uxCoreAffinityMask,
    TaskHandle_t * const pxCreatedTask)
{
    return xTaskCreateRestrictedAffinitySet(
        (const TaskParameters_t * const)pxTaskDefinition,
        uxCoreAffinityMask,
        pxCreatedTask);
}
#endif /* portUSING_MPU_WRAPPERS */

/*===========================================================================
 * MESSAGE BUFFER - ISR COMPLETED CALLBACKS
 *===========================================================================*/

#if (configUSE_STREAM_BUFFERS == 1)
/**
 * @brief Wrapper for xMessageBufferSendCompletedFromISR()
 * Called from ISR after sending to a message buffer from non-FreeRTOS code
 * @param xMessageBuffer Message buffer handle
 * @param pxHigherPriorityTaskWoken Higher priority task woken flag
 * @return BaseType_t - pdTRUE if a context switch is needed
 */
BaseType_t freertos_rs_message_buffer_send_completed_from_isr(
    MessageBufferHandle_t xMessageBuffer,
    BaseType_t * const pxHigherPriorityTaskWoken)
{
    return xMessageBufferSendCompletedFromISR(xMessageBuffer, pxHigherPriorityTaskWoken);
}

/**
 * @brief Wrapper for xMessageBufferReceiveCompletedFromISR()
 * Called from ISR after receiving from a message buffer from non-FreeRTOS code
 * @param xMessageBuffer Message buffer handle
 * @param pxHigherPriorityTaskWoken Higher priority task woken flag
 * @return BaseType_t - pdTRUE if a context switch is needed
 */
BaseType_t freertos_rs_message_buffer_receive_completed_from_isr(
    MessageBufferHandle_t xMessageBuffer,
    BaseType_t * const pxHigherPriorityTaskWoken)
{
    return xMessageBufferReceiveCompletedFromISR(xMessageBuffer, pxHigherPriorityTaskWoken);
}
#endif /* configUSE_STREAM_BUFFERS */