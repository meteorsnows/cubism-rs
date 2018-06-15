use libc::c_char;
use libc::c_uchar;
use libc::c_float;
use libc::c_int;
use libc::c_ushort;

use csmVector2;
use model::csmModel;

pub const csmBlendAdditive: u8 = 1 << 0;
pub const csmBlendMultiplicative:u8 = 1 << 1;
pub const csmIsDoubleSided: u8 = 1 << 2;

pub const csmIsVisible: u8 = 1 << 0;
pub const csmVisibilityDidChange: u8 = 1 << 1;
pub const csmOpacityDidChange: u8 = 1 << 2;
pub const csmDrawOrderDidChange: u8 = 1 << 3;
pub const csmRenderOrderDidChange: u8 = 1 << 4;
pub const csmVertexPositionsDidChange: u8 = 1 << 5;

pub type csmFlags = c_uchar;

extern "C" {
    pub fn csmReadCanvasInfo(model: *const csmModel, outSizeInPixels: *mut csmVector2,
                             outOriginalInPixels: *mut csmVector2, outPixelsPerUnit: *mut c_float);
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
