use nix::unistd::getpid;

/// Returns true if the process is currently being run as the
/// init system (PID 1)
pub fn ensure_running_as_init_system() -> crate::error::Result<()>
{
    if getpid().as_raw() == 1 {
        Ok(())
    } else {
        Err(crate::error::Error::NotRunningAsInitSystem)
    }
}
