use metarust::PLUG_LOADTIME;
pub use metarust::*;

/// Most values are inherited from cargo
const PLUGIN_INFO: PluginInfo = PluginInfo::new(
    "METARUST",
    PLUG_LOADTIME::PT_CHANGELEVEL,
    PLUG_LOADTIME::PT_CHANGELEVEL,
);
