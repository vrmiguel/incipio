#![no_std]

/// Utilities related to booting up the system
pub mod boot;
/// Crate's error enum and Result alias
mod error;
/// Utilities related to executing files
pub mod exec;
/// Utilities related to files and filesystems
pub mod fs;
pub mod macros;
/// Utilities related to (un)mounting filesystems
pub mod mount;
/// Utilities related to reading or setting PID values
pub mod pid;
/// A seed for rand generated at compile-time in build.rs
mod rand_seed;
/// Utilities related to handling signal interrupts
pub mod signal;
/// General utilities
pub mod utils;
/// Utilities related to waiting for processes to exit
pub mod wait;

use boot::boot_up_system;
pub use error::{Error, Result};
pub use libc_print::libc_eprintln as eprintln;
use mount::mount_filesystem;
use nix::libc::{exit, EXIT_FAILURE};
use pid::ensure_running_as_init_system;
use signal::install_signal_handler;

fn run() -> Result<()> {
    // Make sure we're running with PID 1.
    ensure_running_as_init_system()?;

    // Get our signal handler up and running
    install_signal_handler()?;

    // Mount procfs, sysfs, /run and /dev, /dev/pts, /dev/shm
    mount_filesystem()?;

    // Set hostname, seed /dev/urandom and disable Ctrl+Alt+Del
    boot_up_system()?;

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {
            // Should not be reached, kernel will panic
        }
        Err(error) => {
            eprintln!("Error: {}", error.description());
        }
    }

    // Safety: there are no more destructors to be run
    // at this point
    unsafe { exit(EXIT_FAILURE) }
}
