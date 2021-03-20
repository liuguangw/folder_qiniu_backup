mod archive_error;
mod zip_folder;

pub use archive_error::ArchiveError;
pub use zip_folder::zip_folder;
pub type ArchiveResult = Result<(), ArchiveError>;
