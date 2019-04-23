#![deny(warnings)]
#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int};

/// Precache model for server and clients at given path.
/// TODO: Usage example
pub type fnPrecacheModel = unsafe extern "C" fn(s: *const c_char) -> c_int;
