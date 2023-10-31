//! Bindings to mode-private.h

use {
    crate::types::RofiIntMatcher,
    ::std::{
        ffi::c_void,
        os::raw::{c_char, c_int, c_uint},
        ptr,
    },
};

#[cfg(any(doc, rofi_next))]
use ::{bitflags::bitflags, std::mem};

/// ABI version to check if loaded plugin is compatible.
///
/// When `cfg(rofi_next)` is enabled, this is 7; otherwise, it is 6.
pub const ABI_VERSION: c_uint = if cfg!(rofi_next) { 7 } else { 6 };

#[cfg(any(doc, rofi_next))]
bitflags! {
    /// Indicator what type of mode this is.
    /// For now it can be the classic switcher, or also implement a completer.
    ///
    /// **Semver-exempt and only available with `cfg(rofi_next)`.**
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ModeType: c_uint {
        /// Mode type is not set.
        const UNSET = 0x0;
        /// A normal mode.
        const SWITCHER = 0x1;
        /// A mode that can be used to completer.
        const COMPLETER = 0x2;
        /// DMenu mode.
        const DMENU = 0x4;
    }
}

#[repr(C)]
#[cfg(rofi_next)]
#[allow(dead_code)]
enum ModeTypeSizeChecker {
    Unset = 0,
    Switcher = 1,
    Completer = 2,
}

// Make sure the enum's sizes match
#[cfg(rofi_next)]
const _: [(); mem::size_of::<ModeType>()] = [(); mem::size_of::<ModeTypeSizeChecker>()];

/// Free the switcher.
///
/// Only to be used when the switcher object itself is dynamic and has data in [`Mode::ed`].
pub type ModeFree = Option<unsafe extern "C" fn(data: *mut Mode)>;

/// Get the string to display for the entry.
///
/// Returns the string and state for displaying.
///
/// - `selected_line`: The selected line
/// - `state`: The state to display \[out\]
/// - `attribute_list`: List of extra (pango) attributes to apply when displaying. \[out\] \[null\]
/// - `get_entry`: If it should only return the state
pub type ModeGetDisplayValue = Option<
    unsafe extern "C" fn(
        sw: *const Mode,
        selected_line: c_uint,
        state: *mut c_int,
        attribute_list: *mut *mut glib_sys::GList,
        get_entry: c_int,
    ) -> *mut c_char,
>;

/// Obtains the icon of the entry if available.
///
/// - `selected_line`: The selected line.
pub type ModeGetIcon = Option<
    unsafe extern "C" fn(
        sw: *const Mode,
        selected_line: c_uint,
        height: c_int,
    ) -> *mut cairo_sys::cairo_surface_t,
>;

/// Obtains the string to complete.
///
/// - `selected_line`: The selected line
pub type ModeGetCompletion =
    Option<unsafe extern "C" fn(sw: *const Mode, selected_line: c_uint) -> *mut c_char>;

/// Token match for the matching algorithm.
///
/// Returns 1 when it matches, 0 if not.
///
/// - `tokens`: List of (input) tokens to match.
/// - `input`: The entry to match against.
/// - `case_sensitive`: Whether case is significant.
/// - `index`: The current selected index.
/// - `data`: User data.
pub type ModeTokenMatch = Option<
    unsafe extern "C" fn(sw: *const Mode, tokens: *mut *mut RofiIntMatcher, index: c_uint) -> c_int,
>;

/// Initialize the Mode.
///
/// Returns `true` if successful.
pub type ModeInit = Option<unsafe extern "C" fn(sw: *mut Mode) -> c_int>;

/// Get the number of entries to display (unfiltered).
pub type ModeGetNumEntries = Option<unsafe extern "C" fn(sw: *const Mode) -> c_uint>;

/// Destroy the current mode. Still ready to restart.
pub type ModeDestroy = Option<unsafe extern "C" fn(sw: *mut Mode)>;

/// Process the result of the user selection.
///
/// Returns the next action to take.
///
/// - `menu_retv`: The return value
/// - `input`: The input string
/// - `selected_line`: The selected line
pub type ModeResult = Option<
    unsafe extern "C" fn(
        sw: *mut Mode,
        menu_retv: c_int,
        input: *mut *mut c_char,
        selected_line: c_uint,
    ) -> c_int,
>;

/// Preprocess the input for sorting.
///
/// Returns the entry stripped from markup for sorting.
///
/// - `input`: The input string
pub type ModePreprocessInput =
    Option<unsafe extern "C" fn(sw: *mut Mode, input: *const c_char) -> *mut c_char>;

