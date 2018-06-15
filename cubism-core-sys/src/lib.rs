#![crate_name="cubism_core_sys"]
#![crate_type = "lib"]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate libc;

pub mod draw;
pub mod logging;
pub mod moc;
pub mod model;

pub use draw::*;
pub use logging::*;
pub use moc::*;
pub use model::*;

pub type csmVersion = libc::c_uint;

#[repr(C)]
pub struct csmVector2 {
    pub x: libc::c_float,
    pub y: libc::c_float,
}

extern "C" {
    pub fn csmGetVersion() -> csmVersion;
}
