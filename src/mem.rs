use std::alloc::{Alloc, AllocErr, Global, Layout};
use std::fmt;
use std::mem;
use std::ptr::NonNull;

/// A block of memory that has a specific alignment
pub struct AlignedMemory<T> {
    ptr: NonNull<T>,
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
    pub fn new(size: usize) -> Result<Self, AllocErr> {
        let layout = Layout::from_size_align(size, mem::align_of::<T>()).unwrap();
        let ptr = unsafe { Global.alloc(layout)? }.cast();
        Ok(AlignedMemory { ptr, layout })
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T> Drop for AlignedMemory<T> {
    fn drop(&mut self) {
        unsafe { Global.dealloc(self.ptr.cast(), self.layout) };
    }
}
