use std::path::PathBuf;

pub fn open_file_explorer(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        let normalized_path = dunce::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());

        #[cfg(target_os = "windows")]
        {
            use crate::helpers::win_os_helper;
            win_os_helper::open_file_explorer(normalized_path);
        }

        #[cfg(target_os = "macos")]
        {
            use crate::helpers::mac_os_helper;
            mac_os_helper::open_file_explorer(parent);
        }
        #[cfg(target_os = "linux")]
        {
            use crate::helpers::linux_os_helper;
            linux_os_helper::open_file_explorer(parent);
        }
    }
}

pub fn open_program(path: &PathBuf) {
    #[cfg(target_os = "windows")]
    {
        use crate::helpers::win_os_helper;
        win_os_helper::open_program(path);
    }

    #[cfg(target_os = "macos")]
    {
        use crate::helpers::mac_os_helper;
        mac_os_helper::open_program(path);
    }
    #[cfg(target_os = "linux")]
    {
        use crate::helpers::linux_os_helper;
        linux_os_helper::open_program(path);
    }
}
