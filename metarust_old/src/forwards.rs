use std::os::raw::c_int;

use hlsdk_sys::{edict_t, DLL_FUNCTIONS};

use crate::init::{gpMetaGlobals, PLUGIN_FORWARDS};
use crate::metamod::CallResult;

#[derive(Debug, Copy, Clone)]
pub struct Forwards {
    pub plugin_init: Option<fn() -> CallResult>,
}

pub fn map_requested_forwards(
    metarust_forwards: &Forwards,
    _g_function_table: &mut DLL_FUNCTIONS,
    g_function_table_post: &mut DLL_FUNCTIONS,
) {
    if metarust_forwards.plugin_init.is_some() {
        g_function_table_post.pfnServerActivate = Some(raw_plugin_init);
    }
}

extern "C" fn raw_plugin_init(
    _p_edict_list: *const edict_t,
    _edict_count: c_int,
    _client_max: c_int,
) {
    // TODO: Proper expect?
    let meta_result = unsafe { PLUGIN_FORWARDS.unwrap() }.plugin_init.unwrap()();
    unsafe { gpMetaGlobals.as_mut().unwrap() }.mres = meta_result.into();
}
