#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::convert::TryFrom;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use cstr_macro::cstr;
use hlsdk_sys::{edict_t, BOOL, DLL_FUNCTIONS, TRUE};
use metamod_bindgen::{enginefuncs_t, gamedll_funcs_t, globalvars_t};
use metamod_sys::{
    meta_globals_t, plugin_info_t, GETENTITYAPI_FN_INTERFACE_VERSION, META_FUNCTIONS, META_RES::*,
    PLUG_LOADTIME,
};

use super::{PluginInfo, PluginInfoOwned};
use crate::forwards;

const gMetaFunctionTable: META_FUNCTIONS = META_FUNCTIONS {
    pfnGetEntityAPI: None,
    pfnGetEntityAPI_Post: None,
    pfnGetEntityAPI2: Some(get_entity_api2),
    pfnGetEntityAPI2_Post: Some(get_entity_api2_post),
    pfnGetNewDLLFunctions: None,
    pfnGetNewDLLFunctions_Post: None,
    pfnGetEngineFunctions: None,
    pfnGetEngineFunctions_Post: None,
};

static mut gFunctionTable: DLL_FUNCTIONS = DLL_FUNCTIONS {
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
    pfnStartFrame: None,
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

static mut gFunctionTable_Post: DLL_FUNCTIONS = DLL_FUNCTIONS {
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
    pfnServerActivate: Some(server_activate_post),
    pfnServerDeactivate: None,
    pfnPlayerPreThink: None,
    pfnPlayerPostThink: None,
    pfnStartFrame: None,
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

pub static mut gpGlobals: Option<&globalvars_t> = None;
pub static mut gpMetaGlobals: Option<&mut meta_globals_t> = None;
static mut PLUGIN_INFO_OWNED: Option<PluginInfoOwned> = None;
static mut PLUGIN_INFO_RAW: Option<plugin_info_t> = None;
pub static mut PLUGIN_FORWARDS: Option<forwards::Forwards> = None;

/* Initialization pointer/hook processing functions */

// Inline all functions into proxy versions
#[inline(always)]
pub unsafe extern "C" fn Meta_Attach(
    _plug_loadtime: PLUG_LOADTIME,
    pFunctionTable: *mut META_FUNCTIONS,
    pMGlobals: *mut meta_globals_t,
    _pGamedllFuncs: *const gamedll_funcs_t,
) -> BOOL {
    *pFunctionTable = gMetaFunctionTable;
    gpMetaGlobals = Some(&mut *pMGlobals);

    TRUE
}

#[inline(always)]
pub extern "C" fn Meta_Detach() -> BOOL {
    TRUE
}

#[inline(always)]
pub unsafe extern "C" fn Meta_Query(
    ifvers: *const c_char,
    pinfo: *mut *const plugin_info_t,
    _mutil_funcs: c_char,
    // !Warning! not present in original version, used purely for metarust purposes
    metarust_plugin_info: &PluginInfo,
    metarust_forwards: &forwards::Forwards,
) -> BOOL {
    // TODO Check interface version is correct
    let _interface_version = CStr::from_ptr(ifvers);

    PLUGIN_FORWARDS = Some(metarust_forwards.clone());

    // Map plugin requested forwards to raw versions
    forwards::map_requested_forwards(
        PLUGIN_FORWARDS.as_ref().unwrap(),
        &mut gFunctionTable,
        &mut gFunctionTable_Post,
    );

    // TODO: Error handling
    PLUGIN_INFO_OWNED = Some(PluginInfoOwned::try_from(metarust_plugin_info).unwrap());
    PLUGIN_INFO_RAW = Some(PLUGIN_INFO_OWNED.as_ref().unwrap().as_plugin_info_t());
    println!("PASSING PLUGIN_INFO: {:?}", PLUGIN_INFO_RAW);
    *pinfo = PLUGIN_INFO_RAW.as_ref().unwrap();

    TRUE
}

#[inline(always)]
pub unsafe extern "C" fn GiveFnptrsToDll(
    _pengfuncsFromEngine: *const enginefuncs_t,
    pGlobals: *const globalvars_t,
) {
    gpGlobals = Some(&*pGlobals);
}

pub unsafe extern "C" fn get_entity_api2(
    pFunctionTable: *mut DLL_FUNCTIONS,
    interfaceVersion: *const c_int,
) -> BOOL {
    // TODO: Make fail handling as in metamod plugin example
    if *interfaceVersion != GETENTITYAPI_FN_INTERFACE_VERSION {
        panic!(
            "Inconsistent GETENTITYAPI_FN_INTERFACE_VERSION, theirs: {}, ours: {}",
            *interfaceVersion, GETENTITYAPI_FN_INTERFACE_VERSION
        )
    }

    // Return our hook list to engine
    *pFunctionTable = gFunctionTable.clone();

    TRUE
}

pub unsafe extern "C" fn get_entity_api2_post(
    pFunctionTable: *mut DLL_FUNCTIONS,
    interfaceVersion: *const c_int,
) -> BOOL {
    // TODO: Make fail handling as in metamod plugin example
    if *interfaceVersion != GETENTITYAPI_FN_INTERFACE_VERSION {
        panic!(
            "Inconsistent GETENTITYAPI_FN_INTERFACE_VERSION, theirs: {}, ours: {}",
            *interfaceVersion, GETENTITYAPI_FN_INTERFACE_VERSION
        )
    }

    // Return our hook list to engine
    *pFunctionTable = gFunctionTable_Post;

    TRUE
}

/* Library defined hooks */

use hlsdk_sys::string_t;
use std::borrow::Cow;

fn read_string<'a>(string_offset: string_t) -> Cow<'a, str> {
    // Leave me issue or comment where you may read string_t without gpGlobals being initialized
    let globals =
        unsafe { gpGlobals.expect("Engine globals should be already initialized to this moment") };
    let base_offset = globals.pStringBase as string_t;
    let string_t_addr = (base_offset + string_offset) as *const c_char;
    let cstr = unsafe { CStr::from_ptr(string_t_addr) };

    cstr.to_string_lossy()
}

// amxmodx's plugin_init
pub unsafe extern "C" fn server_activate_post(
    _pEdictList: *const edict_t,
    _edictCount: i32,
    _clientMax: i32,
) {
    println!("plugin_init()");
    println!("gpGlobals: {:?}", gpGlobals.unwrap());

    println!("Map name: {:?}", read_string(gpGlobals.unwrap().mapname));

    let meta_globals = gpMetaGlobals
        .as_mut()
        .expect("Meta globals should be already initialized to this moment");

    meta_globals.mres = MRES_IGNORED;
}
