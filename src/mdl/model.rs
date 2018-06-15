use std::rc::Rc;
use std::ops;
use std::slice;
use std::io::Read;

use mem::AlignedMemory;

use core::{self, csmModel};
use super::moc::Moc;
use CubismError;

macro_rules! is_bit_set {
    ($byte:expr, $mask:expr) => { ($byte & $mask) == $mask };
}

//todo Parameter and Part iterator interface

#[derive(Debug)]
pub struct Model {
    mem: AlignedMemory<csmModel>,
    moc: Rc<Moc>,
    param_values: &'static mut [f32],
    part_opacities: &'static mut [f32],
}

//member funcs
impl Model {
    /// Returns the parameter index of `name` or `None` if the parameter name does not exist in this model
    pub fn parameter_index(&self, name: &str) -> Option<usize> {
        self.parameter_ids().iter().position(|id| *id == name)
    }

    /// Returns the part index of `name` or `None` if the part name does not exist in this model
    pub fn part_index(&self, name: &str) -> Option<usize> {
        self.part_ids().iter().position(|id| *id == name)
    }

    #[inline]
    pub fn parameter_values(&self) -> &[f32] {
        self.param_values
    }

    #[inline]
    pub fn parameter_values_mut(&mut self) -> &mut [f32] {
        self.param_values
    }

    #[inline]
    pub fn set_parameter_value(&mut self, idx: usize, val: f32) {
        self.param_values[idx] = val;
    }

    #[inline]
    pub fn part_opacities(&self) -> &[f32] {
        self.part_opacities
    }

    #[inline]
    pub fn part_opacities_mut(&mut self) -> &mut [f32] {
        self.part_opacities
    }

    #[inline]
    pub fn set_part_opacity(&mut self, idx: usize, val: f32) {
        self.part_opacities[idx] = val;
    }

    #[inline]
    pub fn update(&mut self) {
        unsafe { core::csmUpdateModel(self.mem.as_mut_ptr()) };
        unsafe { core::csmResetDrawableDynamicFlags(self.mem.as_mut_ptr()) };
    }
}

//member funcs
impl Model {
    pub fn canvas_dimensions(&self) -> (f32, f32) {
        let mut size = core::csmVector2 { x: 0.0, y: 0.0};
        let mut origin = core::csmVector2 { x: 0.0, y: 0.0};
        let mut ppu = 0.0;
        unsafe {
            core::csmReadCanvasInfo(self.mem.as_ptr(), &mut size, &mut origin, &mut ppu);
        }
        (size.x / ppu, size.y / ppu)
    }

