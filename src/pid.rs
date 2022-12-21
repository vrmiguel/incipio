use nix::unistd::getpid;

/// Returns true if the process is currently being run as the
/// init system (PID 1)
pub fn is_running_as_init_system() -> bool {
    getpid().as_raw() == 1
}