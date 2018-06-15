use libc::c_uint;
use libc::c_void;

pub const csmAlignofMoc: usize = 64;

#[repr(C)]
pub struct csmMoc {
    _unused: [u64; 0]
}

extern "C" {
    pub fn csmReviveMocInPlace(aligned_address: *mut c_void, size: c_uint) -> *mut csmMoc;
}
