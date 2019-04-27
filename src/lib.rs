use metarust::{PluginInfo, PLUG_LOADTIME};
/// Most values are inherited from cargo
const PLUGIN_INFO: PluginInfo = PluginInfo::new(
    "METARUST",
    PLUG_LOADTIME::PT_CHANGELEVEL,
    PLUG_LOADTIME::PT_CHANGELEVEL,
);

use metarust::hlsdk_sys::BOOL;
use metarust::metamod_bindgen::{enginefuncs_t, gamedll_funcs_t, globalvars_t};
use metarust::metamod_sys::{meta_globals_t, plugin_info_t, META_FUNCTIONS};
use metarust::{
    GiveFnptrsToDll as raw_GiveFnptrsToDll, Meta_Attach as raw_Meta_Attach,
    Meta_Detach as raw_Meta_Detach, Meta_Query as raw_Meta_Query,
};
use std::os::raw::c_char;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Meta_Attach(
    plug_loadtime: PLUG_LOADTIME,
    pFunctionTable: *mut META_FUNCTIONS,
    pMGlobals: *mut meta_globals_t,
    pGamedllFuncs: *const gamedll_funcs_t,
) -> BOOL {
    raw_Meta_Attach(plug_loadtime, pFunctionTable, pMGlobals, pGamedllFuncs)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Meta_Detach() -> BOOL {
    raw_Meta_Detach()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Meta_Query(
    ifvers: *const c_char,
    pinfo: *mut *const plugin_info_t,
    mutil_funcs: c_char,
) -> BOOL {
    raw_Meta_Query(ifvers, pinfo, mutil_funcs)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn GiveFnptrsToDll(
    pengfuncsFromEngine: *const enginefuncs_t,
    pGlobals: *const globalvars_t,
) {
    raw_GiveFnptrsToDll(pengfuncsFromEngine, pGlobals)
}
