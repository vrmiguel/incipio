use core::{ffi::CStr, ptr};

use cstr::cstr;
use libc_print::libc_eprintln;

use crate::{exec::fork_and_execute_command, utils::NixPathExt};

static GETTY: &CStr = cstr!("/usr/bin/getty");
static AGETTY: &CStr = cstr!("/usr/bin/agetty");

fn tty_opener_binary() -> Option<&'static CStr> {
    match (GETTY.is_executable(), AGETTY.is_executable()) {
        (true, _) => Some(GETTY),
        (_, true) => Some(AGETTY),
        (false, false) => None,
    }
}

pub fn open_ttys() {
    static BAUD_RATES: &CStr = cstr!("115200,38400,9600");
    // The value for the TERM env variable
    static TERM: &CStr = cstr!("linux");
    static TTYS: &[&CStr] = &[
        cstr!("tty1"),
        cstr!("tty2"),
        cstr!("tty3"),
        cstr!("tty4"),
        cstr!("tty5"),
        cstr!("tty6"),
        cstr!("tty7"),
        cstr!("tty8"),
    ];

    let Some(tty_opener) = tty_opener_binary() else {
        libc_eprintln!("No tty opener found!");
        return;
    };

    for tty in TTYS {
        let command = [
            tty_opener.as_ptr(),
            BAUD_RATES.as_ptr(),
            tty.as_ptr(),
            TERM.as_ptr(),
            ptr::null(),
        ];
        let should_wait = false;

        if let Err(err) =
            fork_and_execute_command(command, should_wait)
        {
            libc_eprintln!(
                "Failed to open {:?}: {}",
                tty,
                err.description()
            );
        }
    }
}
