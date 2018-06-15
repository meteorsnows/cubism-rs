use std::alloc::Layout;

use std::os::raw::c_char;
use std::slice;

use core::{self, csmAlignofMoc, csmAlignofModel, csmMoc, csmModel};

use mem::AlignedMemory;
use CubismError;

type Result<T> = ::std::result::Result<T, CubismError>;

/// Represents the Moc struct. Every Model instance owns an Rc<Moc>.
/// This Moc kind of represents a shared immutable state of the models that are based on it.
#[derive(Debug)]
pub struct Moc {
    mem: AlignedMemory<csmMoc>,
    part_ids: Vec<&'static str>,
    param_ids: Vec<&'static str>,
    param_def_val: &'static [f32],
    param_max_val: &'static [f32],
    param_min_val: &'static [f32],
}

impl Moc {
    /// Returns the part names
    #[inline]
    pub fn part_ids(&self) -> &[&str] {
        &self.part_ids
    }

    /// Returns the parameter names
    #[inline]
    pub fn parameter_ids(&self) -> &[&str] {
        &self.param_ids
    }

    /// Returns the parameter max values
    #[inline]
    pub fn parameter_max(&self) -> &[f32] {
        self.param_max_val
    }

    /// Returns the parameter min values
    #[inline]
    pub fn parameter_min(&self) -> &[f32] {
        self.param_min_val
    }

    /// Returns the parameter default values
    #[inline]
    pub fn parameter_default(&self) -> &[f32] {
        self.param_def_val
    }

    /// todo: Is this really needed? You can get this value by invoking len on any of the slices
    #[inline]
    pub fn parameter_count(&self) -> usize {
        self.param_ids.len()
    }

    /// todo: Is this really needed? You can get this value by invoking len on the part_ids slice
    #[inline]
    pub fn part_count(&self) -> usize {
        self.part_ids.len()
    }
}

impl Moc {
    pub(crate) fn new(data: &[u8]) -> Result<Self> {
        let layout = Layout::from_size_align(data.len(), csmAlignofMoc)?;
        let mut mem = AlignedMemory::from_layout(layout)?;
        unsafe {
            ::std::ptr::copy(data.as_ptr(), mem.as_mut_ptr() as *mut u8, data.len());
            if core::csmReviveMocInPlace(mem.as_mut_ptr() as *mut _, mem.layout().size() as u32).is_null() {
                Err(CubismError::ReviveMocInPlace)
            } else {
                Ok(Moc {
                    mem,
                    part_ids: Vec::new(),
                    param_ids: Vec::new(),
                    param_def_val: slice::from_raw_parts(0x1 as *const f32, 0),
                    param_max_val: slice::from_raw_parts(0x1 as *const f32, 0),
                    param_min_val: slice::from_raw_parts(0x1 as *const f32, 0),
                })
            }
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const csmMoc {
        self.mem.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut csmMoc {
        self.mem.as_mut_ptr()
    }

    ///Called once when the first model of a moc is created to initialize the shared `str` storage
    pub(crate) fn init_ids(&mut self, model: &AlignedMemory<csmModel>) {
        debug_assert!(self.param_ids.is_empty());//Make sure that this hasnt been called before
        unsafe {
            let param_count = core::csmGetParameterCount(model.as_ptr()) as usize;
            let param_ids = core::csmGetParameterIds(model.as_ptr());
            self.param_ids = Self::init_id_vec(param_ids, param_count);
            let part_count = core::csmGetPartCount(model.as_ptr()) as usize;
            let part_ids = core::csmGetPartIds(model.as_ptr());
            self.part_ids = Self::init_id_vec(part_ids, part_count);
            self.param_def_val = slice::from_raw_parts(core::csmGetParameterDefaultValues(model.as_ptr()), param_count) ;
            self.param_max_val = slice::from_raw_parts(core::csmGetParameterMaximumValues(model.as_ptr()), param_count);
            self.param_min_val = slice::from_raw_parts(core::csmGetParameterMinimumValues(model.as_ptr()), param_count);
        }
    }

    fn init_id_vec(ptr: *mut *const c_char, len: usize) -> Vec<&'static str> {
        use std::ffi::CStr;
        let mut out = Vec::with_capacity(len);
        for ptr in unsafe { ::std::slice::from_raw_parts_mut(ptr, len).iter() } {
            unsafe {
                if let Ok(string) = CStr::from_ptr(*ptr).to_str() {
                    out.push(string);
                }
            }
        }
        out
    }

    ///Creates a new model from this moc
    pub(crate) fn init_new_model(&self) -> Result<AlignedMemory<csmModel>> {
        unsafe {
            let model_size = core::csmGetSizeofModel(self.mem.as_ptr());
            let layout = Layout::from_size_align(model_size as usize, csmAlignofModel)?;
            let mut model_mem: AlignedMemory<csmModel> = AlignedMemory::from_layout(layout)?;

            if core::csmInitializeModelInPlace(self.mem.as_ptr(), model_mem.as_mut_ptr() as *mut _, model_size).is_null() {
                Err(CubismError::InitializeModelInPlace)
            } else {
                Ok(model_mem)
            }
        }
    }
}