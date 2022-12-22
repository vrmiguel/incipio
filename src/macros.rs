#[macro_export]
/// Builds the command array to be used with `execv`.
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
        $crate::exec::fork_and_execute_command([cstr::cstr!($x).as_ptr(), $(cstr::cstr!($y).as_ptr(),)* core::ptr::null()])?
    )
}
