// use openssl::error::ErrorStack;
use super::vcs::VcsError;
use curl::{Error as CurlError, FormError as CurlFormError};
use git2::Error as Git2Error;
use serde_json::Error as SerdeJsonError;
use std::io::Error as IoError;
use std::str::Utf8Error;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;

#[derive(Debug)]
pub enum ErrorKind {
    // Openssl(ErrorStack),
    TomlSer(TomlSerError),
    TomlDe(TomlDeError),
    IoError(IoError),
    Git2Error(Git2Error),
    VcsError(VcsError),
    CurlError(CurlError),
    CurlFormError(CurlFormError),
    EnvironmentVariableNotFound(String),
    SerdeJsonError(SerdeJsonError),
    Utf8Error(Utf8Error),
}

pub type Error = Box<ErrorKind>;

pub type Result<T> = std::result::Result<T, Error>;

// impl From<ErrorStack> for Error {
//     fn from(err: ErrorStack) -> Self {
//         Box::new(ErrorKind::Openssl(err))
//     }
// }

impl From<TomlSerError> for Error {
    fn from(err: TomlSerError) -> Self {
        Box::new(ErrorKind::TomlSer(err))
    }
}

impl From<TomlDeError> for Error {
    fn from(err: TomlDeError) -> Self {
        Box::new(ErrorKind::TomlDe(err))
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Box::new(ErrorKind::IoError(err))
    }
}

impl From<Git2Error> for Error {
    fn from(err: Git2Error) -> Self {
        Box::new(ErrorKind::Git2Error(err))
    }
}

impl From<VcsError> for Error {
    fn from(err: VcsError) -> Self {
        Box::new(ErrorKind::VcsError(err))
    }
}

impl From<CurlError> for Error {
    fn from(err: CurlError) -> Self {
        Box::new(ErrorKind::CurlError(err))
    }
}

impl From<CurlFormError> for Error {
    fn from(err: CurlFormError) -> Self {
        Box::new(ErrorKind::CurlFormError(err))
    }
}

impl From<SerdeJsonError> for Error {
    fn from(err: SerdeJsonError) -> Self {
        Box::new(ErrorKind::SerdeJsonError(err))
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Box::new(ErrorKind::Utf8Error(err))
    }
}