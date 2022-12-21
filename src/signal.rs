use nix::{
    libc::{
        LINUX_REBOOT_CMD_POWER_OFF, LINUX_REBOOT_CMD_RESTART,
    },
    sys::{
        signal::{sigaction, SaFlags, SigAction, Signal},
        signalfd::SigSet,
    },
};

use crate::wait::reap_child_processes;

type Callback = fn() -> i32;

/// Wrapper for [`reap_child_processes`] that returns -1 instead
/// of the amount of reaped processes.
///
/// This is needed since `handle_signal` will reboot the system
/// if the callback is larger than 0.
fn reap_child_processes_wrapper() -> i32 {
    reap_child_processes();
    -1
}

fn restart_machine() -> i32 {
    LINUX_REBOOT_CMD_RESTART
}

fn shutdown_machine() -> i32 {
    LINUX_REBOOT_CMD_POWER_OFF
}

extern "C" fn handle_signal(received_signal: i32) {
    if let Some(callback) = signal_to_action(received_signal) {
        let ret_val = callback();

        if ret_val < 0 {
            return;
        }
    }

    // TODO: shutdown and reboot the system
}

/// Installs the global signal handler for this process
pub fn install_signal_handler() -> nix::Result<()> {
    // Handle signals with the `handle_signal` function.
    let handler =
        nix::sys::signal::SigHandler::Handler(handle_signal);

    // The action to take whenever we receive a signal interrupt
    let sig_action = SigAction::new(
        // Send signals to the `handle_signal` function
        handler,
        SaFlags::empty(),
        // sigemptyset
        SigSet::empty(),
    );

    for signal in
        [Signal::SIGINT, Signal::SIGUSR1, Signal::SIGCHLD]
    {
        unsafe { sigaction(signal, &sig_action)? };
    }

    Ok(())
}

pub fn signal_to_action(signal: i32) -> Option<Callback> {
    match signal {
        nix::libc::SIGCHLD => Some(reap_child_processes_wrapper),
        nix::libc::SIGUSR1 => Some(shutdown_machine),
        nix::libc::SIGINT => Some(restart_machine),
        _ => None,
    }
}
