use std::os::raw::c_char;

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [::std::os::raw::c_char]
            as *const ::std::os::raw::c_char
    };
}

/// Metamod interface version.
/// Declaration copy of META_INTERFACE_VERSION.
/// Description copied from metamod-p sources:
/// ```
/// Version consists of "major:minor", two separate integer numbers.
/// Version 1	 original
/// Version 2	 added plugin_info_t **pinfo
/// Version 3	 init split into query and attach, added detach
/// Version 4	 added (PLUG_LOADTIME now) to attach
/// Version 5	 changed mm ifvers int* to string, moved pl ifvers to info
/// Version 5:1	 added link support for entity "adminmod_timer" (adminmod)
/// Version 5:2	 added gamedll_funcs to meta_attach() [v1.0-rc2]
/// Version 5:3	 added mutil_funcs to meta_query() [v1.05]
/// Version 5:4	 added centersay utility functions [v1.06]
/// Version 5:5	 added Meta_Init to plugin API [v1.08]
/// Version 5:6	 added CallGameEntity utility function [v1.09]
/// Version 5:7	 added GetUserMsgID, GetUserMsgName util funcs [v1.11]
/// Version 5:8	 added GetPluginPath [v1.11]
/// Version 5:9	 added GetGameInfo [v1.14]
/// Version 5:10 added GINFO_REALDLL_FULLPATH for GetGameInfo [v1.17]
/// Version 5:11 added plugin loading and unloading API [v1.18]
/// Version 5:12 added IS_QUERYING_CLIENT_CVAR to mutils [v1.18]
/// Version 5:13 added MAKE_REQUESTID and GET_HOOK_TABLES to mutils [v1.19]
/// ```
pub const METAMOD_INTERFACE_VERSION: *const c_char = cstr!("5:13");

/// Specifies when metamod plugin should be started
/// This enum is a representation of PLUG_LOADTIME from metamod/plinfo.h
#[repr(C)]
pub enum PluginLoadTime {
    Never = 0,
    /// should only be loaded/unloaded at initial hlds execution
    Startup,
    /// can be loaded/unloaded between maps
    ChangeLevel,
    /// can be loaded/unloaded at any time
    AnyTime,
    /// can be loaded/unloaded at any time, and can be "paused" during a map
    AnyPause,
}

/// Metamod plugin info structure
/// This struct is a representation of metamod/plinfo.h
#[repr(C)]
pub struct PluginInfo {
    /// metamod binary interface version.
    /// MetaRust only supports [METAMOD_INTERFACE_VERSION]
    pub interface_version: *const c_char,
    /// full name of plugin
    pub name: *const c_char,
    /// version in free format
    pub version: *const c_char,
    /// compilation date in free format
    pub date: *const c_char,
    /// author name/email
    pub author: *const c_char,
    /// plugin URL
    pub url: *const c_char,
    /// log message prefix (unused right now)
    pub logtag: *const c_char,
    /// when plugin is allowed to be loaded and attached
    pub loadable: PluginLoadTime,
    /// when plugin is allowed to be unloaded and detached
    pub unloadable: PluginLoadTime,
}
