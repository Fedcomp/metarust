use hlsdk_sys::extdll::{BOOL, TRUE};
use metarust::cstr;
use metarust::metamod::{PluginInfo, PluginLoadTime, METAMOD_INTERFACE_VERSION};
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
    _interface_version: *const c_char,
    plugin_info: *mut *const PluginInfo, /* TODO: mutil_funcs_t *pMetaUtilFuncs */
) -> BOOL {
    // TODO
    // let interface_version = unsafe { std::ffi::CStr::from_ptr(interface_version) };
    // if interface_version != METAMOD_INTERFACE_VERSION {
    //     return false;
    // }

    unsafe { *plugin_info = &PLUGIN_INFO };
    TRUE
}

#[no_mangle]
pub unsafe extern "C" fn GiveFnptrsToDll(/* TODO */) {}
