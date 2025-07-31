use crate::base::{
    FreeRtosBaseType, FreeRtosUBaseType, FreeRtosTickType, FreeRtosVoidPtr
};

//===========================================================================
// TYPE DEFINITIONS
//===========================================================================

/// FreeRTOS List_t handle
pub type FreeRtosListHandle = *mut core::ffi::c_void;

/// FreeRTOS ListItem_t handle  
pub type FreeRtosListItemHandle = *mut core::ffi::c_void;

/// FreeRTOS MiniListItem_t handle
pub type FreeRtosMiniListItemHandle = *mut core::ffi::c_void;

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - LIST OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Wrapper for vListInitialise()
    /// Initializes a list
    pub fn freertos_rs_list_initialise(list: FreeRtosListHandle);

    /// Wrapper for vListInitialiseItem()
    /// Initializes a list item
    pub fn freertos_rs_list_initialise_item(item: FreeRtosListItemHandle);

    /// Wrapper for vListInsert()
    /// Inserts a list item into a list in priority order
    pub fn freertos_rs_list_insert(
        list: FreeRtosListHandle,
        new_list_item: FreeRtosListItemHandle
    );

    /// Wrapper for vListInsertEnd()
    /// Inserts a list item at the end of a list
    pub fn freertos_rs_list_insert_end(
        list: FreeRtosListHandle,
        new_list_item: FreeRtosListItemHandle
    );

    /// Wrapper for uxListRemove()
    /// Removes a list item from a list
    pub fn freertos_rs_list_remove(
        item_to_remove: FreeRtosListItemHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for listGET_OWNER_OF_NEXT_ENTRY()
    /// Gets the owner of the next entry in a list
    pub fn freertos_rs_list_get_owner_of_next_entry(
        list: FreeRtosListHandle,
        list_item: FreeRtosListItemHandle
    ) -> FreeRtosVoidPtr;

    /// Wrapper for listGET_OWNER_OF_HEAD_ENTRY()
    /// Gets the owner of the head entry in a list
    pub fn freertos_rs_list_get_owner_of_head_entry(
        list: FreeRtosListHandle
    ) -> FreeRtosVoidPtr;

    /// Wrapper for listIS_EMPTY()
    /// Checks if a list is empty
    pub fn freertos_rs_list_is_empty(list: FreeRtosListHandle) -> FreeRtosBaseType;

    /// Wrapper for listCURRENT_LIST_LENGTH()
    /// Gets the current length of a list
    pub fn freertos_rs_list_current_list_length(
        list: FreeRtosListHandle
    ) -> FreeRtosUBaseType;

    /// Wrapper for listGET_ITEM_VALUE_OF_HEAD_ENTRY()
    /// Gets the item value of the head entry
    pub fn freertos_rs_list_get_item_value_of_head_entry(
        list: FreeRtosListHandle
    ) -> FreeRtosTickType;

    /// Wrapper for listSET_LIST_ITEM_OWNER()
    /// Sets the owner of a list item
    pub fn freertos_rs_list_set_list_item_owner(
        list_item: FreeRtosListItemHandle,
        owner: FreeRtosVoidPtr
    );

    /// Wrapper for listGET_LIST_ITEM_OWNER()
    /// Gets the owner of a list item
    pub fn freertos_rs_list_get_list_item_owner(
        list_item: FreeRtosListItemHandle
    ) -> FreeRtosVoidPtr;

    /// Wrapper for listSET_LIST_ITEM_VALUE()
    /// Sets the value of a list item
    pub fn freertos_rs_list_set_list_item_value(
        list_item: FreeRtosListItemHandle,
        value: FreeRtosTickType
    );

    /// Wrapper for listGET_LIST_ITEM_VALUE()
    /// Gets the value of a list item
    pub fn freertos_rs_list_get_list_item_value(
        list_item: FreeRtosListItemHandle
    ) -> FreeRtosTickType;

    /// Wrapper for listGET_HEAD_ENTRY()
    /// Gets the head entry of a list
    pub fn freertos_rs_list_get_head_entry(
        list: FreeRtosListHandle
    ) -> FreeRtosListItemHandle;

    /// Wrapper for listGET_NEXT()
    /// Gets the next item in a list
    pub fn freertos_rs_list_get_next(
        list_item: FreeRtosListItemHandle
    ) -> FreeRtosListItemHandle;

    /// Wrapper for listLIST_ITEM_CONTAINER()
    /// Gets the container list of a list item
    pub fn freertos_rs_list_list_item_container(
        list_item: FreeRtosListItemHandle
    ) -> FreeRtosListHandle;
}
