#![no_std]

/// Utilities related to (un)mounting filesystems
pub mod mount;
/// Utilities related to reading or setting PID values
pub mod pid;
/// Utilities related to handling signal interrupts
pub mod signal;
/// Utilities related to waiting for processes to exit
pub mod wait;

use core::ops::Not;

use nix::libc::exit;
use pid::is_running_as_init_system;

fn run() -> i32 {
    if is_running_as_init_system().not() {
        return 1;
    }

    0
}

fn main() {
    let status_code = run();

    // Safety: there are no more destructors to be run
    // at this point
    unsafe { exit(status_code) }
}
