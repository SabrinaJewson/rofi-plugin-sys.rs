#![warn(unsafe_op_in_unsafe_fn)]

use ::{
    rofi_plugin_sys::glib_sys,
    std::{
        ffi::{c_void, CString},
        os::raw::{c_char, c_int, c_uint},
        ptr,
    },
};

#[no_mangle]
pub static mut mode: rofi_plugin_sys::Mode = rofi_plugin_sys::Mode {
    name: "plugin-example-basic\0".as_ptr() as _,
    cfg_name_key: {
        let display_name = b"A basic Rofi plugin";
        let mut buf = [0; 128];
        let mut i = 0;
        while i < display_name.len() {
            buf[i] = display_name[i] as c_char;
            i += 1;
        }
        buf
    },
    _init: Some(init),
    _destroy: Some(destroy),
    _get_num_entries: Some(get_num_entries),
    _result: Some(result),
    _token_match: Some(token_match),
    _get_display_value: Some(get_display_value),
    ..rofi_plugin_sys::Mode::default()
};

const _: [(); 264] = [(); std::mem::size_of::<rofi_plugin_sys::Mode>()];

unsafe extern "C" fn init(sw: *mut rofi_plugin_sys::Mode) -> c_int {
    if unsafe { mode_state(sw) }.is_null() {
        let state = State {
            entries: vec![CString::new("foo").unwrap(), CString::new("bar").unwrap()],
        };
        let ptr = Box::into_raw(Box::new(state)).cast::<c_void>();
        unsafe { rofi_plugin_sys::mode_set_private_data(sw, ptr) };
    }
    c_int::from(true)
}

unsafe extern "C" fn destroy(sw: *mut rofi_plugin_sys::Mode) {
    println!("Destroy!");
    let ptr = unsafe { mode_state_mut(sw) };
    if ptr.is_null() {
        return;
    }
    drop(unsafe { Box::from_raw(ptr) });
    unsafe { rofi_plugin_sys::mode_set_private_data(sw, ptr::null_mut()) };
}

unsafe extern "C" fn get_num_entries(sw: *const rofi_plugin_sys::Mode) -> c_uint {
    unsafe { &*mode_state(sw) }
        .entries
        .len()
        .try_into()
        .unwrap()
}

unsafe extern "C" fn result(
    _sw: *mut rofi_plugin_sys::Mode,
    mretv: c_int,
    _input: *mut *mut c_char,
    _selected_line: c_uint,
) -> c_int {
    if mretv & rofi_plugin_sys::menu::NEXT != 0 {
        rofi_plugin_sys::NEXT_DIALOG
    } else if mretv & rofi_plugin_sys::menu::PREVIOUS != 0 {
        rofi_plugin_sys::PREVIOUS_DIALOG
    } else if mretv & rofi_plugin_sys::menu::QUICK_SWITCH != 0 {
        mretv & rofi_plugin_sys::menu::LOWER_MASK
    } else if mretv & rofi_plugin_sys::menu::OK != 0
        || mretv & rofi_plugin_sys::menu::ENTRY_DELETE == rofi_plugin_sys::menu::ENTRY_DELETE
    {
        rofi_plugin_sys::RELOAD_DIALOG
    } else {
        rofi_plugin_sys::EXIT
    }
}

unsafe extern "C" fn get_display_value(
    sw: *const rofi_plugin_sys::Mode,
    selected_line: c_uint,
    _state: *mut c_int,
    _attr_list: *mut *mut glib_sys::GList,
    get_entry: c_int,
) -> *mut c_char {
    let state = unsafe { &*mode_state(sw) };

    if get_entry != 0 {
        let displayed: *const c_char = state.entries[selected_line as usize].as_ptr();
        unsafe { glib_sys::g_strdup(displayed) }
    } else {
        ptr::null_mut()
    }
}

unsafe extern "C" fn token_match(
    sw: *const rofi_plugin_sys::Mode,
    tokens: *mut *mut rofi_plugin_sys::RofiIntMatcher,
    index: c_uint,
) -> c_int {
    let state = unsafe { &*mode_state(sw) };

    unsafe { rofi_plugin_sys::helper::token_match(tokens, state.entries[index as usize].as_ptr()) }
}

unsafe fn mode_state(sw: *const rofi_plugin_sys::Mode) -> *const State {
    unsafe { rofi_plugin_sys::mode_get_private_data(sw) }.cast()
}

unsafe fn mode_state_mut(sw: *mut rofi_plugin_sys::Mode) -> *mut State {
    unsafe { rofi_plugin_sys::mode_get_private_data(sw) }.cast() as _
}

struct State {
    entries: Vec<CString>,
}
