use core::{
    ffi::{c_int, c_void},
    num::NonZeroUsize,
    ops::Not,
    ptr::NonNull,
};

use libc_print::libc_eprintln;
use nix::{
    fcntl::{open, OFlag},
    sys::{
        mman::{mmap, munmap, MapFlags, ProtFlags},
        stat::{fstat, Mode},
    },
    unistd::{access, close, AccessFlags},
    NixPath,
};

use crate::Error;

/// Represents a file opened through a memory mapping.
///
/// The mapping is undone and the file is closed during Drop.
pub struct FileMapping {
    raw_fd: c_int,
    mapped_addr: NonNull<c_void>,
    length: NonZeroUsize,
}

impl FileMapping {
    /// Open a file through a memory mapping
    pub fn open<P: ?Sized + NixPath>(
        path: &P,
    ) -> crate::Result<Self> {
        let raw_fd = open(path, OFlag::O_RDONLY, Mode::S_IRUSR)?;
        let file_length = {
            let status = fstat(raw_fd)?;

            NonZeroUsize::new(status.st_size as usize)
                .ok_or(Error::UnexpectedEmptyFile)?
        };

        let mapped_addr = unsafe {
            mmap(
                // addr is NULL meaning the kernel chooses the
                // (page-aligned) address at which to
                // create the mapping
                None,
                // Length of mapping
                file_length,
                // Only read permission
                ProtFlags::PROT_READ,
                // Changes are not shared
                MapFlags::MAP_PRIVATE,
                // The file descriptor of the file being mapped
                raw_fd,
                // No offset
                0,
            )
        }?;

        let mapped_addr =
            NonNull::new(mapped_addr).ok_or(Error::MmapFailed)?;

        Ok(Self {
            mapped_addr,
            raw_fd,
            length: file_length,
        })
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.mapped_addr.as_ptr() as *const u8,
                self.length.get(),
            )
        }
    }

    /// Close the mapping and close the corresponding file descriptor.
    pub fn close(&mut self) -> crate::Result<()> {
        // Close the file
        close(self.raw_fd)?;

        // Unmap the mapping
        unsafe {
            munmap(self.mapped_addr.as_ptr(), self.length.get() as _)?
        };

        // Set the raw descriptor to an invalid state to signify that
        // this mapping has been closed.
        self.raw_fd = -1;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.raw_fd == -1
    }
}

impl Drop for FileMapping {
    fn drop(&mut self) {
        if self.is_closed().not() {
            // Close the mapping without proper error checking if
            // the user hasn't closed it.
            //
            // I guess this can be solved in a better way once linear
            // types get added to Rust
            if let Err(err) = self.close() {
                libc_eprintln!(
                    "Failed to close memory mapping: {}",
                    err.description()
                )
            }
        }
    }
}

pub trait NixPathExt: NixPath {
    fn is_executable(&self) -> bool {
        access(self, AccessFlags::X_OK).is_ok()
    }
}
impl<P: NixPath + ?Sized> NixPathExt for P {}
