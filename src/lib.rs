#[macro_use]
pub mod metamod;

use std::os::raw::{c_char, c_int};
use std::ffi::{CStr};

use metamod::PLUG_LOADTIME::PT_CHANGELEVEL;
use metamod::{META_INTERFACE_VERSION, plugin_info_t};

const PLUGIN_INFO: plugin_info_t = plugin_info_t {
    ifvers:  META_INTERFACE_VERSION,
    name:    cstr!("MetaRust"),
    version: cstr!("0.0.1"),
    date:    cstr!("30.06.2017"),
    author:  cstr!("Fedcomp"),
    url:     cstr!("http://amx-x/ru"),
    logtag:  cstr!("METARUST"),
    loadable: PT_CHANGELEVEL as i32,
    unloadable: PT_CHANGELEVEL as i32
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
pub unsafe extern "C" fn Meta_Query(ifvers: *const c_char, pinfo: *mut *const plugin_info_t, _mutil_funcs: c_char) -> c_int {
    let _interface_version = CStr::from_ptr(ifvers);
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
