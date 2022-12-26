use core::{ffi::CStr, ptr::NonNull};

use cstr::cstr;
use nix::{
    libc::{endmntent, getmntent, setmntent, FILE},
    NixPath,
};

use crate::Error;

const READ_FLAG: &CStr = cstr!("r");

pub struct MountPointParser {
    stream: NonNull<FILE>,
}

impl MountPointParser {
    pub fn new<P: NixPath + ?Sized>(path: &P) -> crate::Result<Self> {
        let stream = path.with_nix_path(|path| unsafe {
            setmntent(path.as_ptr(), READ_FLAG.as_ptr())
        })?;
        let stream =
            NonNull::new(stream).ok_or(Error::MountPointParser)?;

        Ok(Self { stream })
    }
}

impl Drop for MountPointParser {
    fn drop(&mut self) {
        // endmntent always returns 1
        unsafe { endmntent(self.stream.as_ptr()) };
    }
}

pub struct Entry {
    entry_ptr: NonNull<nix::libc::mntent>,
}

impl Entry {
    /// The name of the path this entry represents
    pub fn path(&self) -> &CStr {
        // Safety: this (non-null) pointer is provenient of a
        // succesfull call to `setmntent`, which manages the
        // entire lifetime of this pointer until the stream is
        // closed with `endmntent`.
        unsafe {
            let mnt_dir_ptr = self.entry_ptr.as_ref().mnt_dir;
            CStr::from_ptr(mnt_dir_ptr)
        }
    }
}

impl Entry {
    pub fn from_raw(ptr: *mut nix::libc::mntent) -> Option<Self> {
        NonNull::new(ptr).map(|entry_ptr| Self { entry_ptr })
    }
}

impl Iterator for MountPointParser {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry_ptr = unsafe { getmntent(self.stream.as_ptr()) };

        Entry::from_raw(entry_ptr)
    }
}
