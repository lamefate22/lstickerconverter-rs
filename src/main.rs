use std::sync::atomic::{AtomicUsize, Ordering};
use crate::types::{Args, ApplicationErrors};
use std::path::PathBuf;
use rayon::prelude::*;
use clap::Parser;

mod filesystem;
mod types;
mod converter;


fn main() -> Result<(), ApplicationErrors> {
    let args: Args = Args::parse();

    let source_path: PathBuf = args.path;
    let should_resize: bool = !args.no_resize;

    let output_path: PathBuf = filesystem::create_converted_folder(&source_path)?;

    let files: Vec<PathBuf> = filesystem::get_image_files(&source_path)?;
    let total: usize = files.len();
    
    println!("Total of {} images were found at the specified path.", total);

    let count: AtomicUsize = AtomicUsize::new(0);

    files.par_iter().for_each(|file_path| {
        match converter::process_image(file_path, &output_path, should_resize) {
            Ok(_) => {
                let c = count.fetch_add(1, Ordering::SeqCst);
                println!("[{}/{}] Successfully: {:?}", c + 1, total, file_path.file_name().unwrap());
            }
            Err(e) => {
                eprintln!("Error while processing {:?}: {}", file_path, e);
            }
        }
    });

    println!("\nDone! Processed: {}", count.load(Ordering::SeqCst));
    println!("Results available in: {}", output_path.display());

    Ok(())
}