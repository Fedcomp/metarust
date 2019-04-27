use std::convert::TryFrom;
use std::ffi::{self, CStr, CString};

use cstr_macro::cstr;
use metamod_sys::{plugin_info_t, META_INTERFACE_VERSION};

pub use metamod_sys::PLUG_LOADTIME;

// TODO: Document me
#[derive(Debug)]
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
    pub const fn new(
        name: &'static str,
        version: &'static str,
        date: &'static str,
        author: &'static str,
        url: &'static str,
        logtag: &'static str,
        loadable: PLUG_LOADTIME,
        unloadable: PLUG_LOADTIME,
    ) -> Self {
        PluginInfo {
            name,
            version,
            date,
            author,
            url,
            logtag,
            loadable,
            unloadable,
        }
    }
}

pub(crate) struct PluginInfoOwned {
    name: CString,
    version: CString,
    date: CString,
    author: CString,
    url: CString,
    logtag: CString,
    loadable: PLUG_LOADTIME,
    unloadable: PLUG_LOADTIME,
}

impl PluginInfoOwned {
    pub fn as_plugin_info_t(&self) -> plugin_info_t {
        plugin_info_t {
            ifvers: META_INTERFACE_VERSION,
            name: self.name.as_ptr(),
            version: self.name.as_ptr(),
            date: self.date.as_ptr(),
            author: self.author.as_ptr(),
            url: self.url.as_ptr(),
            logtag: self.logtag.as_ptr(),
            loadable: self.loadable,
            unloadable: self.unloadable,
        }
    }
}

impl TryFrom<&PluginInfo> for PluginInfoOwned {
    type Error = ffi::NulError;

    fn try_from(plugin_info: &PluginInfo) -> Result<PluginInfoOwned, Self::Error> {
        Ok(PluginInfoOwned {
            name: CString::new(plugin_info.name)?,
            version: CString::new(plugin_info.version)?,
            date: CString::new(plugin_info.date)?,
            author: CString::new(plugin_info.author)?,
            url: CString::new(plugin_info.url)?,
            logtag: CString::new(plugin_info.logtag)?,
            loadable: plugin_info.loadable,
            unloadable: plugin_info.unloadable,
        })
    }
}
