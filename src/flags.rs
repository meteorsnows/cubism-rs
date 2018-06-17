use libc::c_uchar;

use core::{
    csmBlendAdditive, csmBlendMultiplicative, csmDrawOrderDidChange, csmIsDoubleSided,
    csmIsVisible, csmOpacityDidChange, csmRenderOrderDidChange, csmVertexPositionsDidChange,
    csmVisibilityDidChange,
};

bitflags! {
    /// The constant flags of a [Model](model/struct.Model.html)'s drawable.
    #[repr(C)]
    pub struct ConstantFlags: c_uchar {
        /// The drawable should be blended additively.
        const BLEND_ADDITIVE = csmBlendAdditive;
        /// The drawable should be blended multiplicatively.
        const BLEND_MULTIPLICATIVE = csmBlendMultiplicative;
        /// The drawable is double sided and therefore shouldn't be culled.
        const IS_DOUBLE_SIDED = csmIsDoubleSided;
    }
}

bitflags! {
    /// The dynamic flags of a [Model](model/struct.Model.html)'s drawable.
    #[repr(C)]
    pub struct DynamicFlags: c_uchar {
        /// The drawable is visible.
        const IS_VISIBLE = csmIsVisible;
        /// The drawable's visibility changed since the last update.
        const VISIBILITY_CHANGED = csmVisibilityDidChange;
        /// The drawable's opacity changed since the last update.
        const OPACITY_CHANGED = csmOpacityDidChange;
        /// The drawable's drawing order changed since the last update.
        const DRAW_ORDER_CHANGED = csmDrawOrderDidChange;
        /// The drawable's render order changed since the last update.
        const RENDER_ORDER_CHANGED = csmRenderOrderDidChange;
        /// The drawable's vertex positions changed since the last update.
        const VERTEX_POSITIONS_CHANGED = csmVertexPositionsDidChange;
    }
}
