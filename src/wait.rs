use nix::{
    errno::Errno,
    sys::wait::{waitpid, WaitPidFlag, WaitStatus},
    unistd::Pid,
};

/// Wait for the given PID retrying if interrupted
#[inline(always)]
pub fn wait_pid_no_interrupt(
    pid: impl Into<Option<Pid>>,
    flags: impl Into<Option<WaitPidFlag>>,
) -> nix::Result<WaitStatus> {
    let (pid, flags) = (pid.into(), flags.into());

    loop {
        match waitpid(pid, flags) {
            Ok(wait_status) => return Ok(wait_status),
            Err(Errno::EINTR) => {
                // We got interrupted while we were waiting.
                // Let's try again.
                continue;
            }
            Err(errno) => return Err(errno),
        }
    }
}

/// Reap processes spawned by `incipio`. The value returned is
/// the amount of processes reaped.
pub fn reap_child_processes() -> i32 {
    // Equivalent to `(pid_t)-1`; status is then requested for
    // any child process
    let pid = None;
    // waitpid shall not suspend execution of the calling thread
    // if status is not immediately available for  one  of  the
    // child  processes specified by pid
    let flags = WaitPidFlag::WNOHANG;
    let mut reaped = 0;

    while wait_pid_no_interrupt(pid, flags).is_ok() {
        reaped += 1;
    }

    reaped
}
