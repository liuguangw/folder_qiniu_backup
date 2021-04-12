use std::io::Error;
use zip::result::ZipError;
use std::fmt::{Display, Formatter};

pub enum ArchiveError {
    IoError(std::io::Error),
    CompressError(ZipError),
}

impl From<std::io::Error> for ArchiveError {
    fn from(err: Error) -> Self {
        ArchiveError::IoError(err)
    }
}

impl From<ZipError> for ArchiveError {
    fn from(err: ZipError) -> Self {
        ArchiveError::CompressError(err)
    }
}

impl Display for ArchiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::IoError(inner_error) => inner_error.fmt(f),
            ArchiveError::CompressError(inner_error) => inner_error.fmt(f)
        }
    }
}