macro_rules! windows_path {
    ($folderid:path) => {{
        static CELL: once_cell::sync::OnceCell<Option<PathBuf>> = once_cell::sync::OnceCell::new();
        CELL.get_or_init(|| match windirs::known_folder_path($folderid) {
            Ok(path) => Some(path),
            Err(_) => None,
        })
    }};
}

pub mod system;
pub mod user;
