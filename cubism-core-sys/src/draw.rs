use libc::{c_char, c_float, c_int, c_uchar, c_ushort};

use csmVector2;
use model::csmModel;

pub const csmBlendAdditive: csmFlags = 1 << 0;
pub const csmBlendMultiplicative: csmFlags = 1 << 1;
pub const csmIsDoubleSided: csmFlags = 1 << 2;

pub const csmIsVisible: csmFlags = 1 << 0;
pub const csmVisibilityDidChange: csmFlags = 1 << 1;
pub const csmOpacityDidChange: csmFlags = 1 << 2;
pub const csmDrawOrderDidChange: csmFlags = 1 << 3;
pub const csmRenderOrderDidChange: csmFlags = 1 << 4;
pub const csmVertexPositionsDidChange: csmFlags = 1 << 5;

pub type csmFlags = c_uchar;

extern "C" {
    pub fn csmReadCanvasInfo(
        model: *const csmModel,
        outSizeInPixels: *mut csmVector2,
        outOriginalInPixels: *mut csmVector2,
        outPixelsPerUnit: *mut c_float,
    );
}

extern "C" {
    pub fn csmGetDrawableCount(model: *const csmModel) -> c_int;
    pub fn csmGetDrawableIds(model: *const csmModel) -> *mut *const c_char;
    pub fn csmGetDrawableConstantFlags(model: *const csmModel) -> *const csmFlags;
    pub fn csmGetDrawableDynamicFlags(model: *const csmModel) -> *const csmFlags;
    pub fn csmGetDrawableTextureIndices(model: *const csmModel) -> *const c_int;
    pub fn csmGetDrawableDrawOrders(model: *const csmModel) -> *const c_int;
    pub fn csmGetDrawableRenderOrders(model: *const csmModel) -> *const c_int;
    pub fn csmGetDrawableOpacities(model: *const csmModel) -> *const c_float;
    pub fn csmGetDrawableMaskCounts(model: *const csmModel) -> *const c_int;
    pub fn csmGetDrawableMasks(model: *const csmModel) -> *mut *const c_int;
    pub fn csmGetDrawableVertexCounts(model: *const csmModel) -> *const c_int;
    pub fn csmGetDrawableVertexPositions(model: *const csmModel) -> *mut *const csmVector2;
    pub fn csmGetDrawableVertexUvs(model: *const csmModel) -> *mut *const csmVector2;
    pub fn csmGetDrawableIndexCounts(model: *const csmModel) -> *const c_int;
    pub fn csmGetDrawableIndices(model: *const csmModel) -> *mut *const c_ushort;
    pub fn csmResetDrawableDynamicFlags(model: *mut csmModel);
}
