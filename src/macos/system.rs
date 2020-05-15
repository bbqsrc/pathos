use std::path::{Path, PathBuf};

#[inline]
fn config_dir() -> &'static Path {
    "/Library/Preferences/".as_ref()
}

#[inline]
fn cache_dir() -> &'static Path {
    "/Library/Caches/".as_ref()
}

#[inline]
fn log_dir() -> &'static Path {
    "/Library/Logs/".as_ref()
}

#[inline]
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    config_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    cache_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    log_dir().join(prefix.as_ref())
}