/// Message to show in the message bar.
///
/// Returns the (valid Pango markup) message to display.
pub type ModeGetMessage = Option<unsafe extern "C" fn(sw: *const Mode) -> *mut c_char>;

/// Create a new instance of this mode.
/// Free (free) result after use, after using `mode_destroy`.
///
/// **Semver-exempt and only available with `cfg(rofi_next)`.**
#[cfg(any(doc, rofi_next))]
pub type ModeCreate = Option<unsafe extern "C" fn() -> *mut Mode>;

/// Handle the user accepting an entry in completion mode.
/// Returns the next action to take.
/// - `sw`: The mode pointer
/// - `menu_retv`: The return value
/// - `input`: The input string
/// - `selected_line`: The selected line
/// - `path`: the path that was completed
///
/// **Semver-exempt and only available with `cfg(rofi_next)`.**
#[cfg(any(doc, rofi_next))]
pub type ModeCompleterResult = Option<
    unsafe extern "C" fn(*mut Mode, c_int, *mut *mut c_char, c_uint, *mut *mut c_char) -> c_uint,
>;

/// Structure defining a switcher.
///
/// Access should be done through `mode_*` functions,
/// not the function pointer fields on this type.
///
/// It consists of a name, callback and if enabled a textbox for the sidebar-mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Mode {
    /// Used for external plugins.
    ///
    /// You should set this to [`ABI_VERSION`].
    pub abi_version: c_uint,

    /// Name (max 31 char long)
    pub name: *mut c_char,
    pub cfg_name_key: [c_char; 128],
    pub display_name: *mut c_char,

    /// Initialize the Mode.
    ///
    /// Returns `true` if successful.
    pub _init: ModeInit,

    /// Destroy the switcher, e.g. free all its memory. Still ready to restart.
    pub _destroy: ModeDestroy,

    /// Get the number of entries to display (unfiltered).
    pub _get_num_entries: ModeGetNumEntries,

    /// Process the result of the user selection.
    pub _result: ModeResult,

    /// Token match for the matching algorithm.
    pub _token_match: ModeTokenMatch,

    /// Get the string to display for the entry.
    pub _get_display_value: ModeGetDisplayValue,

    /// Obtains the icon of the entry if available.
    pub _get_icon: ModeGetIcon,

    /// Obtains the string to complete.
    pub _get_completion: ModeGetCompletion,

    /// Preprocess the input for sorting.
    pub _preprocess_input: ModePreprocessInput,

    /// Message to show in the message bar.
    pub _get_message: ModeGetMessage,

    /// Pointer to private data.
    pub private_data: *mut c_void,

    /// Free the switcher.
    pub free: ModeFree,

    /// Create mode.
    ///
    /// **Semver-exempt and only available with `cfg(rofi_next)`.**
    #[cfg(any(doc, rofi_next))]
    pub _create: ModeCreate,

    /// If this mode is used as a completer.
    ///
    /// **Semver-exempt and only available with `cfg(rofi_next)`.**
    #[cfg(any(doc, rofi_next))]
    pub _completer_result: ModeCompleterResult,

    /// Extra fields for the script.
    pub ed: *mut c_void,

    /// Module
    pub module: *mut GModule,

    /// Fallback icon
    pub fallback_icon_fetch_uid: u32,

    /// Fallback icon
    pub fallback_icon_not_found: u32,

    /// Type.
    ///
    /// **Semver-exempt and only available with `cfg(rofi_next)`.**
    #[cfg(any(doc, rofi_next))]
    pub r#type: ModeType,
}

impl Mode {
    const DEFAULT: Self = Self {
        abi_version: ABI_VERSION,
        name: ptr::null_mut(),
        cfg_name_key: [0; 128],
        display_name: ptr::null_mut(),
        _init: None,
        _destroy: None,
        _get_num_entries: None,
        _result: None,
        _token_match: None,
        _get_display_value: None,
        _get_icon: None,
        _get_completion: None,
        _preprocess_input: None,
        _get_message: None,
        private_data: ptr::null_mut(),
        free: None,
        #[cfg(any(doc, rofi_next))]
        _create: None,
        #[cfg(any(doc, rofi_next))]
        _completer_result: None,
        ed: ptr::null_mut(),
        module: ptr::null_mut(),
        fallback_icon_fetch_uid: 0,
        fallback_icon_not_found: 0,
        #[cfg(any(doc, rofi_next))]
        r#type: ModeType::SWITCHER,
    };

    /// Create a [`Mode`] with all `None`/null fields.
    pub const fn default() -> Self {
        Self::DEFAULT
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// Mode needs to be put in a static
unsafe impl Sync for Mode {}

/// An opaque C type from GLib.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GModule {
    _unused: [u8; 0],
}
