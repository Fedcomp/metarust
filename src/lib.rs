use metarust::{register_plugin, PluginInfo, PLUG_LOADTIME};

const PLUGIN_INFO: PluginInfo = PluginInfo::new(
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_DATE"),
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_HOMEPAGE"),
    "METARUST",
    PLUG_LOADTIME::PT_CHANGELEVEL,
    PLUG_LOADTIME::PT_CHANGELEVEL,
);

register_plugin!(PLUGIN_INFO);
