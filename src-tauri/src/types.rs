use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationErrors {
    #[error("Path is not a directory: {0}")]
    NotADirectory(String),
    #[error("Failed folder creation: {0}")]
    FolderCreationFailed(String),
    #[error("Failed to read directory: {0}")]
    ReadDirFailed(String),
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
