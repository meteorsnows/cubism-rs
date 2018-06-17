use std::alloc::{self, Layout};
use std::fmt;
use std::mem;

use Result;

/// A block of memory that has a specific alignment
pub struct AlignedMemory<T> {
    ptr: *mut T,
    layout: Layout,
}

impl<T> fmt::Debug for AlignedMemory<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AlignedMemory")
            .field("ptr", &self.ptr)
            .field("layout", &self.layout)
            .finish()
    }
}

impl<T> AlignedMemory<T> {
    pub fn new(size: usize) -> Result<Self> {
        let layout = Layout::from_size_align(size, mem::align_of::<T>()).unwrap();
        let ptr = unsafe { alloc::alloc(layout) as *mut T };
        if ptr.is_null() {
            Err("Allocation error".into())
        } else {
            Ok(AlignedMemory { ptr, layout })
        }
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }
}

impl<T> Drop for AlignedMemory<T> {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.ptr as *mut u8, self.layout) };
    }
}
