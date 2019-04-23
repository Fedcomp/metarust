use std::os::raw::c_char;

use cstr_macro::cstr;

#[allow(non_camel_case_types)]
pub enum PLUG_LOADTIME {
    PT_NEVER,
    PT_STARTUP,     // should only be loaded/unloaded at initial hlds execution
    PT_CHANGELEVEL, // can be loaded/unloaded between maps
    PT_ANYTIME,     // can be loaded/unloaded at any time
    PT_ANYPAUSE,    // can be loaded/unloaded at any time, and can be "paused" during a map
}

pub const META_INTERFACE_VERSION: *const c_char = cstr!("5:13");

#[repr(C)]
pub struct plugin_info_t {
    pub ifvers: *const c_char,
    pub name: *const c_char,
    pub version: *const c_char,
    pub date: *const c_char,
    pub author: *const c_char,
    pub url: *const c_char,
    pub logtag: *const c_char,
    pub loadable: i32,
    pub unloadable: i32,
}
