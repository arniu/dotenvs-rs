use std::env;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(io::Error),
    Env(env::VarError),
    Parse(String),
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        if let Error::Io(err) = self {
            return err.kind() == io::ErrorKind::NotFound;
        }

        false
    }

    pub(crate) fn not_found() -> Self {
        io::Error::new(io::ErrorKind::NotFound, "path not found").into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => err.fmt(fmt),
            Error::Env(err) => err.fmt(fmt),
            Error::Parse(err) => err.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Env(err) => Some(err),
            Error::Parse(_) => None,
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
        Error::Env(err)
    }
}

impl<I: Display> From<nom::error::Error<I>> for Error {
    fn from(err: nom::error::Error<I>) -> Self {
        Error::Parse(format!("{err}"))
    }
}
