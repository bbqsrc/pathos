macro_rules! static_path {
    ($directory:path, $domain:path) => {{
        static CELL: once_cell::sync::OnceCell<Result<PathBuf, $crate::Error>> =
            once_cell::sync::OnceCell::new();
        CELL.get_or_init(|| {
            let ns_url = $crate::macos::FILE_MANAGER
                .url_for_directory($directory, $domain, None, false)
                .unwrap();
            let ns_string = ns_url.path();
            Ok(PathBuf::from(ns_string.to_string()))
        })
    }};
    ($directory:path, $domain:path, $extra:tt) => {{
        static CELL: once_cell::sync::OnceCell<Result<PathBuf, $crate::Error>> =
            once_cell::sync::OnceCell::new();
        CELL.get_or_init(|| {
            let ns_url = $crate::macos::FILE_MANAGER
                .url_for_directory($directory, $domain, None, false)
                .unwrap();
            let ns_string = ns_url.path();
            Ok(PathBuf::from(format!(
                "{}/{}",
                ns_string.to_string(),
                $extra
            )))
        })
    }};
}

pub mod system;
pub mod user;

use once_cell::sync::Lazy;

static FILE_MANAGER: Lazy<fruity::core::Arc<fruity::foundation::NSFileManager<'_>>> =
    Lazy::new(|| fruity::foundation::NSFileManager::default_manager());
