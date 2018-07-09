use std::error;
use std::fmt;

use serde_json;
use subprocess::PopenError;

#[derive(Debug)]
pub enum NotmuchError {
    SerializationError(serde_json::Error),
    SubprocessError(PopenError),
}

impl fmt::Display for NotmuchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotmuchError::SerializationError(ref e) => e.fmt(f),
            NotmuchError::SubprocessError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for NotmuchError {
    fn description(&self) -> &str {
        match *self {
            NotmuchError::SerializationError(ref e) => e.description(),
            NotmuchError::SubprocessError(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            NotmuchError::SerializationError(ref e) => Some(e),
            NotmuchError::SubprocessError(ref e) => Some(e),
        }
    }
}

impl From<serde_json::Error> for NotmuchError {
    fn from(err: serde_json::Error) -> NotmuchError {
        NotmuchError::SerializationError(err)
    }
}

impl From<PopenError> for NotmuchError {
    fn from(err: PopenError) -> NotmuchError {
        NotmuchError::SubprocessError(err)
    }
}
