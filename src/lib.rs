#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use(cstr)]
extern crate cstr_macro;
extern crate metamod_bindgen;

pub mod metamod;

use std::os::raw::{c_char, c_int};
use std::ffi::{CStr};

use metamod::PLUG_LOADTIME::PT_CHANGELEVEL;
use metamod::{META_INTERFACE_VERSION, plugin_info_t};
use metamod_bindgen::{META_FUNCTIONS, enginefuncs_t, globalvars_t, meta_globals_t, gamedll_funcs_t};

const PLUGIN_INFO: plugin_info_t = plugin_info_t {
    ifvers:  META_INTERFACE_VERSION,
    name:    cstr!("MetaRust"),
    version: cstr!("0.0.1"),
    date:    cstr!("14.07.2018"),
    author:  cstr!("Fedcomp"),
    url:     cstr!("http://amx-x/ru"),
    logtag:  cstr!("METARUST"),
    loadable: PT_CHANGELEVEL as i32,
    unloadable: PT_CHANGELEVEL as i32
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
pub unsafe extern "C" fn Meta_Attach(plug_loadtime: i32, pFunctionTable: *const META_FUNCTIONS, pMGlobals: *const meta_globals_t, pGamedllFuncs: *const gamedll_funcs_t) -> c_int {
    println!("<<<<< CHECKING GLOBALS >>>>>");
    if let Some(globals) = gpGlobals {
        println!("{:?}", *globals);
        println!("Trying map name");
        let p_map = (*globals).mapname;
        println!("p_map is {:?}", p_map);

        if p_map != 0 {
            let st = CStr::from_ptr(p_map as *const c_char);
            println!("Mapname: {:?}", st);
        }
    }

    1
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Meta_Detach() -> c_int {
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
pub unsafe extern "C" fn GiveFnptrsToDll(pengfuncsFromEngine: *const enginefuncs_t, pGlobals: *const globalvars_t) {
    gpGlobals = Some(pGlobals);
}
