use std::ptr::NonNull;
use std::ops::Deref;
use std::os::fd::AsRawFd;
use libc::{mmap, munmap, PROT_READ, MAP_SHARED, MAP_FAILED};
use crate::unix::Handle;

// Zero copy view of a file or kernel buffer
pub struct View {
    ptr: NonNull<libc::c_void>,
    len: usize,
}

impl View {
    pub fn new(handle: &Handle, len: usize) -> std::io::Result<Self> {
        unsafe {
            let ptr = mmap(
                std::ptr::null_mut(),
                len,
                PROT_READ,
                MAP_SHARED,
                handle.as_raw_fd(),
                0
            );

            if ptr == MAP_FAILED {
                return Err(std::io::Error::last_os_error());
            }

            Ok(Self {
                ptr: NonNull::new_unchecked(ptr),
                len,
            })
        }
    }

    // Volatile read for hardware buffers
    pub unsafe fn read_volatile<T>(&self, offset: usize) -> T { unsafe {
        // Offset must be within len and aligned for <T>
        let src = (self.ptr.as_ptr() as *const u8).add(offset).cast::<T>();
        std::ptr::read_volatile(src)
    }}
}

impl Deref for View {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr().cast(), self.len) }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe { munmap(self.ptr.as_ptr(), self.len) };
    }
}
