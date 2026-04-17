//! `FreeRTOS` linked list module.
//!
//! Provides FFI bindings for `FreeRTOS` linked list operations. These are
//! primarily internal `FreeRTOS` kernel data structures, but are exposed for
//! advanced use cases like custom scheduling algorithms or debugging tools.

use crate::base::{FreeRtosBaseType, FreeRtosTickType, FreeRtosUBaseType, FreeRtosVoidPtr, PD_TRUE};

//===========================================================================
// TYPE DEFINITIONS
//===========================================================================

/// Opaque handle to a `FreeRTOS` `List_t`.
pub type FreeRtosListHandle = *mut core::ffi::c_void;

/// Opaque handle to a `FreeRTOS` `ListItem_t`.
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
    /// The `tcb` parameter is a pointer that receives the task TCB owner.
    /// The `list` parameter is the list to iterate.
    pub fn freertos_rs_list_get_owner_of_next_entry(
        tcb: FreeRtosVoidPtr,
        list: FreeRtosListHandle,
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

    /// Gets the end marker of a list.
    ///
    /// Wraps `listGET_END_MARKER()`. Returns the end-marker `ListItem_t*`.
    /// Useful for iteration bounds checking.
    pub fn freertos_rs_list_get_end_marker(list: FreeRtosListHandle) -> FreeRtosListItemHandle;

    /// Removes a list item without returning the new length (inline optimization).
    ///
    /// Wraps `listREMOVE_ITEM()`. Used by `xTaskIncrementTick()` for performance.
    pub fn freertos_rs_list_remove_item(item_to_remove: FreeRtosListItemHandle);

    /// Inserts a list item at the end (inline optimization).
    ///
    /// Wraps `listINSERT_END()`. Used by `xTaskIncrementTick()` for performance.
    pub fn freertos_rs_list_insert_end_inline(
        list: FreeRtosListHandle,
        new_list_item: FreeRtosListItemHandle,
    );

    /// Checks if a list has been initialized.
    ///
    /// Wraps `listLIST_IS_INITIALISED()`. Checks if `xListEnd.xItemValue == portMAX_DELAY`.
    /// Returns `PD_TRUE` if the list is initialized.
    pub fn freertos_rs_list_is_initialised(list: FreeRtosListHandle) -> FreeRtosBaseType;
}

//===========================================================================
// SAFE HELPER FUNCTIONS
//===========================================================================

/// Initializes a `FreeRTOS` list.
///
/// # Safety
/// `list` must point to a valid, properly aligned `List_t` structure.
#[inline]
pub unsafe fn list_initialise(list: FreeRtosListHandle) {
    unsafe { freertos_rs_list_initialise(list) }
}

/// Initializes a `FreeRTOS` list item.
///
/// # Safety
/// `item` must point to a valid, properly aligned `ListItem_t` structure.
#[inline]
pub unsafe fn list_initialise_item(item: FreeRtosListItemHandle) {
    unsafe { freertos_rs_list_initialise_item(item) }
}

/// Inserts a list item in sorted order (ascending by item value).
///
/// # Safety
/// `list` and `new_item` must be valid, initialized structures.
#[inline]
pub unsafe fn list_insert(list: FreeRtosListHandle, new_item: FreeRtosListItemHandle) {
    unsafe { freertos_rs_list_insert(list, new_item) }
}

/// Inserts a list item at the end of the list.
///
/// # Safety
/// `list` and `new_item` must be valid, initialized structures.
#[inline]
pub unsafe fn list_insert_end(list: FreeRtosListHandle, new_item: FreeRtosListItemHandle) {
    unsafe { freertos_rs_list_insert_end(list, new_item) }
}

/// Removes a list item. Returns the new list length.
///
/// # Safety
/// `item` must be a valid list item currently contained in a list.
#[inline]
pub unsafe fn list_remove(item: FreeRtosListItemHandle) -> FreeRtosUBaseType {
    unsafe { freertos_rs_list_remove(item) }
}

/// Returns `true` if the list is empty.
///
/// # Safety
/// `list` must point to a valid, initialized `List_t`.
#[inline]
pub unsafe fn list_is_empty(list: FreeRtosListHandle) -> bool {
    unsafe { freertos_rs_list_is_empty(list) == PD_TRUE }
}

/// Returns the number of items in the list.
///
/// # Safety
/// `list` must point to a valid, initialized `List_t`.
#[inline]
pub unsafe fn list_length(list: FreeRtosListHandle) -> FreeRtosUBaseType {
    unsafe { freertos_rs_list_current_list_length(list) }
}

/// Sets the owner (user data) of a list item.
///
/// # Safety
/// `item` must point to a valid, initialized `ListItem_t`.
#[inline]
pub unsafe fn list_set_item_owner(item: FreeRtosListItemHandle, owner: FreeRtosVoidPtr) {
    unsafe { freertos_rs_list_set_list_item_owner(item, owner) }
}

