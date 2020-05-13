mod metamod;

use crate::metamod::{PluginInfo, PluginLoadTime, METAMOD_INTERFACE_VERSION};
use hlsdk_sys::extdll::{BOOL, TRUE, FALSE};
use std::os::raw::c_char;

const PLUGIN_INFO: PluginInfo = PluginInfo {
    interface_version: METAMOD_INTERFACE_VERSION,
    name: cstr!(env!("CARGO_PKG_NAME")),
    version: cstr!(env!("CARGO_PKG_VERSION")),
    date: cstr!(env!("CARGO_BUILD_DATE")),
    author: cstr!(env!("CARGO_PKG_AUTHORS")),
    url: cstr!(env!("CARGO_PKG_HOMEPAGE")),
    logtag: cstr!(env!("CARGO_PKG_NAME")),
    loadable: PluginLoadTime::Startup,
    unloadable: PluginLoadTime::Startup,
};

const LOGTAG: &str = concat!("[", env!("CARGO_PKG_NAME"), "] ");

#[no_mangle]
pub extern "C" fn Meta_Attach(/* TODO */) -> BOOL {
    TRUE
}

#[no_mangle]
pub extern "C" fn Meta_Detach(/* TODO */) -> BOOL {
    TRUE
}

#[no_mangle]
pub extern "C" fn Meta_Query(
    interface_version: *const c_char,
    plugin_info: *mut *const PluginInfo, /* TODO: mutil_funcs_t *pMetaUtilFuncs */
) -> BOOL {
    // TODO: Check only major version compatibility?
    let ifvers = unsafe { std::ffi::CStr::from_ptr(interface_version) }.to_string_lossy().to_string();
    let expected_ifvers = unsafe { std::ffi::CStr::from_ptr(METAMOD_INTERFACE_VERSION) }.to_string_lossy().to_string();
    if ifvers != expected_ifvers {
        eprintln!("{}Metamod interface mismatch, expected: {}, received: {}", LOGTAG, expected_ifvers, ifvers);
        return FALSE;
    }

    unsafe { *plugin_info = &PLUGIN_INFO };
    TRUE
}

#[no_mangle]
pub unsafe extern "C" fn GiveFnptrsToDll(/* TODO */) {}
