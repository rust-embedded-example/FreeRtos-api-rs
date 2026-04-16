//! FreeRTOS linked list module.
//!
//! Provides FFI bindings for FreeRTOS linked list operations. These are
//! primarily internal FreeRTOS kernel data structures, but are exposed for
//! advanced use cases like custom scheduling algorithms or debugging tools.

use crate::base::{FreeRtosBaseType, FreeRtosTickType, FreeRtosUBaseType, FreeRtosVoidPtr};

//===========================================================================
// TYPE DEFINITIONS
//===========================================================================

/// Opaque handle to a FreeRTOS `List_t`.
pub type FreeRtosListHandle = *mut core::ffi::c_void;

/// Opaque handle to a FreeRTOS `ListItem_t`.
pub type FreeRtosListItemHandle = *mut core::ffi::c_void;

//===========================================================================
// EXTERNAL C FUNCTION DECLARATIONS - LIST OPERATIONS
//===========================================================================

unsafe extern "C" {
    /// Initializes a list structure.
    pub fn freertos_rs_list_initialise(list: FreeRtosListHandle);

    /// Initializes a list item.
    pub fn freertos_rs_list_initialise_item(item: FreeRtosListItemHandle);

    /// Inserts a list item in sorted order (by item value).
    pub fn freertos_rs_list_insert(list: FreeRtosListHandle, new_list_item: FreeRtosListItemHandle);

    /// Inserts a list item at the end of the list.
    pub fn freertos_rs_list_insert_end(list: FreeRtosListHandle, new_list_item: FreeRtosListItemHandle);

    /// Removes a list item. Returns the new list length.
    pub fn freertos_rs_list_remove(item_to_remove: FreeRtosListItemHandle) -> FreeRtosUBaseType;

    /// Gets the owner of the next entry (cycles through the list).
    pub fn freertos_rs_list_get_owner_of_next_entry(
        list: FreeRtosListHandle,
        list_item: FreeRtosListItemHandle,
    ) -> FreeRtosVoidPtr;

    /// Gets the owner of the head entry.
    pub fn freertos_rs_list_get_owner_of_head_entry(list: FreeRtosListHandle) -> FreeRtosVoidPtr;

    /// Checks if a list is empty.
    pub fn freertos_rs_list_is_empty(list: FreeRtosListHandle) -> FreeRtosBaseType;

    /// Gets the current list length.
    pub fn freertos_rs_list_current_list_length(list: FreeRtosListHandle) -> FreeRtosUBaseType;

    /// Gets the item value of the head entry.
    pub fn freertos_rs_list_get_item_value_of_head_entry(list: FreeRtosListHandle) -> FreeRtosTickType;

    /// Sets the owner of a list item.
    pub fn freertos_rs_list_set_list_item_owner(list_item: FreeRtosListItemHandle, owner: FreeRtosVoidPtr);

    /// Gets the owner of a list item.
    pub fn freertos_rs_list_get_list_item_owner(list_item: FreeRtosListItemHandle) -> FreeRtosVoidPtr;

    /// Sets the value of a list item.
    pub fn freertos_rs_list_set_list_item_value(list_item: FreeRtosListItemHandle, value: FreeRtosTickType);

    /// Gets the value of a list item.
    pub fn freertos_rs_list_get_list_item_value(list_item: FreeRtosListItemHandle) -> FreeRtosTickType;

    /// Gets the head entry of a list.
    pub fn freertos_rs_list_get_head_entry(list: FreeRtosListHandle) -> FreeRtosListItemHandle;

    /// Gets the next item in a list.
    pub fn freertos_rs_list_get_next(list_item: FreeRtosListItemHandle) -> FreeRtosListItemHandle;

    /// Gets the container list of a list item.
    pub fn freertos_rs_list_list_item_container(list_item: FreeRtosListItemHandle) -> FreeRtosListHandle;

    /// Checks if a list item is contained within a specific list.
    /// Returns `PD_TRUE` if the item is in the list, `PD_FALSE` otherwise.
    pub fn freertos_rs_list_is_contained_within(
        list: FreeRtosListHandle,
        list_item: FreeRtosListItemHandle,
    ) -> FreeRtosBaseType;
}

//===========================================================================
// UNIT TESTS
//===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_types_are_pointers() {
        use core::mem::size_of;
        assert_eq!(size_of::<FreeRtosListHandle>(), size_of::<*mut core::ffi::c_void>());
        assert_eq!(size_of::<FreeRtosListItemHandle>(), size_of::<*mut core::ffi::c_void>());
    }
}
