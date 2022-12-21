use core::{ffi::CStr, fmt::Debug, ops::Not};

use libc_print::libc_eprintln;
use nix::{
    errno::Errno,
    unistd::{fork, ForkResult},
    NixPath,
};

use crate::{utils::NixPathExt, wait::wait_pid_no_interrupt};

fn execv(path: &CStr) -> crate::Result<()> {
    let command = [path.as_ptr(), core::ptr::null()];

    let ret_val =
        unsafe { nix::libc::execv(path.as_ptr(), command.as_ptr()) };

    Errno::result(ret_val).map_err(Into::into).map(|_result| ())
}

pub fn execute<P: NixPath + Debug + ?Sized>(
    path: &P,
) -> crate::Result<()> {
    if path.is_executable().not() {
        libc_eprintln!("Tried to run {:?} but it doesn't exist or is not executable", path);
    }

    let execute_path = |path: &CStr| {
        let fork_result = unsafe { fork()? };

        match fork_result {
            ForkResult::Child => execv(path)?,
            ForkResult::Parent { child } => {
                wait_pid_no_interrupt(child, None)?;
            }
        }

        Ok(()) as crate::Result<()>
    };

    path.with_nix_path(execute_path)?
}
