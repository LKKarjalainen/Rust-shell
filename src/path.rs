use std::env;
use std::fs::{self, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub fn search_path(command: &str) -> Option<String> {
    let path: String = env::var("PATH").ok()?;
    let path_dirs: Vec<&str> = path.split(':').collect();
    for path_dir in path_dirs {
        let command_path: String = format!("{}/{}", path_dir, command);
        if Path::new(&command_path).exists() {
            let metadata: Metadata = fs::metadata(&command_path).unwrap();
            if metadata.mode() & 0o111 != 0 {
                return Some(command_path);
            }
        }
    }
    None
}
