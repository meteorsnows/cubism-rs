#![feature(allocator_api)]

extern crate libc;
extern crate cubism_core_sys as core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::{error, fmt, alloc, io};

mod mem;
mod mdl;

pub use mdl::*;

/// Returns the linked library version in a (version, major, minor, patch) tuple
pub fn version() -> (u32, u32, u32, u32) {
    let version = unsafe { core::csmGetVersion() };
    let major = (version & 0xFF00_0000) >> 24;
    let minor = (version & 0x00FF_0000) >> 16;
    let patch = version & 0xFFFF;
    (version, major, minor, patch)
}


#[derive(Debug)]
pub enum CubismError {
    ReviveMocInPlace,
    InitializeModelInPlace,
    Layout(alloc::LayoutErr),
    Alloc(alloc::AllocErr),
    Io(io::Error),
}

impl error::Error for CubismError {
    fn description(&self) -> &str {
        match *self {
            CubismError::ReviveMocInPlace => "failed to revive moc in aligned memory",
            CubismError::InitializeModelInPlace =>  "failed to revive moc in aligned memory",
            CubismError::Layout(ref err) => err.description(),
            CubismError::Alloc(ref err) => err.description(),
            CubismError::Io(ref err) => err.description(),
        }
    }
}

impl fmt::Display for CubismError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CubismError::ReviveMocInPlace => write!(fmt, "failed to revive moc in aligned memory"),
            CubismError::InitializeModelInPlace => write!(fmt, "failed to revive moc in aligned memory"),
            CubismError::Layout(ref err) => err.fmt(fmt),
            CubismError::Alloc(ref err) => err.fmt(fmt),
            CubismError::Io(ref err) => err.fmt(fmt),
        }
    }
}

impl From<alloc::LayoutErr> for CubismError {
    fn from(e: alloc::LayoutErr) -> CubismError {
        CubismError::Layout(e)
    }
}

impl From<alloc::AllocErr> for CubismError {
    fn from(e: alloc::AllocErr) -> CubismError {
        CubismError::Alloc(e)
    }
}

impl From<io::Error> for CubismError {
    fn from(e: io::Error) -> CubismError {
        CubismError::Io(e)
    }
}
