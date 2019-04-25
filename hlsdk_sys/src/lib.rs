#![deny(warnings)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_int, c_short, c_uchar, c_uint, c_void};

/// Bindings are work in progress and undone definitions are marked as unfinished (and not working of course)
// TODO: Specify all definitions and remove
type UNFINISHED_FUNCTION = unsafe extern "C" fn();

/// see UNFINISHED_FUNCTION
type UNFINISHED_FUNCTION_POINTER = Option<UNFINISHED_FUNCTION>;

pub type qboolean = c_int;
pub type string_t = c_uint;
pub type byte = c_uchar;

pub type BOOL = c_int;
pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

pub type edict_t = edict_s;
pub type link_t = link_s;
pub type entvars_t = entvars_s;

const MAX_ENT_LEAFS: usize = 48;

#[repr(C)]
pub struct link_s {
    pub prev: *mut link_s,
    pub next: *mut link_s,
}

/// Game Entity data
#[repr(C)]
pub struct edict_s {
    pub free: qboolean,
    pub serialnumber: c_int,
    pub area: link_t,
    pub headnode: c_int,
    pub num_leafs: c_int,
    pub leafnums: [c_short; MAX_ENT_LEAFS],
    pub freetime: f32,
    pub pvPrivateData: *const c_void,
    pub v: entvars_t,
}

pub type vec_t = f32;

#[repr(C)]
pub struct Vector {
    pub x: vec_t,
    pub y: vec_t,
    pub z: vec_t,
}

#[repr(C)]
pub struct entvars_s {
    pub classname: string_t,
    pub globalname: string_t,
    pub origin: Vector,
    pub oldorigin: Vector,
    pub velocity: Vector,
    pub basevelocity: Vector,
    pub clbasevelocity: Vector,
    pub movedir: Vector,
    pub angles: Vector,
    pub avelocity: Vector,
    pub punchangle: Vector,
    pub v_angle: Vector,
    pub endpos: Vector,
    pub startpos: Vector,
    pub impacttime: f32,
    pub starttime: f32,
    pub fixangle: c_int,
    pub idealpitch: f32,
    pub pitch_speed: f32,
    pub ideal_yaw: f32,
    pub yaw_speed: f32,
    pub modelindex: c_int,
    pub model: string_t,
    pub viewmodel: c_int,
    pub weaponmodel: c_int,
    pub absmin: Vector,
    pub absmax: Vector,
    pub mins: Vector,
    pub maxs: Vector,
    pub size: Vector,
    pub ltime: f32,
    pub nextthink: f32,
    pub movetype: c_int,
    pub solid: c_int,
    pub skin: c_int,
    pub body: c_int,
    pub effects: c_int,
    pub gravity: f32,
    pub friction: f32,
    pub light_level: c_int,
    pub sequence: c_int,
    pub gaitsequence: c_int,
    pub frame: f32,
    pub animtime: f32,
    pub framerate: f32,
    pub controller: [byte; 4usize],
    pub blending: [byte; 2usize],
    pub scale: f32,
    pub rendermode: c_int,
    pub renderamt: f32,
    pub rendercolor: Vector,
    pub renderfx: c_int,
    pub health: f32,
    pub frags: f32,
    pub weapons: c_int,
    pub takedamage: f32,
    pub deadflag: c_int,
    pub view_ofs: Vector,
    pub button: c_int,
    pub impulse: c_int,
    pub chain: *mut edict_t,
    pub dmg_inflictor: *mut edict_t,
    pub enemy: *mut edict_t,
    pub aiment: *mut edict_t,
    pub owner: *mut edict_t,
    pub groundentity: *mut edict_t,
    pub spawnflags: c_int,
    pub flags: c_int,
    pub colormap: c_int,
    pub team: c_int,
    pub max_health: f32,
    pub teleport_time: f32,
    pub armortype: f32,
    pub armorvalue: f32,
    pub waterlevel: c_int,
    pub watertype: c_int,
    pub target: string_t,
    pub targetname: string_t,
    pub netname: string_t,
    pub message: string_t,
    pub dmg_take: f32,
    pub dmg_save: f32,
    pub dmg: f32,
    pub dmgtime: f32,
    pub noise: string_t,
    pub noise1: string_t,
    pub noise2: string_t,
    pub noise3: string_t,
    pub speed: f32,
    pub air_finished: f32,
    pub pain_finished: f32,
    pub radsuit_finished: f32,
    pub pContainingEntity: *mut edict_t,
    pub playerclass: c_int,
    pub maxspeed: f32,
    pub fov: f32,
    pub weaponanim: c_int,
    pub pushmsec: c_int,
    pub bInDuck: c_int,
    pub flTimeStepSound: c_int,
    pub flSwimTime: c_int,
    pub flDuckTime: c_int,
    pub iStepLeft: c_int,
    pub flFallVelocity: f32,
    pub gamestate: c_int,
    pub oldbuttons: c_int,
    pub groupinfo: c_int,
    pub iuser1: c_int,
    pub iuser2: c_int,
    pub iuser3: c_int,
    pub iuser4: c_int,
    pub fuser1: f32,
    pub fuser2: f32,
    pub fuser3: f32,
    pub fuser4: f32,
    pub vuser1: Vector,
    pub vuser2: Vector,
    pub vuser3: Vector,
    pub vuser4: Vector,
    pub euser1: *mut edict_t,
    pub euser2: *mut edict_t,
    pub euser3: *mut edict_t,
    pub euser4: *mut edict_t,
}

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
// TODO: Description
pub type fnServerActivate =
    unsafe extern "C" fn(pEdictList: *const edict_t, edictCount: i32, clientMax: i32);

/// List of engine callbacks
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
    pub pfnServerActivate: Option<fnServerActivate>,
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