/// Gets the owner (user data) of a list item.
///
/// # Safety
/// `item` must point to a valid `ListItem_t`.
#[inline]
pub unsafe fn list_get_item_owner(item: FreeRtosListItemHandle) -> FreeRtosVoidPtr {
    unsafe { freertos_rs_list_get_list_item_owner(item) }
}

/// Sets the sort value of a list item.
///
/// # Safety
/// `item` must point to a valid, initialized `ListItem_t`.
#[inline]
pub unsafe fn list_set_item_value(item: FreeRtosListItemHandle, value: FreeRtosTickType) {
    unsafe { freertos_rs_list_set_list_item_value(item, value) }
}

/// Gets the sort value of a list item.
///
/// # Safety
/// `item` must point to a valid `ListItem_t`.
#[inline]
pub unsafe fn list_get_item_value(item: FreeRtosListItemHandle) -> FreeRtosTickType {
    unsafe { freertos_rs_list_get_list_item_value(item) }
}

/// Returns `true` if `item` is currently contained within `list`.
///
/// # Safety
/// Both pointers must be valid, initialized structures.
#[inline]
pub unsafe fn list_is_contained_within(list: FreeRtosListHandle, item: FreeRtosListItemHandle) -> bool {
    unsafe { freertos_rs_list_is_contained_within(list, item) == PD_TRUE }
}

/// Returns the end marker of a list. Useful for iteration bounds.
///
/// # Safety
/// `list` must point to a valid, initialized `List_t`.
#[inline]
pub unsafe fn list_get_end_marker(list: FreeRtosListHandle) -> FreeRtosListItemHandle {
    unsafe { freertos_rs_list_get_end_marker(list) }
}

/// Removes a list item without returning length (optimization).
///
/// # Safety
/// `item` must be a valid list item currently contained in a list.
#[inline]
pub unsafe fn list_remove_item(item: FreeRtosListItemHandle) {
    unsafe { freertos_rs_list_remove_item(item) }
}

/// Returns `true` if the list has been initialized.
///
/// # Safety
/// `list` must point to a valid memory location of at least `sizeof(List_t)`.
#[inline]
pub unsafe fn list_is_initialised(list: FreeRtosListHandle) -> bool {
    unsafe { freertos_rs_list_is_initialised(list) == PD_TRUE }
}

/// Gets the head entry of a list.
///
/// # Safety
/// `list` must point to a valid, initialized `List_t`.
#[inline]
pub unsafe fn list_get_head_entry(list: FreeRtosListHandle) -> FreeRtosListItemHandle {
    unsafe { freertos_rs_list_get_head_entry(list) }
}

/// Gets the next item after the given list item.
///
/// # Safety
/// `item` must be a valid `ListItem_t` contained in a list.
#[inline]
pub unsafe fn list_get_next(item: FreeRtosListItemHandle) -> FreeRtosListItemHandle {
    unsafe { freertos_rs_list_get_next(item) }
}

/// Gets the item value of the head entry.
///
/// # Safety
/// `list` must point to a valid, non-empty `List_t`.
#[inline]
pub unsafe fn list_get_item_value_of_head_entry(list: FreeRtosListHandle) -> FreeRtosTickType {
    unsafe { freertos_rs_list_get_item_value_of_head_entry(list) }
}

/// Gets the owner of the head entry.
///
/// # Safety
/// `list` must point to a valid, non-empty `List_t`.
#[inline]
pub unsafe fn list_get_owner_of_head_entry(list: FreeRtosListHandle) -> FreeRtosVoidPtr {
    unsafe { freertos_rs_list_get_owner_of_head_entry(list) }
}

/// Gets the owner of the next entry (cycles through the list).
///
/// The `tcb` parameter receives the current owner before advancing.
///
/// # Safety
/// `tcb` must point to valid memory for a `void*` output.
/// `list` must point to a valid, initialized `List_t`.
#[inline]
pub unsafe fn list_get_owner_of_next_entry(tcb: FreeRtosVoidPtr, list: FreeRtosListHandle) -> FreeRtosVoidPtr {
    unsafe { freertos_rs_list_get_owner_of_next_entry(tcb, list) }
}

/// Inserts a list item at the end (inline optimization for tick ISR).
///
/// # Safety
/// `list` and `new_item` must be valid, initialized structures.
#[inline]
pub unsafe fn list_insert_end_inline(list: FreeRtosListHandle, new_item: FreeRtosListItemHandle) {
    unsafe { freertos_rs_list_insert_end_inline(list, new_item) }
}

/// Gets the container list of a list item.
///
/// # Safety
/// `item` must point to a valid `ListItem_t`.
#[inline]
pub unsafe fn list_item_container(item: FreeRtosListItemHandle) -> FreeRtosListHandle {
    unsafe { freertos_rs_list_list_item_container(item) }
}

//===========================================================================
// COMPILE-TIME ASSERTIONS (replaces #[test] for no_std bare-metal)
//===========================================================================

const _: () = assert!(core::mem::size_of::<FreeRtosListHandle>() == core::mem::size_of::<*mut core::ffi::c_void>());
const _: () = assert!(core::mem::size_of::<FreeRtosListItemHandle>() == core::mem::size_of::<*mut core::ffi::c_void>());
