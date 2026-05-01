use crate::types::ApplicationErrors;
use std::path::{Path, PathBuf};
use std::fs;

pub fn create_converted_folder(base_path: &Path) -> Result<PathBuf, ApplicationErrors> {
    if !base_path.is_dir() {
        return Err(ApplicationErrors::NotADirectory(base_path.display().to_string()));
    }

    let mut new_path = base_path.to_path_buf();
    new_path.push("Converted");

    if !new_path.exists() {
        fs::create_dir(&new_path).map_err(|_| {
            ApplicationErrors::FolderCreationFailed(new_path.display().to_string())
        })?;
    }

    Ok(new_path)
}

pub fn get_image_files(base_path: &Path) -> Result<Vec<PathBuf>, ApplicationErrors> {
    let extensions = ["jpg", "jpeg", "png", "webp", "bmp"];
    let mut files = Vec::new();

    let entries = fs::read_dir(base_path)
        .map_err(|_| ApplicationErrors::ReadDirFailed(base_path.display().to_string()))?;

    for entry in entries {
        let path = entry?.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if extensions.contains(&ext.to_lowercase().as_str()) {
                    files.push(path);
                }
            }
        }
    }

    Ok(files)
}