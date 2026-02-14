use std::path::PathBuf;

pub fn open_file_explorer(normalized_path: PathBuf) {
    let _ = std::process::Command::new("explorer")
        .arg(normalized_path)
        .spawn();
}

pub fn open_program(path: &PathBuf) {
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", "", &path.to_string_lossy()])
        .spawn();
}
