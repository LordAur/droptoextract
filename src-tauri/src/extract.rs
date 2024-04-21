use std::fs::{File, read_to_string};
use std::path::Path;
use std::io::copy;
use zip::read::ZipArchive;

fn sanitize_filename(filename: &str) -> String {
    let sanitized: String = filename.chars()
    .filter(|c| match *c {
        // Add more unsupported characters if needed
        '/' | '\\' | ':' | '*'  | '\"' | '<' | '>' | '|' | '.' => false,
        _ => true,
    })
    .collect();

    sanitized
}

#[tauri::command]
pub fn unzip_file(file_path: &str) -> bool { 
    // get config
    let dir_cache = dirs::cache_dir().unwrap();
    let cfg_dir = format!("{}/{}", dir_cache.to_str().unwrap(), "droptoextract");
    let cfg_file = format!("{}/{}", cfg_dir, "config.json");
    let path = Path::new(cfg_file.as_str());
    let data = read_to_string(path).expect("Unable to read file");
    let config: serde_json::Value = serde_json::from_str(&data).unwrap();
    let delete_after_complete = config["delete_after_complete"].as_bool().unwrap_or(false);

    let p = Path::new(&file_path);

    let parent_folder = p.parent().unwrap().display();
    let file_name = sanitize_filename(p.file_stem().unwrap().to_str().unwrap());
    let file_name_str = file_name.as_str();

    let mut dest_path = Path::new(&parent_folder.to_string()).join(file_name_str);
    let tmp_dest_path = Path::new(&dest_path);
    println!("exists: {}", tmp_dest_path.exists());
    if !tmp_dest_path.exists() {
        std::fs::create_dir_all(&dest_path).unwrap();
    } else {
        dest_path = Path::new(&parent_folder.to_string()).join(file_name_str).join("{1}");
        std::fs::create_dir_all(&dest_path).unwrap();
    }

    match p.extension() {
        Some(ext) if ext == "zip" => {
            let file = File::open(file_path).unwrap();
            match ZipArchive::new(file) {
                Ok(mut archive) => {
                    for i in 0..archive.len() {
                        let mut file = archive.by_index(i).unwrap();
                        let outpath = dest_path.join(file.name().to_string());
                        let mut outfile = File::create(&outpath).unwrap();
        
                        copy(&mut file, &mut outfile).unwrap();
                    }
                    if delete_after_complete {
                        std::fs::remove_file(file_path).unwrap();
                    }

                    return true
                },
                Err(e) => {
                    println!("error: {}", e);
                    return false
                }
            }
        },
        Some(ext) if ext == "rar" => {
            println!("filepath: {} === destpath: {}", file_path, dest_path.to_str().unwrap());
            match rar::Archive::extract_all(file_path, dest_path.to_str().unwrap(), "") {
                Ok(_) => {
                    println!("berhasil");
                    if delete_after_complete {
                        std::fs::remove_file(file_path).unwrap();
                    }
                    return true
                },
                Err(e) => {
                    println!("error extract rar: {}", e);
                    return false
                }
            }
        },
        Some(_) => {
            return false
        },
        None =>{
            return false
        },
    }
}