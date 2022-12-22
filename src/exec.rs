use core::{
    ffi::{c_char, CStr},
    fmt::Debug,
    ops::Not,
};

use libc_print::libc_eprintln;
use nix::{
    errno::Errno,
    unistd::{fork, ForkResult},
    NixPath,
};

use crate::{utils::NixPathExt, wait::wait_pid_no_interrupt};

/// Sad work-around since a generic `execute` function would likely
/// have to allocate on the heap.
///
/// Fork and execute through `execv` with an already built command
/// array.
pub fn fork_and_execute_command<const N: usize>(
    commands: [*const c_char; N],
    should_wait: bool,
) -> crate::Result<()> {
    // TODO: Check if we're able to run vfork here
    let fork_result = unsafe { fork()? };

    match fork_result {
        ForkResult::Child => execv_commands(commands)?,
        ForkResult::Parent { child } => {
            if should_wait {
                wait_pid_no_interrupt(child, None)?;
            }
        }
    }

    Ok(())
}

/// Run `execv` with an already built `commands` sequence.
fn execv_commands<const N: usize>(
    commands: [*const c_char; N],
) -> crate::Result<()> {
    let ret_val =
        unsafe { nix::libc::execv(commands[0], commands.as_ptr()) };

    Errno::result(ret_val).map_err(Into::into).map(|_result| ())
}

// Run execv without additional arguments
fn execv(path: &CStr) -> crate::Result<()> {
    let command = [path.as_ptr(), core::ptr::null()];

    execv_commands(command)
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
