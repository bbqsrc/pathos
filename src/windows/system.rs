use std::path::{Path, PathBuf};

#[inline]
fn config_dir() -> &'static Path {
    // TODO: use winapi to get the drive letter.
    r"C:\ProgramData\".as_ref()
}

#[inline]
pub fn app_data_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    config_dir().join(prefix)
}

#[inline]
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_data_dir(prefix).join("config")
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_data_dir(prefix).join("cache")
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_data_dir(prefix).join("log")
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_data_dir(prefix).join("cache/tmp")
}
