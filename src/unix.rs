use std::os::fd::{AsRawFd, OwnedFd, FromRawFd};
use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK};

// File descriptor
pub struct Handle(OwnedFd);

impl Handle {
    // Create a new handle from a C integer
    // fd must be a valid AND open
    pub unsafe fn from_raw(fd: i32) -> Self {
        Self(unsafe { OwnedFd::from_raw_fd(fd) })
    }

    pub fn set_nonblocking(&self, enabled: bool) -> std::io::Result<()> {
        let fd = self.0.as_raw_fd();
        unsafe {
            let flags = fcntl(fd, F_GETFL);
            if flags == -1 { return Err(std::io::Error::last_os_error()); }
            
            let new_flags = if enabled { flags | O_NONBLOCK } else { flags & !O_NONBLOCK };
            if fcntl(fd, F_SETFL, new_flags) == -1 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(())
    }
}

impl AsRawFd for Handle {
    fn as_raw_fd(&self) -> i32 {
        self.0.as_raw_fd()
    }
}
