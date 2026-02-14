use std::path::{Path, PathBuf};

//No idea if it works but its there
#[allow(dead_code)]
pub fn open_file_explorer(folder_path: &Path) {
    let _ = std::process::Command::new("xdg-open")
        .arg(folder_path)
        .spawn();
}

//No idea if it works but its there
#[allow(dead_code)]
pub fn open_program(path: &PathBuf) {
    let _ = std::process::Command::new("xdg-open").arg(path).spawn();
}
