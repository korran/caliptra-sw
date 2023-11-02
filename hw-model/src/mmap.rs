use std::{path::Path, os::fd::{OwnedFd, AsFd, AsRawFd}, fs::File, ptr, io};

use libc::c_void;



pub struct Mmap {
    ptr: *mut c_void,
    len: usize,
}

impl Mmap {
    pub fn open_read_write(path: &Path, offset: u64, len: usize) -> std::io::Result<Self> {
        let fd = OwnedFd::from(File::options().read(true).write(true).open(path)?);
        let ptr = unsafe { libc::mmap(ptr::null_mut(), len, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd.as_raw_fd(), offset as i64) };
        if ptr.is_null() {
            return Err(io::Error::last_os_error());
        }
        Ok(Mmap {  
            ptr,
            len
        })
    }

    pub fn ptr(&self) -> *mut c_void {
        self.ptr
    }
    pub fn len(&self) -> usize {
        self.len
    }
}
impl Drop for Mmap {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.ptr, self.len) };
    }
}