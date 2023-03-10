use libc_print::libc_eprintln;
use nix::{
    fcntl::{open, OFlag},
    libc::{reboot, LINUX_REBOOT_CMD_CAD_OFF},
    sys::stat::Mode,
    unistd::{close, write},
};

use crate::{
    mount::{turn_off_swap_partitions, unmount_all_filesystems},
    rand_seed::SEED,
    tty::open_ttys,
    utils::FileMapping,
};

pub fn boot_up_system() -> crate::Result<()> {
    // Set hostname by reading /etc/hostname
    set_hostname()?;

    // Seed /dev/urandom
    seed_urandom()?;

    // Stop CAD from rebooting the system
    disable_control_alt_del();

    // Open TTYs
    open_ttys();

    Ok(())
}

pub fn boot_down_system() -> crate::Result<()> {
    if let Err(err) = turn_off_swap_partitions() {
        libc_eprintln!(
            "Failed to turn off swap partitions: {}",
            err.description()
        );
    }

    // nftw()
    // nix::libc::nftw

    unmount_all_filesystems()
}

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
    let fd = open(
        "/proc/sys/kernel/hostname",
        OFlag::O_WRONLY,
        Mode::S_IWUSR,
    )?;

    if let Ok(mut mapping) = FileMapping::open("/etc/hostname") {
        write(fd, mapping.as_slice())?;
        mapping.close()?;
    } else {
        write(fd, b"linux")?;
    }

    close(fd)?;

    Ok(())
}

/// Makes it so that Ctrl+Alt+Del (CAD, or tree-finger-salute) no
/// longer reboots the system.
///
/// After this call, CAD will only send a SIGINT interrupt to PID 1.
fn disable_control_alt_del() {
    // After CAD is disabled, the CAD keystroke will cause a SIGINT
    // signal to be sent to incipio causing it to reap children
    // processes.
    unsafe { reboot(LINUX_REBOOT_CMD_CAD_OFF) };
}
