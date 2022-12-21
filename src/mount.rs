use nix::mount::{mount, MsFlags};
use cstr::cstr;

pub fn mount_pseudo_filesystems() -> nix::Result<()> {
    let mode0755 = cstr!("mode=0755");

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
        Some(mode0755),
    )?;

    mount(
        Some(cstr!("dev")),
        cstr!("/dev"),
        Some(cstr!("devtmpfs")),
        MsFlags::MS_NOSUID,
        Some(mode0755),
    )?;

    Ok(())
}
