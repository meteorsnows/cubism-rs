use libc::{c_char, c_float, c_int, c_uint, c_void};

use moc::csmMoc;

pub const csmAlignofModel: usize = 16;

#[repr(C, align(16))]
pub struct csmModel {
    _unused: [u16; 0],
}

extern "C" {
    pub fn csmGetSizeofModel(moc: *const csmMoc) -> c_uint;
    pub fn csmInitializeModelInPlace(
        moc: *const csmMoc,
        aligned_address: *mut c_void,
        size: c_uint,
    ) -> *mut csmModel;
    pub fn csmUpdateModel(model: *mut csmModel);

    pub fn csmGetParameterCount(model: *const csmModel) -> c_int;
    pub fn csmGetParameterIds(model: *const csmModel) -> *mut *const c_char;
    pub fn csmGetParameterMinimumValues(model: *const csmModel) -> *const c_float;
    pub fn csmGetParameterMaximumValues(model: *const csmModel) -> *const c_float;
    pub fn csmGetParameterDefaultValues(model: *const csmModel) -> *const c_float;
    pub fn csmGetParameterValues(model: *mut csmModel) -> *mut c_float;

    pub fn csmGetPartCount(model: *const csmModel) -> c_int;
    pub fn csmGetPartIds(model: *const csmModel) -> *mut *const c_char;
    pub fn csmGetPartOpacities(model: *mut csmModel) -> *mut c_float;
}

#[test]
fn model_alignment() {
    assert_eq!(::std::mem::align_of::<csmModel>(), csmAlignofModel);
}
