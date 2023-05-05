//! Bindings to mode.h

use {
    crate::{mode_private::Mode, types::RofiIntMatcher},
    ::std::{
        ffi::c_void,
        os::raw::{c_char, c_int, c_uint},
    },
};

/// Mode to exit Rofi.
pub const EXIT: c_int = 1000;

/// Mode to skip to the next cycle-able dialog.
pub const NEXT_DIALOG: c_int = 1001;

/// Mode to reload current dialog.
pub const RELOAD_DIALOG: c_int = 1002;

/// Mode to go to the previous dialog.
pub const PREVIOUS_DIALOG: c_int = 1003;

/// Mode to reload the dialog and unset user input.
pub const RESET_DIALOG: c_int = 1004;

/// States returned by the rofi window.
pub mod menu {
    use ::std::os::raw::c_int;

    /// Entry is selected.
    pub const OK: c_int = 0x00010000;

    /// User canceled the operation. (e.g. pressed escape)
    pub const CANCEL: c_int = 0x00020000;

    /// User requested a mode switch
    pub const NEXT: c_int = 0x00040000;

    /// Custom (non-matched) input was entered.
    pub const CUSTOM_INPUT: c_int = 0x00080000;

    /// User wanted to delete entry from history.
    pub const ENTRY_DELETE: c_int = 0x00100000;

    /// User wants to jump to another switcher.
    pub const QUICK_SWITCH: c_int = 0x00200000;

    /// User wants to jump to custom command.
    pub const CUSTOM_COMMAND: c_int = 0x00800000;

    /// Go to the previous menu.
    pub const PREVIOUS: c_int = 0x00400000;

    /// Go to the complete.
    pub const COMPLETE: c_int = 0x01000000;

    /// Bindings specifics
    pub const CUSTOM_ACTION: c_int = 0x10000000;

    /// Mask
    pub const LOWER_MASK: c_int = 0x0000FFF;
}

extern "C" {
    /// Initialize a mode.
    ///
    /// Returns FALSE if there was a failure, TRUE if successful.
    pub fn mode_init(mode: *mut Mode) -> c_int;

    /// Destroy the mode.
    pub fn mode_destroy(mode: *mut Mode);

    /// Get the number of entries in the mode.
    pub fn mode_get_num_entries(mode: *const Mode) -> c_uint;

    /// Returns the string as it should be displayed for the entry and the state of how it should
    /// be displayed.
    ///
    /// When `get_entry` is `TRUE` a new string is allocated and returned.
    ///
    /// - `selected_line`: The entry to query
    /// - `state`: The state of the entry \[out\]
    /// - `attribute_list`: List of extra (pango) attributes to apply when displaying \[out\] \[null\]
    /// - `get_entry`: If the entry should be returned
    pub fn mode_get_display_value(
        mode: *const Mode,
        selected_line: c_uint,
        state: *mut c_int,
        attribute_list: *mut *mut glib_sys::GList,
        get_entry: c_int,
    ) -> *mut c_char;

    /// Returns the icon for the selected line.
    ///
    /// Returns a newly allocated `cairo_surface_t` if applicable.
    ///
    /// - `selected_line`: The entry to query
    /// - `height`: The desired height of the icon
    pub fn mode_get_icon(
        mode: *mut Mode,
        selected_line: c_uint,
        height: c_int,
    ) -> *mut cairo_sys::cairo_surface_t;

    /// Get a string that can be used for completion. It should have no markup.
    ///
    /// Returns a newly allocated string.
    ///
    /// - `selected_line`: The entry to query
    pub fn mode_get_completion(mode: *const Mode, selected_line: c_uint) -> *const c_char;

    /// Acts on the user interaction.
    ///
    /// Returns the next mode state.
    ///
    /// - `menu_retv`: The menu return value.
    /// - `input`: Pointer to the user input string. \[in\] \[out\]
    /// - `selected_line`: The line selected by the user.
    pub fn mode_result(
        mode: *mut Mode,
        menu_retv: c_int,
        input: *mut *mut c_char,
        selected_line: c_uint,
    ) -> c_int;

    /// Match entry against the set of tokens.
    ///
    /// Returns TRUE if it matches.
    ///
    /// - `tokens`: The set of tokens to match against.
    /// - `selected_line`: The index of the entry to match.
    pub fn mode_token_match(
        mode: *const Mode,
        tokens: *mut *mut RofiIntMatcher,
        selected_line: c_uint,
    ) -> c_int;

    /// Get the name of the mode.
    pub fn mode_get_name(mode: *const Mode) -> *const c_char;

    /// Free the resources allocated for this mode.
    pub fn mode_free(mode: *mut *mut Mode);

    /// A helper function for modes: get the private data object.
    pub fn mode_get_private_data(mode: *const Mode) -> *mut c_void;

    /// A helper function for modes: set the private data object.
    pub fn mode_set_private_data(mode: *mut Mode, pd: *mut c_void);

    /// Get the name of the mode as it should be presented to the user.
    pub fn mode_get_display_name(mode: *const Mode) -> *const c_char;

    /// Adds the display-name configuration option for the mode.
    /// Should be called once for each mode.
    pub fn mode_set_config(mode: *mut Mode);

    /// Process the input so it can be used for matching and sorting.
    /// This includes removing Pango markup.
    ///
    /// Returns a newly allocated string.
    ///
    /// - `input`: The input to process.
    pub fn mode_preprocess_input(mode: *mut Mode, input: *const c_char) -> *const c_char;

    /// Query the mode for a user display.
    ///
    /// Returns a newly allocated (valid Pango markup) message to display,
    /// which the user must free.
    pub fn mode_get_message(mode: *const Mode) -> *const c_char;
}
