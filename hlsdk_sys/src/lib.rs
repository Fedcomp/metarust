#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int};

/// Bindings are work in progress undone definitions are marked as unfinished (and not working of course)
// TODO: Specify all definitions and remove
type UNFINISHED_FUNCTION = unsafe extern "C" fn();

/// see UNFINISHED_FUNCTION
type UNFINISHED_FUNCTION_POINTER = Option<UNFINISHED_FUNCTION>;

// TODO: Description
pub type fnGameInit = unsafe extern "C" fn();
// TODO: Description
pub type fnStartFrame = unsafe extern "C" fn();
// TODO: Description
pub type fnParmsNewLevel = unsafe extern "C" fn();
// TODO: Description
pub type fnParmsChangeLevel = unsafe extern "C" fn();
// TODO: Description
pub type fnRegisterEncoders = unsafe extern "C" fn();
// TODO: Description
pub type fnCreateInstancedBaselines = unsafe extern "C" fn();

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

/// Precache model for server and clients at given path.
/// TODO: Usage example
pub type fnPrecacheModel = unsafe extern "C" fn(s: *const c_char) -> c_int;
