//! A framework for Live2D's cubism sdk
#![deny(missing_docs)]

extern crate cubism_core_sys as core;
extern crate libc;
#[macro_use]
extern crate bitflags;

use std::{error, fmt, io, str};

mod flags;
mod mdl;
mod mem;

pub use flags::*;
pub use mdl::*;

/// Returns the linked library version in a (version, major, minor, patch) tuple
pub fn version() -> (u32, u32, u32, u32) {
    let version = unsafe { core::csmGetVersion() };
    let major = (version & 0xFF00_0000) >> 24;
    let minor = (version & 0x00FF_0000) >> 16;
    let patch = version & 0xFFFF;
    (version, major, minor, patch)
}

pub(crate) type Result<T> = ::std::result::Result<T, CubismError>;

/// The error type returned by all fallable functions
#[derive(Debug)]
pub enum CubismError {
    /// A Parameter or Part had an invalid Id, a malformed utf8 string for example
    InvalidId(str::Utf8Error),
    /// An I/O error occured.
    Io(io::Error),
    /// A different error
    Other(String),
}

impl error::Error for CubismError {
    fn description(&self) -> &str {
        match *self {
            CubismError::InvalidId(ref err) => err.description(),
            CubismError::Io(ref err) => err.description(),
            CubismError::Other(ref s) => s,
        }
    }
}

impl fmt::Display for CubismError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CubismError::InvalidId(ref err) => err.fmt(fmt),
            CubismError::Io(ref err) => err.fmt(fmt),
            CubismError::Other(ref s) => fmt.write_str(s),
        }
    }
}

impl From<str::Utf8Error> for CubismError {
    fn from(e: str::Utf8Error) -> CubismError {
        CubismError::InvalidId(e)
    }
}

impl From<io::Error> for CubismError {
    fn from(e: io::Error) -> CubismError {
        CubismError::Io(e)
    }
}

impl<'a> From<&'a str> for CubismError {
    fn from(e: &'a str) -> CubismError {
        CubismError::Other(e.to_owned())
    }
}
