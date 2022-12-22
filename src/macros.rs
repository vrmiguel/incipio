#[macro_export]
/// Forks the current process, runs the given command in the child
/// process and then waits for the child process to exit.
///
/// For example,
/// ```
/// run!("ls", "-li")
/// ```
/// generates
/// ```
/// fork_and_execute_command([cstr!("ls").as_ptr(), cstr!("-li").as_ptr(), core::ptr::null()])
/// ```
macro_rules! run {
    ($x:expr, $($y:expr),+) => (
        $crate::exec::fork_and_execute_command([cstr::cstr!($x).as_ptr(), $(cstr::cstr!($y).as_ptr(),)* core::ptr::null()], true)?
    )
}

/// Forks the current process, runs the given command in the child
/// process without waiting for the child process to exit.
#[macro_export]
macro_rules! run_in_background {
    ($x:expr, $($y:expr),+) => (
        $crate::exec::fork_and_execute_command([cstr::cstr!($x).as_ptr(), $(cstr::cstr!($y).as_ptr(),)* core::ptr::null()], false)?
    )
}
