#![no_std]

/// Utilities related to booting up the system
pub mod boot;
/// Crate's error enum and Result alias
mod error;
/// Utilities related to (un)mounting filesystems
pub mod mount;
/// Utilities related to reading or setting PID values
pub mod pid;
/// A seed for rand generated at compile-time in build.rs
mod rand_seed;
/// Utilities related to handling signal interrupts
pub mod signal;
/// Utilities related to waiting for processes to exit
pub mod wait;

use boot::boot_up_system;
pub use error::{Error, Result};
use nix::libc::exit;

fn run() -> Result<()> {
    boot_up_system()
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
