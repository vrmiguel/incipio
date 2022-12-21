use core::ffi::{c_char, CStr};

use cstr::cstr;
use nix::{
    mount::{mount, MsFlags},
    sys::stat::Mode,
    unistd::mkdir,
};

use crate::exec::fork_and_execute_command;

/// 755 means read and execute access for everyone and also write
/// access for the owner of the file.
macro_rules! perms_0755 {
    () => {
        // Work-around since bit-or for Mode is not const
        Mode::S_IRWXU | Mode::S_IXOTH | Mode::S_IROTH
    };
}

static MODE_0755: &CStr = cstr!("mode=0755");
static MOUNT: &CStr = cstr!("mount");
static REMOUNT_RW: &CStr = cstr!("remount,rw");
static ROOT: &CStr = cstr!("/");
static FLAG_ALL: &CStr = cstr!("-a");
static SWAPON: &CStr = cstr!("swapon");

pub fn mount_filesystem() -> crate::Result<()> {
    // Mount procfs, sysfs, /run and /dev
    mount_pseudo_filesystems()?;

    // Mount /dev/shm, /dev/pts/, /run/lock
    remaining_filesystem_runlevel()?;

    // Remount root
    // Runs `mount remount,rw /`
    run([
        MOUNT.as_ptr(),
        REMOUNT_RW.as_ptr(),
        ROOT.as_ptr(),
        core::ptr::null(),
    ])?;

    // Mount all filesystems
    // Runs `mount -a`
    run([MOUNT.as_ptr(), FLAG_ALL.as_ptr(), core::ptr::null()])?;

    // Turn on all swap partitions in /etc/fstab
    // Runs `swapon -a`
    run([SWAPON.as_ptr(), FLAG_ALL.as_ptr(), core::ptr::null()])?;

    Ok(())
}

fn run<const N: usize>(
    command: [*const c_char; N],
) -> crate::Result<()> {
    // Arguments to `execv` must end with a NULL pointer
    fork_and_execute_command(command)
}

/// Remaining FS run-level operations to take place after
/// pseudo-filesystems have been mounted.
// TODO: make these paths configurable for those who might want to
// mount /run/shm instead of /dev/shm, /var/lock instead of /run/lock,
// and so on
fn remaining_filesystem_runlevel() -> nix::Result<()> {
    fn mkdir_0755(path: &CStr) -> nix::Result<()> {
        mkdir(path, perms_0755!())
    }

    // Mounts pseudo-terminals through /dev/pts.
    // Assumes that `/dev/` has already been mounted.
    mkdir_0755(cstr!("/dev/pts"))?;

    // Mounts the temporary shared memory storage
    mkdir_0755(cstr!("/dev/shm"))?;

    // Mounts the directory for lock files, i.e. files indicating that
    // a shared device or other system resource is in use and
    // containing the identity of the process (PID) using it.
    mkdir_0755(cstr!("/run/lock"))
}

fn mount_pseudo_filesystems() -> nix::Result<()> {
    // Mounting procfs
    mount(
        Some(cstr!("proc")),
        cstr!("/proc"),
        Some(cstr!("proc")),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None as Option<&str>,
    )?;

    // Mounting sys
    mount(
        Some(cstr!("sys")),
        cstr!("/sys"),
        Some(cstr!("sysfs")),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None as Option<&str>,
    )?;

    mount(
        Some(cstr!("run")),
        cstr!("/run"),
        Some(cstr!("tmpfs")),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        Some(MODE_0755),
    )?;

    mount(
        Some(cstr!("dev")),
        cstr!("/dev"),
        Some(cstr!("devtmpfs")),
        MsFlags::MS_NOSUID,
        Some(MODE_0755),
    )?;

    Ok(())
}
