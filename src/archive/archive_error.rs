use std::io::Error;
use zip::result::ZipError;

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
