use std::path::{Path, PathBuf};

#[inline]
pub fn app_data_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    PathBuf::from("/usr/share/").join(prefix)
}

#[inline]
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    PathBuf::from("/etc/").join(prefix)
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    PathBuf::from("/var/cache/").join(prefix)
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    PathBuf::from("/var/tmp/").join(prefix)
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    PathBuf::from("/var/log/").join(prefix)
}
