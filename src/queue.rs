use crate::base::{
    FreeRtosBaseType, FreeRtosTickType, FreeRtosQueueHandle, FreeRtosUBaseType,
    FreeRtosVoidPtr, FreeRtosQueueSetHandle, FreeRtosQueueSetMemberHandle, FreeRtosTaskHandle
};

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE CREATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xQueueCreate()
    /// Creates a new queue
    pub fn freertos_rs_queue_create(
        queue_length: FreeRtosUBaseType,
        item_size: FreeRtosUBaseType
    ) -> FreeRtosQueueHandle;
    
    /// Wrapper for xQueueCreateStatic()
    /// Creates a new queue using statically allocated memory
    pub fn freertos_rs_queue_create_static(
        queue_length: FreeRtosUBaseType,
        item_size: FreeRtosUBaseType,
        storage_buffer: *mut u8,
        queue_buffer: FreeRtosVoidPtr
    ) -> FreeRtosQueueHandle;
    
    /// Wrapper for vQueueDelete()
    /// Deletes a queue
    pub fn freertos_rs_queue_delete(queue: FreeRtosQueueHandle);
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xQueueSend()
    /// Sends an item to the back of a queue
    pub fn freertos_rs_queue_send(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueueSendToFront()
    /// Sends an item to the front of a queue
    pub fn freertos_rs_queue_send_to_front(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueueSendToBack()
    /// Sends an item to the back of a queue
    pub fn freertos_rs_queue_send_to_back(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueueReceive()
    /// Receives an item from a queue
    pub fn freertos_rs_queue_receive(
        queue: FreeRtosQueueHandle,
        buffer: FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueuePeek()
    /// Peeks at an item in a queue without removing it
    pub fn freertos_rs_queue_peek(
        queue: FreeRtosQueueHandle,
        buffer: FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE ISR OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xQueueSendFromISR()
    /// Sends an item to a queue from an ISR
    pub fn freertos_rs_queue_send_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueueSendToFrontFromISR()
    /// Sends an item to the front of a queue from an ISR
    pub fn freertos_rs_queue_send_to_front_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueueSendToBackFromISR()
    /// Sends an item to the back of a queue from an ISR
    pub fn freertos_rs_queue_send_to_back_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueueReceiveFromISR()
    /// Receives an item from a queue from an ISR
    pub fn freertos_rs_queue_receive_from_isr(
        queue: FreeRtosQueueHandle,
        buffer: FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE INFORMATION
//===========================================================================

unsafe extern "C" {
    /// Wrapper for uxQueueMessagesWaiting()
    /// Returns the number of messages waiting in a queue
    pub fn freertos_rs_queue_messages_waiting(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;

    /// Wrapper for uxQueueMessagesWaitingFromISR()
    /// Returns the number of messages waiting in a queue from an ISR
    pub fn freertos_rs_queue_messages_waiting_from_isr(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;

    /// Wrapper for xQueueIsQueueEmptyFromISR()
    /// Checks if a queue is empty from an ISR
    pub fn freertos_rs_queue_is_queue_empty_from_isr(queue: FreeRtosQueueHandle) -> FreeRtosBaseType;

    /// Wrapper for xQueueIsQueueFullFromISR()
    /// Checks if a queue is full from an ISR
    pub fn freertos_rs_queue_is_queue_full_from_isr(queue: FreeRtosQueueHandle) -> FreeRtosBaseType;
    
    /// Wrapper for uxQueueSpacesAvailable()
    /// Returns the number of free spaces in a queue
    pub fn freertos_rs_queue_spaces_available(queue: FreeRtosQueueHandle) -> FreeRtosUBaseType;
    
    /// Wrapper for xQueueReset()
    /// Resets a queue to its empty state
    pub fn freertos_rs_queue_reset(queue: FreeRtosQueueHandle) -> FreeRtosBaseType;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - QUEUE SETS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xQueueCreateSet()
    /// Creates a queue set
    pub fn freertos_rs_queue_create_set(
        set_length: FreeRtosUBaseType
    ) -> FreeRtosQueueSetHandle;
    
    /// Wrapper for xQueueAddToSet()
    /// Adds a queue to a queue set
    pub fn freertos_rs_queue_add_to_set(
        queue_or_semaphore: FreeRtosQueueSetMemberHandle,
        queue_set: FreeRtosQueueSetHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueueRemoveFromSet()
    /// Removes a queue from a queue set
    pub fn freertos_rs_queue_remove_from_set(
        queue_or_semaphore: FreeRtosQueueSetMemberHandle,
        queue_set: FreeRtosQueueSetHandle
    ) -> FreeRtosBaseType;
    
    /// Wrapper for xQueueSelectFromSet()
    /// Selects from a queue set
    pub fn freertos_rs_queue_select_from_set(
        queue_set: FreeRtosQueueSetHandle,
        ticks_to_wait: FreeRtosTickType
    ) -> FreeRtosQueueSetMemberHandle;
}

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - ADVANCED QUEUE OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for xQueueOverwrite()
    /// Overwrites an item in a queue (queue must have length 1)
    pub fn freertos_rs_queue_overwrite(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr
    ) -> FreeRtosBaseType;

    /// Wrapper for xQueueOverwriteFromISR()
    /// Overwrites an item in a queue from an ISR
    pub fn freertos_rs_queue_overwrite_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType
    ) -> FreeRtosBaseType;

    /// Wrapper for xQueueGenericSend()
    /// Generic queue send function
    pub fn freertos_rs_queue_generic_send(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        ticks_to_wait: FreeRtosTickType,
        copy_position: FreeRtosBaseType
    ) -> FreeRtosBaseType;

    /// Wrapper for xQueueGenericSendFromISR()
    /// Generic queue send function from ISR
    pub fn freertos_rs_queue_generic_send_from_isr(
        queue: FreeRtosQueueHandle,
        item_to_queue: *const FreeRtosVoidPtr,
        higher_priority_task_woken: *mut FreeRtosBaseType,
        copy_position: FreeRtosBaseType
    ) -> FreeRtosBaseType;

    /// Wrapper for xQueueCreateMutex()
    /// Creates a mutex (internal function)
    pub fn freertos_rs_queue_create_mutex(
        queue_type: u8
    ) -> FreeRtosQueueHandle;

    /// Wrapper for xQueueCreateMutexStatic()
    /// Creates a mutex using static allocation (internal function)
    pub fn freertos_rs_queue_create_mutex_static(
        queue_type: u8,
        static_queue: FreeRtosVoidPtr
    ) -> FreeRtosQueueHandle;

    /// Wrapper for xQueueGetMutexHolder()
    /// Gets the holder of a mutex
    pub fn freertos_rs_queue_get_mutex_holder(
        semaphore: FreeRtosQueueHandle
    ) -> FreeRtosTaskHandle;

    /// Wrapper for xQueueGetMutexHolderFromISR()
    /// Gets the holder of a mutex from ISR
    pub fn freertos_rs_queue_get_mutex_holder_from_isr(
        semaphore: FreeRtosQueueHandle
    ) -> FreeRtosTaskHandle;

    /// Wrapper for xQueuePeekFromISR()
    /// Peeks at an item in a queue from an ISR
    pub fn freertos_rs_queue_peek_from_isr(
        queue: FreeRtosQueueHandle,
        buffer: FreeRtosVoidPtr
    ) -> FreeRtosBaseType;

    /// Wrapper for xQueueGetStaticBuffers()
    /// Gets the static buffers associated with a queue
    pub fn freertos_rs_queue_get_static_buffers(
        queue: FreeRtosQueueHandle,
        queue_storage: *mut *mut u8,
        static_queue: *mut FreeRtosVoidPtr
    ) -> FreeRtosBaseType;

    /// Wrapper for uxQueueGetQueueItemSize()
    /// Gets the size of items in a queue
    pub fn freertos_rs_queue_get_queue_item_size(
        queue: FreeRtosQueueHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for uxQueueGetQueueLength()
    /// Gets the length of a queue
    pub fn freertos_rs_queue_get_queue_length(
        queue: FreeRtosQueueHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for vQueueAddToRegistry()
    /// Adds a queue to the registry
    pub fn freertos_rs_queue_add_to_registry(
        queue: FreeRtosQueueHandle,
        queue_name: *const u8
    );

    /// Wrapper for vQueueUnregisterQueue()
    /// Removes a queue from the registry
    pub fn freertos_rs_queue_unregister_queue(queue: FreeRtosQueueHandle);

    /// Wrapper for pcQueueGetName()
    /// Gets the name of a queue
    pub fn freertos_rs_queue_get_name(queue: FreeRtosQueueHandle) -> *const u8;

    /// Wrapper for vQueueSetQueueNumber()
    /// Sets the queue number for tracing
    pub fn freertos_rs_queue_set_queue_number(
        queue: FreeRtosQueueHandle,
        queue_number: FreeRtosUBaseType
    );

    /// Wrapper for uxQueueGetQueueNumber()
    /// Gets the queue number for tracing
    pub fn freertos_rs_queue_get_queue_number(
        queue: FreeRtosQueueHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for ucQueueGetQueueType()
    /// Gets the type of a queue
    pub fn freertos_rs_queue_get_queue_type(queue: FreeRtosQueueHandle) -> u8;
}
