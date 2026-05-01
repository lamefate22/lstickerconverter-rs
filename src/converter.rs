use crate::types::ApplicationErrors;
use std::path::{Path, PathBuf};
use image::DynamicImage;


pub fn process_image(input_path: &Path, output_dir: &Path, resize: bool) -> Result<(), ApplicationErrors> {
    let mut img: DynamicImage = image::open(input_path)?;

    if resize {
        img = img.thumbnail(512, 512);
    }

    let file_stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
    let mut output_path: PathBuf = output_dir.to_path_buf();
    output_path.push(format!("{}.png", file_stem));

    img.save(output_path)?;
    Ok(())
}