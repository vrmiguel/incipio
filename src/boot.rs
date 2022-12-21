use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{close, write},
};

use crate::{
    mount::mount_pseudo_filesystems,
    pid::ensure_running_as_init_system, rand_seed::SEED,
    signal::install_signal_handler, utils::FileMapping,
};

/// Seed `/dev/urandom`
fn seed_urandom() -> crate::Result<()> {
    let raw_fd = nix::fcntl::open(
        "/dev/urandom",
        OFlag::O_WRONLY,
        Mode::S_IRWXU,
    )?;
    write(raw_fd, SEED)?;
    close(raw_fd)?;

    Ok(())
}

fn set_hostname() -> crate::Result<()> {
    let fd = open("/proc/sys/kernel/hostname", OFlag::O_WRONLY, Mode::S_IWUSR)?;

    if let Ok(mut mapping) = FileMapping::open("/etc/hostname") {
        write(fd, mapping.as_slice())?;
        mapping.close()?;
    } else {
        write(fd, b"linux")?;
    }

    close(fd)?;

    Ok(())
}

pub fn boot_up_system() -> crate::Result<()> {
    // Make sure we're running with PID 1.
    ensure_running_as_init_system()?;

    // Get our signal handler up and running
    install_signal_handler()?;

    // Set hostname by reading /etc/hostname
    set_hostname()?;

    // Mount procfs, sysfs, /run and /dev
    mount_pseudo_filesystems()?;

    // Seed /dev/urandom
    seed_urandom()?;

    Ok(())
}
