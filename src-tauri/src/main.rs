#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::filesystem::{create_converted_folder, get_image_files};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::converter::process_image;
use std::path::PathBuf;
use rayon::prelude::*;
use serde::Serialize;
use tauri::Emitter;

mod filesystem;
mod converter;
mod types;

#[derive(Serialize, Clone)]
struct ProgressPayload {
    current: usize,
    total: usize,
    filename: String,
}

#[derive(Serialize)]
struct ProcessResult {
    success_count: usize,
    total_count: usize,
    output_path: String,
}

#[tauri::command]
async fn start_conversion(window: tauri::Window, path: String, resize: bool) -> Result<ProcessResult, String> {
    let source_path = PathBuf::from(&path);

    let output_path = create_converted_folder(&source_path).map_err(|e| e.to_string())?;
    let files = get_image_files(&source_path).map_err(|e| e.to_string())?;

    let total_count = files.len();
    if total_count == 0 {
        return Err("No images found.".to_string());
    }

    let success_count = AtomicUsize::new(0);
    let processed_count = AtomicUsize::new(0);

    files.par_iter().for_each(|file_path| {
        let res = process_image(file_path, &output_path, resize);

        let current_processed = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
        if res.is_ok() {
            success_count.fetch_add(1, Ordering::SeqCst);
        }

        let _ = window.emit(
            "progress-update",
            ProgressPayload {
                current: current_processed,
                total: total_count,
                filename: file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into(),
            },
        );
    });

    Ok(ProcessResult {
        success_count: success_count.load(Ordering::SeqCst),
        total_count,
        output_path: output_path.display().to_string(),
    })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![start_conversion])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
