use std::env;
use std::error;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(io::Error),
    EnvVar(env::VarError),
    LineParse(String, usize),
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        if let Error::Io(err) = self {
            return err.kind() == io::ErrorKind::NotFound;
        }

        false
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => err.fmt(fmt),
            Error::EnvVar(err) => err.fmt(fmt),
            Error::LineParse(text, index) => {
                write!(fmt, "Failed to parse '{}' at line {}", text, index)
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::EnvVar(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Self {
        Error::EnvVar(err)
    }
}
