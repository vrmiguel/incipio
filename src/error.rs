use nix::errno::Errno;

#[derive(Debug)]
pub enum Error {
    NotRunningAsInitSystem,
    Errno(Errno),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<Errno> for Error {
    fn from(value: Errno) -> Self {
        Self::Errno(value)
    }
}