    #[inline]
    pub fn moc(&self) -> Rc<Moc> {
        self.moc.clone()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const csmModel {
        self.mem.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut csmModel {
        self.mem.as_mut_ptr()
    }
}

//drawable funcs
impl Model {
    pub fn drawable_count(&self) -> usize {
        unsafe { core::csmGetDrawableCount(self.as_ptr()) as usize }
    }

    pub fn drawable_render_orders(&self) -> &[i32] {
        unsafe { slice::from_raw_parts(core::csmGetDrawableRenderOrders(self.as_ptr()), self.drawable_count()) }
    }

    pub fn drawable_texture_indices(&self) -> &[i32] {
        unsafe { slice::from_raw_parts(core::csmGetDrawableTextureIndices(self.as_ptr()), self.drawable_count()) }
    }

    pub fn drawable_index_counts(&self) -> &[i32] {
        unsafe { slice::from_raw_parts(core::csmGetDrawableIndexCounts(self.as_ptr()), self.drawable_count()) }
    }

    pub fn drawable_vertex_counts(&self) -> &[i32] {
        unsafe { slice::from_raw_parts(core::csmGetDrawableVertexCounts(self.as_ptr()), self.drawable_count()) }
    }

    pub fn drawable_indices(&self, idx: usize) -> &[u16] {
        debug_assert!(idx < self.drawable_count());
        unsafe {
            slice::from_raw_parts(
                *core::csmGetDrawableIndices(self.as_ptr()).offset(idx as isize),
                self.drawable_index_counts()[idx] as usize
            )
        }
    }

    pub fn drawable_vertex_positions(&self, idx: usize) -> &[(f32, f32)] {
        debug_assert!(idx < self.drawable_count());
        unsafe {
            slice::from_raw_parts(
                *core::csmGetDrawableVertexPositions(self.as_ptr()).offset(idx as isize) as *const _,
                self.drawable_vertex_counts()[idx] as usize
            )
        }
    }

    pub fn drawable_vertex_uvs(&self, idx: usize) -> &[(f32, f32)] {
        debug_assert!(idx < self.drawable_count());
        unsafe {
            slice::from_raw_parts(
                *core::csmGetDrawableVertexUvs(self.as_ptr()).offset(idx as isize) as *const _,
                self.drawable_vertex_counts()[idx] as usize
            )
        }
    }

    pub fn drawable_opacity(&self) -> &[f32] {
        unsafe { slice::from_raw_parts(core::csmGetDrawableOpacities(self.as_ptr()), self.drawable_count()) }
    }

    pub fn drawable_is_double_sided(&self, idx: usize) -> bool {
        debug_assert!(idx < self.drawable_count());
        unsafe {
            let cf = *core::csmGetDrawableConstantFlags(self.as_ptr()).offset(idx as isize);
            is_bit_set!(cf, core::csmIsDoubleSided)
        }
    }
    /*bitflags
    pub fn drawable_culling(&self, idx: usize) -> i32 {
        unsafe { slice::from_raw_parts(core::csmGetDrawableConstantFlags(self.as_ptr()), self.drawable_count())[idx] }
    }

Rendering::CubismRenderer::CubismBlendMode CubismModel::GetDrawableBlendMode(csmInt32 drawableIndex) const
{
    const csmUint8* constantFlags = Core::csmGetDrawableConstantFlags(_model);
    return (IsBitSet(constantFlags[drawableIndex], Core::csmBlendAdditive))
               ? Rendering::CubismRenderer::CubismBlendMode::CubismBlendMode_Additive
               : (IsBitSet(constantFlags[drawableIndex], Core::csmBlendMultiplicative))
               ? Rendering::CubismRenderer::CubismBlendMode::CubismBlendMode_Multiplicative
               : Rendering::CubismRenderer::CubismBlendMode::CubismBlendMode_Normal;
}

    */

    pub fn drawable_mask_counts(&self) -> &[i32] {
        unsafe { slice::from_raw_parts(core::csmGetDrawableMaskCounts(self.as_ptr()), self.drawable_count()) }
    }

    /// Returns the drawable mask for the given index
    pub fn drawable_mask(&self, idx: usize) -> &[i32] {
        unsafe {
            slice::from_raw_parts(
                slice::from_raw_parts(core::csmGetDrawableMasks(self.as_ptr()), self.drawable_count())[idx] as *const _,
                self.drawable_mask_counts()[idx] as usize
            )
        }
    }

    /// Returns true if this model is masked.
    pub fn is_masked(&self) -> bool {
        let maskcounts = self.drawable_mask_counts();
        (0..self.drawable_count()).any(|i| maskcounts[i] <= 0)
    }

    //todo replace slices with asserts and pointer arithmetic
    //etc
}

//Constructors
impl Model {
    /// Creates a model instance from byte data
    pub fn from_bytes(data: &[u8]) -> Result<Self, CubismError> {
        let mut moc = Moc::new(data)?;
        let model_mem = moc.init_new_model()?;
        moc.init_ids(&model_mem);
        Ok(Self::new_impl(Rc::new(moc), model_mem))
    }

    /// Creates a model instance from a reader instance
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, CubismError> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Self::from_bytes(&buf)
    }

    /// Creates a model instance from another models Moc
    pub fn from_moc(moc: Rc<Moc>) -> Result<Self, CubismError> {
        let model_mem = moc.init_new_model()?;
        Ok(Self::new_impl(moc, model_mem))
    }

    ///Creates a copy from this Model, this can fail.
    pub fn try_clone_from(&self) -> Result<Self, CubismError> {
        let moc = self.moc.clone();
        let model_mem = moc.init_new_model()?;
        let model = Self::new_impl(moc, model_mem);
        model.param_values.copy_from_slice(self.param_values);
        model.part_opacities.copy_from_slice(self.part_opacities);
        Ok(model)
    }

    pub(crate) fn new_impl(moc: Rc<Moc>, mut mem: AlignedMemory<core::csmModel>) -> Model {
        unsafe {
            let param_values = slice::from_raw_parts_mut(core::csmGetParameterValues(mem.as_mut_ptr()), moc.parameter_count());
            let part_opacities = slice::from_raw_parts_mut(core::csmGetPartOpacities(mem.as_mut_ptr()), moc.part_count());

            Model {
                mem,
                moc,
                param_values,
                part_opacities,
            }
        }
    }
}

impl ops::Deref for Model {
    type Target = Moc;
    fn deref(&self) -> &Self::Target {
        &self.moc
    }
}
