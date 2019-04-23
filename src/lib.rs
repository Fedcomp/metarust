#![cfg_attr(feature = "strict", deny(warnings))]

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use cstr_macro::cstr;
use metamod_bindgen::{enginefuncs_t, gamedll_funcs_t, globalvars_t, meta_globals_t};
use metamod_sys::{
    plugin_info_t, META_FUNCTIONS, META_INTERFACE_VERSION,
    PLUG_LOADTIME::{self, PT_CHANGELEVEL},
};

const PLUGIN_INFO: plugin_info_t = plugin_info_t {
    ifvers: META_INTERFACE_VERSION,
    name: cstr!("MetaRust"),
    version: cstr!("0.0.1"),
    date: cstr!("23.04.2019"),
    author: cstr!("Fedcomp"),
    url: cstr!("http://amx-x.ru"),
    logtag: cstr!("METARUST"),
    loadable: PT_CHANGELEVEL,
    unloadable: PT_CHANGELEVEL,
};

const META_FUNCTIONS_TABLE: META_FUNCTIONS = META_FUNCTIONS {
    pfnGetEntityAPI: None,
    pfnGetEntityAPI_Post: None,
    pfnGetEntityAPI2: None,
    pfnGetEntityAPI2_Post: None,
    pfnGetNewDLLFunctions: None,
    pfnGetNewDLLFunctions_Post: None,
    pfnGetEngineFunctions: None,
    pfnGetEngineFunctions_Post: None,
};

static mut gpGlobals: Option<*const globalvars_t> = None;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Meta_Attach(
    _plug_loadtime: PLUG_LOADTIME,
    pFunctionTable: *mut META_FUNCTIONS,
    _pMGlobals: *const meta_globals_t,
    _pGamedllFuncs: *const gamedll_funcs_t,
) -> c_int {
    *pFunctionTable = META_FUNCTIONS_TABLE;
    1
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Meta_Detach() -> c_int {
    1
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Meta_Query(
    ifvers: *const c_char,
    pinfo: *mut *const plugin_info_t,
    _mutil_funcs: c_char,
) -> c_int {
    let _interface_version = CStr::from_ptr(ifvers);
    *pinfo = &PLUGIN_INFO;
    1
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn GiveFnptrsToDll(
    _pengfuncsFromEngine: *const enginefuncs_t,
    pGlobals: *const globalvars_t,
) {
    gpGlobals = Some(pGlobals);
}
