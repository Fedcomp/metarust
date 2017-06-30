use std::os::raw::*;
use std::ffi::{CStr};

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[allow(dead_code)]
enum PLUG_LOADTIME {
    PT_NEVER,
	PT_STARTUP,		// should only be loaded/unloaded at initial hlds execution
	PT_CHANGELEVEL, // can be loaded/unloaded between maps
	PT_ANYTIME,		// can be loaded/unloaded at any time
	PT_ANYPAUSE		// can be loaded/unloaded at any time, and can be "paused" during a map
}

#[repr(C)]
#[derive(Debug)]
pub struct plugin_info_t {
    ifvers:  *const c_char,
    name:    *const c_char,
    version: *const c_char,
    date:    *const c_char,
    author:  *const c_char,
    url:     *const c_char,
    logtag:  *const c_char,
    loadable: i32,
    unloadable: i32
}

macro_rules! cstr {
  ($s:expr) => (
    concat!($s, "\0") as *const str as *const [c_char] as *const c_char
  );
}

const META_INTERFACE_VERSION: *const c_char = cstr!("5:13");

const PLUGIN_INFO: plugin_info_t = plugin_info_t {
    ifvers:  META_INTERFACE_VERSION,
    name:    cstr!("MetaRust"),
    version: cstr!("0.0.1"),
    date:    cstr!("30.06.2017"),
    author:  cstr!("Fedcomp"),
    url:     cstr!("http://amx-x/ru"),
    logtag:  cstr!("METARUST"),
    loadable: PLUG_LOADTIME::PT_CHANGELEVEL as i32,
    unloadable: PLUG_LOADTIME::PT_CHANGELEVEL as i32
};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Meta_Attach() -> c_int {
    1
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Meta_Dettach() -> c_int {
    1
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Meta_Query(raw_interface_version: *const c_char, pinfo: *mut *const plugin_info_t, _mutil_funcs: c_char) -> c_int {
    let _interface_version = CStr::from_ptr(raw_interface_version);
    *pinfo = &PLUGIN_INFO;
    1
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn GiveFnptrsToDll() -> () {
    ()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
