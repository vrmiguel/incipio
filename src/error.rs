use nix::errno::Errno;

#[derive(Debug)]
pub enum Error {
    MountPointParser,
    UnexpectedEmptyFile,
    WriteToString,
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
            Error::WriteToString => "failed to write to string",
            Error::MountPointParser => {
                "failed to parse mount point file"
            }
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
