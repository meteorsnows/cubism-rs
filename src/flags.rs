use libc::c_uchar;

use core::{csmBlendAdditive, csmBlendMultiplicative, csmIsDoubleSided, csmIsVisible,
           csmVisibilityDidChange, csmOpacityDidChange, csmDrawOrderDidChange,
           csmRenderOrderDidChange, csmVertexPositionsDidChange};

bitflags! {
    #[repr(C)]
    pub struct ConstantFlags: c_uchar {
        const BLEND_ADDITIVE = csmBlendAdditive;
        const BLEND_MULTIPLICATIVE = csmBlendMultiplicative;
        const IS_DOUBLE_SIDED = csmIsDoubleSided;
    }
}

bitflags! {
    #[repr(C)]
    pub struct DynamicFlags: c_uchar {
        const IS_VISIBLE = csmIsVisible;
        const VISIBILITY_CHANGED = csmVisibilityDidChange;
        const OPACITY_CHANGED = csmOpacityDidChange;
        const DRAW_ORDER_CHANGED = csmDrawOrderDidChange;
        const RENDER_ORDER_CHANGED = csmRenderOrderDidChange;
        const VERTEX_POSITIONS_CHANGED = csmVertexPositionsDidChange;
    }
}
