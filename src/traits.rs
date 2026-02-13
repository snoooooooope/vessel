pub trait Deallocator: Send + Sync + 'static {
    // ptr must be a valid pointer from the allocator
    unsafe fn deallocate(ptr: *mut u8, len: usize);
}

pub struct LibcFree;
impl Deallocator for LibcFree {
    unsafe fn deallocate(ptr: *mut u8, _len: usize) { unsafe {
        if !ptr.is_null() {
            libc::free(ptr.cast());
        }
    }}
}
