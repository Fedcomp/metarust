#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int};

use cstr_macro::cstr;

/// Metamod binary interface version.
/// Any metamod implementation with interface lower
/// than this will fail to load plugin when used these bindings
/// Current version is "5:13"
pub const META_INTERFACE_VERSION: *const c_char = cstr!("5:13");

/// GETENTITYAPI_FN Interface version.
/// This value is received during research and may not be correct.
pub const GETENTITYAPI_FN_INTERFACE_VERSION: c_int = 140;

/// When metamod plugin can be loaded and unloaded
#[repr(u32)]
pub enum PLUG_LOADTIME {
    /// After loaded, should never be unloaded (?)
    PT_NEVER = 0,
    /// should only be loaded/unloaded at initial hlds execution
    PT_STARTUP = 1,
    /// can be loaded/unloaded between maps
    PT_CHANGELEVEL = 2,
    /// can be loaded/unloaded at any time
    PT_ANYTIME = 3,
    /// can be loaded/unloaded at any time, and can be "paused" during a map
    PT_ANYPAUSE = 4,
}

/// Basic information about plugin for metamod.
/// Contains information date for end user and plugin load/unload data
#[repr(C)]
pub struct plugin_info_t {
    /// meta_interface version. See [META_INTERFACE_VERSION](constant.META_INTERFACE_VERSION.html)
    pub ifvers: *const c_char,
    /// full name of plugin
    pub name: *const c_char,
    /// plugin version
    pub version: *const c_char,
    /// plugin date
    pub date: *const c_char,
    /// author name/email
    pub author: *const c_char,
    /// plugin URL
    pub url: *const c_char,
    /// log message prefix (unused right now)
    pub logtag: *const c_char,
    /// when plugin is loadable
    pub loadable: PLUG_LOADTIME,
    /// when plugin is unloadable
    pub unloadable: PLUG_LOADTIME,
}

/// Bindings are work in progress undone definitions are marked as unfinished (and not working of course)
// TODO: Specify all definitions and remove
type UNFINISHED_FUNCTION = unsafe extern "C" fn();

/// see UNFINISHED_FUNCTION
type UNFINISHED_FUNCTION_POINTER = Option<UNFINISHED_FUNCTION>;

pub type GETENTITYAPI_FN =
    unsafe extern "C" fn(pFunctionTable: *mut DLL_FUNCTIONS, interfaceVersion: c_int) -> c_int;

#[repr(C)]
#[derive(Debug)]
pub struct META_FUNCTIONS {
    pub pfnGetEntityAPI: Option<GETENTITYAPI_FN>,
    pub pfnGetEntityAPI_Post: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEntityAPI2: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEntityAPI2_Post: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetNewDLLFunctions: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetNewDLLFunctions_Post: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEngineFunctions: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetEngineFunctions_Post: UNFINISHED_FUNCTION_POINTER,
}

// TODO: Description
type fnGameInit = unsafe extern "C" fn();
// TODO: Description
type fnStartFrame = unsafe extern "C" fn();
// TODO: Description
type fnParmsNewLevel = unsafe extern "C" fn();
// TODO: Description
type fnParmsChangeLevel = unsafe extern "C" fn();
// TODO: Description
type fnRegisterEncoders = unsafe extern "C" fn();
// TODO: Description
type fnCreateInstancedBaselines = unsafe extern "C" fn();

#[repr(C)]
#[derive(Debug)]
pub struct DLL_FUNCTIONS {
    pub pfnGameInit: Option<fnGameInit>,
    pub pfnSpawn: UNFINISHED_FUNCTION_POINTER,
    pub pfnThink: UNFINISHED_FUNCTION_POINTER,
    pub pfnUse: UNFINISHED_FUNCTION_POINTER,
    pub pfnTouch: UNFINISHED_FUNCTION_POINTER,
    pub pfnBlocked: UNFINISHED_FUNCTION_POINTER,
    pub pfnKeyValue: UNFINISHED_FUNCTION_POINTER,
    pub pfnSave: UNFINISHED_FUNCTION_POINTER,
    pub pfnRestore: UNFINISHED_FUNCTION_POINTER,
    pub pfnSetAbsBox: UNFINISHED_FUNCTION_POINTER,
    pub pfnSaveWriteFields: UNFINISHED_FUNCTION_POINTER,
    pub pfnSaveReadFields: UNFINISHED_FUNCTION_POINTER,
    pub pfnSaveGlobalState: UNFINISHED_FUNCTION_POINTER,
    pub pfnRestoreGlobalState: UNFINISHED_FUNCTION_POINTER,
    pub pfnResetGlobalState: UNFINISHED_FUNCTION_POINTER,
    pub pfnClientConnect: UNFINISHED_FUNCTION_POINTER,
    pub pfnClientDisconnect: UNFINISHED_FUNCTION_POINTER,
    pub pfnClientKill: UNFINISHED_FUNCTION_POINTER,
    pub pfnClientPutInServer: UNFINISHED_FUNCTION_POINTER,
    pub pfnClientCommand: UNFINISHED_FUNCTION_POINTER,
    pub pfnClientUserInfoChanged: UNFINISHED_FUNCTION_POINTER,
    pub pfnServerActivate: UNFINISHED_FUNCTION_POINTER,
    pub pfnServerDeactivate: UNFINISHED_FUNCTION_POINTER,
    pub pfnPlayerPreThink: UNFINISHED_FUNCTION_POINTER,
    pub pfnPlayerPostThink: UNFINISHED_FUNCTION_POINTER,
    pub pfnStartFrame: Option<fnStartFrame>,
    pub pfnParmsNewLevel: Option<fnParmsNewLevel>,
    pub pfnParmsChangeLevel: Option<fnParmsChangeLevel>,
    pub pfnGetGameDescription: UNFINISHED_FUNCTION_POINTER,
    pub pfnPlayerCustomization: UNFINISHED_FUNCTION_POINTER,
    pub pfnSpectatorConnect: UNFINISHED_FUNCTION_POINTER,
    pub pfnSpectatorDisconnect: UNFINISHED_FUNCTION_POINTER,
    pub pfnSpectatorThink: UNFINISHED_FUNCTION_POINTER,
    pub pfnSys_Error: UNFINISHED_FUNCTION_POINTER,
    pub pfnPM_Move: UNFINISHED_FUNCTION_POINTER,
    pub pfnPM_Init: UNFINISHED_FUNCTION_POINTER,
    pub pfnPM_FindTextureType: UNFINISHED_FUNCTION_POINTER,
    pub pfnSetupVisibility: UNFINISHED_FUNCTION_POINTER,
    pub pfnUpdateClientData: UNFINISHED_FUNCTION_POINTER,
    pub pfnAddToFullPack: UNFINISHED_FUNCTION_POINTER,
    pub pfnCreateBaseline: UNFINISHED_FUNCTION_POINTER,
    pub pfnRegisterEncoders: Option<fnRegisterEncoders>,
    pub pfnGetWeaponData: UNFINISHED_FUNCTION_POINTER,
    pub pfnCmdStart: UNFINISHED_FUNCTION_POINTER,
    pub pfnCmdEnd: UNFINISHED_FUNCTION_POINTER,
    pub pfnConnectionlessPacket: UNFINISHED_FUNCTION_POINTER,
    pub pfnGetHullBounds: UNFINISHED_FUNCTION_POINTER,
    pub pfnCreateInstancedBaselines: Option<fnCreateInstancedBaselines>,
    pub pfnInconsistentFile: UNFINISHED_FUNCTION_POINTER,
    pub pfnAllowLagCompensation: UNFINISHED_FUNCTION_POINTER,
}
