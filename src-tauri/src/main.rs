// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::fs::{self, File};
use std::io::Write;
use serde_json::json;

mod extract;

#[tauri::command]
fn get_config() -> serde_json::Value {
    let dir_cache = dirs::cache_dir().unwrap();
    let cfg_dir = format!("{}/{}", dir_cache.to_str().unwrap(), "droptoextract");
    let cfg_file = format!("{}/{}", cfg_dir, "config.json");
    let path = Path::new(cfg_file.as_str());
    let data = fs::read_to_string(path).expect("Unable to read file");
    let config: serde_json::Value = serde_json::from_str(&data).unwrap();
    config
}

#[tauri::command]
fn set_config(cfg: bool) {
    let dir_cache = dirs::cache_dir().unwrap();
    let cfg_dir = format!("{}/{}", dir_cache.to_str().unwrap(), "droptoextract");
    let cfg_file = format!("{}/{}", cfg_dir, "config.json");
    let path = Path::new(cfg_file.as_str());
    let config = json!({
        "delete_after_complete": cfg,
    });

    let json = serde_json::to_string_pretty(&config).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_config,
            set_config,
            extract::unzip_file
        ])
        .setup(|_| {
            let dir_cache = dirs::cache_dir().unwrap();
            let cfg_dir = format!("{}/{}", dir_cache.to_str().unwrap(), "droptoextract");
            let path = Path::new(cfg_dir.as_str());
            if !path.exists() {
                fs::create_dir_all(&path).expect("Failed to create directory");
            }
            let cfg_file = format!("{}/{}", cfg_dir, "config.json");
            let config = json!({
                "delete_after_complete": false,
            });
        
            let json = serde_json::to_string_pretty(&config).unwrap();
            let mut file = File::create(cfg_file).unwrap();
            file.write_all(json.as_bytes()).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
