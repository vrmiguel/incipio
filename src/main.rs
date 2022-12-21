#![no_std]

mod error;
/// Utilities related to (un)mounting filesystems
pub mod mount;
/// Utilities related to reading or setting PID values
pub mod pid;
/// Utilities related to handling signal interrupts
pub mod signal;
/// Utilities related to waiting for processes to exit
pub mod wait;
/// A seed for rand generated at compile-time in build.rs
mod rand_seed;

pub use error::{Error, Result};
use mount::mount_pseudo_filesystems;
use nix::libc::exit;
use pid::ensure_running_as_init_system;
use signal::install_signal_handler;

fn run() -> Result<()> {
    // Make sure we're running with PID 1.
    ensure_running_as_init_system()?;

    // Get our signal handler up and running
    install_signal_handler()?;

    // Mount procfs, sysfs, /run and /dev
    mount_pseudo_filesystems()?;

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(_) => {
            // Safety: there are no more destructors to be run
            // at this point
            unsafe { exit(1) }
        }
    }
}
