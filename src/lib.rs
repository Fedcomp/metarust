#[macro_use]
extern crate lazy_static;

use std::os::raw::*;
use std::ffi::{CStr, CString};
use std::fmt::Debug;

#[allow(non_camel_case_types)]
#[derive(Debug)]
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
    loadable: i8,
    unloadable: i8
}

unsafe impl Sync for plugin_info_t {}

// pub const META_INTERFACE_VERSION: *const str = "5:13\0";
lazy_static! {
    static ref PLUGIN_INFO: plugin_info_t = {
        plugin_info_t {
            ifvers:  CString::new("5:13").unwrap().as_ptr(),
            name:    CString::new("SimpleMeta").unwrap().as_ptr(),
            version: CString::new("0.0.1").unwrap().as_ptr(),
            date:    CString::new("29.06.2017").unwrap().as_ptr(),
            author:  CString::new("Fedcomp").unwrap().as_ptr(),
            url:     CString::new("http://amx-x/ru").unwrap().as_ptr(),
            logtag:  CString::new("SIMETA").unwrap().as_ptr(),
            loadable: 2,
            unloadable: 2
        }
    };
}

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

    println!("===LOOK AT ME===");
    let vers = CStr::from_ptr((*PLUGIN_INFO).ifvers);
    println!("{:?}", vers);
    *pinfo = &*PLUGIN_INFO;

    // println!("{:?}", *PLUGIN_INFO);
    // *pinfo = PLUGIN_INFO;
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
