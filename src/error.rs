use nix::errno::Errno;

#[derive(Debug)]
pub enum Error {
    UnexpectedEmptyFile,
    MmapFailed,
    NotRunningAsInitSystem,
    Errno(Errno),
}

impl Error {
    pub fn description(&self) -> &str {
        match self {
            Error::UnexpectedEmptyFile => {
                "found an empty file where content was expected"
            }
            Error::MmapFailed => "mmap returned a null pointer",
            Error::NotRunningAsInitSystem => "not running as PID 1",
            Error::Errno(errno) => errno.desc(),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<Errno> for Error {
    fn from(value: Errno) -> Self {
        Self::Errno(value)
    }
}
