use std::error::Error as ErrorTrait;
use std::fmt;
use std::io::Error as IoError;

pub enum Error {
    IoError(IoError),
    CondensifyError(ErrorKind),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::IoError(e)
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Error::CondensifyError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
        match self {
            Error::IoError(e) => write!(f, e.description())?,
            Error::CondensifyError(e) => match e {
                ErrorKind::HashParseError => write!(f, "Hash parse error")?,
                ErrorKind::HashNotALink => write!(f, "Hash not a link")?,
            },
        }

        Ok(())
    }
}

impl ErrorTrait for Error {
    fn source(&self) -> Option<&dyn ErrorTrait> {
        match self {
            Error::IoError(e) => e,
            Error::CondensifyError(_) => None,
        }
    }
}

pub enum ErrorKind {
    HashParseError,
    HashNotALink,
}
