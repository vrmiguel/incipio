use core::ffi::CStr;

use cstr::cstr;
use libc_print::libc_eprintln;
use nix::{
    mount::{mount, umount, MsFlags},
    sys::stat::Mode,
    unistd::mkdir,
};

use crate::{fs::MountPointParser, run};

/// 755 means read and execute access for everyone and also write
/// access for the owner of the file.
macro_rules! perms_0755 {
    () => {
        // Work-around since bit-or for Mode is not const
        Mode::S_IRWXU | Mode::S_IXOTH | Mode::S_IROTH
    };
}

static ROOT: &CStr = cstr!("/");
static MODE_0755: &CStr = cstr!("mode=0755");

pub fn mount_filesystem() -> crate::Result<()> {
    // Mount procfs, sysfs, /run and /dev
    mount_pseudo_filesystems()?;

    // Mount /dev/shm, /dev/pts/, /run/lock
    remaining_filesystem_runlevel()?;

    // Remount root
    run!("/usr/bin/mount", "remount,rw", "/");

    // Mount all filesystems
    run!("/usr/bin/mount", "-a");

    // Turn on all swap partitions in /etc/fstab
    // Runs `swapon -a`
    run!("/usr/bin/swapon", "-a");

    Ok(())
}

pub fn turn_off_swap_partitions() -> crate::Result<()> {
    run!("/usr/bin/swapoff", "-a");
    Ok(())
}

/// Read through the entries of `/proc/mounts` attempting to unmount
/// the file systems found.
///
/// If an unmount operation fails, we'll try to remount the given FS
/// as read-only
pub fn unmount_all_filesystems() -> crate::Result<()> {
    let parser = MountPointParser::new(cstr!("/proc/mounts"))?;
    let is_root = |path: &CStr| path == ROOT;

    for entry in parser {
        let path = entry.path();

        // We'll try to remount this file system as read-only
        // if unmounting it fails or if it's the root partition
        let mut should_remount = is_root(path);

        if let Err(err) = umount(path) {
            libc_eprintln!("Failed to unmount {:?}: {}", path, err);
            should_remount = true;
        }

        if should_remount {
            if let Err(err) = mount(
                None as Option<&str>,
                path,
                None as Option<&str>,
                MsFlags::MS_REMOUNT | MsFlags::MS_RDONLY,
                None as Option<&str>,
            ) {
                libc_eprintln!(
                    "Failed to remount {:?} as read-only: {}",
                    path,
                    err
                );
            }
        }
    }

    Ok(())
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
