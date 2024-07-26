use std::fs;

pub fn list_files(directory: &str, extension: &str) -> Vec<String> {
    let mut files = Vec::new();

    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(_) => return files,
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = match path.extension() {
            Some(ext) => ext,
            None => continue,
        };

        if ext != extension {
            continue;
        }

        if let Some(path_str) = path.to_str() {
            files.push(path_str.to_string());
        }
    }

    files
}