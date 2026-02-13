use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use crate::traits::{Deallocator, LibcFree};

// Container for C memory
pub struct Blob<T, D: Deallocator = LibcFree> {
    ptr: NonNull<T>,
    len: usize,
    _marker: PhantomData<D>,
}

impl<T, D: Deallocator> Blob<T, D> {
    // ptr must be non-null, aligned for T, and owned by the caller
    pub unsafe fn from_raw(ptr: *mut T, len: usize) -> Self {
        let non_null = NonNull::new(ptr).expect("vessel: Null pointer passed to Blob");
        
        debug_assert!(
            non_null.as_ptr() as usize % std::mem::align_of::<T>() == 0,
            "vessel: Misaligned pointer for type {}", std::any::type_name::<T>()
        );

        Self {
            ptr: non_null,
            len,
            _marker: PhantomData,
        }
    }

    pub fn leak(self) -> *mut T {
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);
        ptr
    }
}

impl<T, D: Deallocator> Deref for Blob<T, D> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        // ptr and len are guaranteed valid for Blob's lifetime
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T, D: Deallocator> DerefMut for Blob<T, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T, D: Deallocator> Drop for Blob<T, D> {
    fn drop(&mut self) {
        unsafe {
            D::deallocate(self.ptr.as_ptr().cast(), self.len * std::mem::size_of::<T>());
        }
    }
}

// Blobs are send/sync if the underlying <T> is
unsafe impl<T: Send, D: Deallocator> Send for Blob<T, D> {}
unsafe impl<T: Sync, D: Deallocator> Sync for Blob<T, D> {}
