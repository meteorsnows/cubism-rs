use libc::{c_uint, c_void};

pub const csmAlignofMoc: usize = 64;

#[repr(C, align(64))]
pub struct csmMoc {
    _unused: [u64; 0],
}

extern "C" {
    pub fn csmReviveMocInPlace(aligned_address: *mut c_void, size: c_uint) -> *mut csmMoc;
}

#[test]
fn alignment() {
    assert_eq!(::std::mem::align_of::<csmMoc>(), csmAlignofMoc);
}
