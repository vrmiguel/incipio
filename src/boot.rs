use nix::{fcntl::OFlag, sys::stat::Mode, unistd::{write, close}};

use crate::{
    mount::mount_pseudo_filesystems,
    pid::ensure_running_as_init_system,
    signal::install_signal_handler, rand_seed::SEED,
};

/// Seed `/dev/urandom`
fn seed_urandom() -> crate::Result<()> {
    let raw_fd = nix::fcntl::open("/dev/urandom", OFlag::O_WRONLY, Mode::S_IRWXU)?;
    write(raw_fd, SEED)?;
    close(raw_fd)?;

    Ok(())
}

pub fn boot_up_system() -> crate::Result<()> {
    // Make sure we're running with PID 1.
    ensure_running_as_init_system()?;

    // Get our signal handler up and running
    install_signal_handler()?;

    // Mount procfs, sysfs, /run and /dev
    mount_pseudo_filesystems()?;

    Ok(())
}
