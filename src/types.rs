use thiserror::Error;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
#[command(name = "LStickerConverter-rs")]
#[command(about = "Converts photos to fit the Telegram sticker format.", long_about = None)]
pub struct Args {
    /// The path to the photo folder
    #[arg(short, long)]
    pub path: PathBuf,
    /// Flag: Do not resize the photo
    #[arg(short, long, default_value_t = false)]
    pub no_resize: bool
}

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