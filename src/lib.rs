#![cfg_attr(feature = "strict", deny(warnings))]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use cstr_macro::cstr;
use hlsdk_sys::DLL_FUNCTIONS;
use metamod_bindgen::{enginefuncs_t, gamedll_funcs_t, globalvars_t};
use metamod_sys::{
    meta_globals_t, plugin_info_t, GETENTITYAPI_FN_INTERFACE_VERSION, META_FUNCTIONS,
    META_INTERFACE_VERSION,
    META_RES::*,
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
    pfnGetEntityAPI: Some(get_entity_api),
    pfnGetEntityAPI_Post: None,
    pfnGetEntityAPI2: None,
    pfnGetEntityAPI2_Post: None,
    pfnGetNewDLLFunctions: None,
    pfnGetNewDLLFunctions_Post: None,
    pfnGetEngineFunctions: None,
    pfnGetEngineFunctions_Post: None,
};

const gFunctionTable: DLL_FUNCTIONS = DLL_FUNCTIONS {
    pfnGameInit: None,
    pfnSpawn: None,
    pfnThink: None,
    pfnUse: None,
    pfnTouch: None,
    pfnBlocked: None,
    pfnKeyValue: None,
    pfnSave: None,
    pfnRestore: None,
    pfnSetAbsBox: None,
    pfnSaveWriteFields: None,
    pfnSaveReadFields: None,
    pfnSaveGlobalState: None,
    pfnRestoreGlobalState: None,
    pfnResetGlobalState: None,
    pfnClientConnect: None,
    pfnClientDisconnect: None,
    pfnClientKill: None,
    pfnClientPutInServer: None,
    pfnClientCommand: None,
    pfnClientUserInfoChanged: None,
    pfnServerActivate: None,
    pfnServerDeactivate: None,
    pfnPlayerPreThink: None,
    pfnPlayerPostThink: None,
    pfnStartFrame: Some(start_frame),
    pfnParmsNewLevel: None,
    pfnParmsChangeLevel: None,
    pfnGetGameDescription: None,
    pfnPlayerCustomization: None,
    pfnSpectatorConnect: None,
    pfnSpectatorDisconnect: None,
    pfnSpectatorThink: None,
    pfnSys_Error: None,
    pfnPM_Move: None,
    pfnPM_Init: None,
    pfnPM_FindTextureType: None,
    pfnSetupVisibility: None,
    pfnUpdateClientData: None,
    pfnAddToFullPack: None,
    pfnCreateBaseline: None,
    pfnRegisterEncoders: None,
    pfnGetWeaponData: None,
    pfnCmdStart: None,
    pfnCmdEnd: None,
    pfnConnectionlessPacket: None,
    pfnGetHullBounds: None,
    pfnCreateInstancedBaselines: None,
    pfnInconsistentFile: None,
    pfnAllowLagCompensation: None,
};

static mut gpGlobals: Option<&globalvars_t> = None;
static mut gpMetaGlobals: Option<&mut meta_globals_t> = None;

pub unsafe extern "C" fn get_entity_api(
    pFunctionTable: *mut DLL_FUNCTIONS,
    interfaceVersion: c_int,
) -> c_int {
    if interfaceVersion != GETENTITYAPI_FN_INTERFACE_VERSION {
        panic!(
            "Inconsistent GETENTITYAPI_FN_INTERFACE_VERSION, theirs: {}, ours: {}",
            interfaceVersion, GETENTITYAPI_FN_INTERFACE_VERSION
        )
    }

    // Fill our pre hooks list
    *pFunctionTable = gFunctionTable;

    1
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Meta_Attach(
    _plug_loadtime: PLUG_LOADTIME,
    pFunctionTable: *mut META_FUNCTIONS,
    pMGlobals: *mut meta_globals_t,
    _pGamedllFuncs: *const gamedll_funcs_t,
) -> c_int {
    *pFunctionTable = META_FUNCTIONS_TABLE;
    gpMetaGlobals = Some(&mut *pMGlobals);
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
    gpGlobals = Some(&*pGlobals);
}

pub unsafe extern "C" fn start_frame() {
    let meta_globals = gpMetaGlobals
        .as_mut()
        .expect("Meta globals should be already initialized to this moment");

    meta_globals.mres = MRES_IGNORED;
}
