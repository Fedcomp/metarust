pub use metamod_sys::PLUG_LOADTIME;

mod raw;

pub use raw::*;

// TODO: Document me
#[derive(Debug, Clone, Copy)]
pub struct PluginInfo {
    name: &'static str,
    version: &'static str,
    date: &'static str,
    author: &'static str,
    url: &'static str,
    logtag: &'static str,
    loadable: PLUG_LOADTIME,
    unloadable: PLUG_LOADTIME,
}

impl PluginInfo {
    // TODO: Test me
    pub const fn new(
        logtag: &'static str,
        loadable: PLUG_LOADTIME,
        unloadable: PLUG_LOADTIME,
    ) -> Self {
        // TODO: Inherit values from cargo
        PluginInfo {
            name: "Metarust",
            version: "0.0.1",
            date: "27.04.2019",
            author: "Fedcomp",
            url: "http://amx-x.ru",
            logtag,
            loadable,
            unloadable,
        }
    }
}
