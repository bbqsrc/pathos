use std::env::{split_paths, var_os};
use std::path::{Path, PathBuf};

#[inline(always)]
pub fn data_home() -> PathBuf {
    var_os("XDG_DATA_HOME")
        .map(|x| PathBuf::from(x))
        .filter(|x| x.is_absolute())
        .unwrap_or_else(|| dirs::home_dir().unwrap().join(".local/share"))
}

#[inline(always)]
pub fn config_home() -> PathBuf {
    var_os("XDG_CONFIG_HOME")
        .map(|x| PathBuf::from(x))
        .filter(|x| x.is_absolute())
        .unwrap_or_else(|| dirs::home_dir().unwrap().join(".config"))
}

#[inline(always)]
pub fn data_dirs() -> Vec<PathBuf> {
    var_os("XDG_DATA_DIRS")
        .map(|x| split_paths(&x).filter(|x| x.is_absolute()).collect())
        .unwrap_or_else(|| {
            vec![
                PathBuf::from("/usr/local/share/"),
                PathBuf::from("/usr/share/"),
            ]
        })
}

#[inline(always)]
pub fn config_dirs() -> Vec<PathBuf> {
    var_os("XDG_CONFIG_DIRS")
        .map(|x| split_paths(&x).filter(|x| x.is_absolute()).collect())
        .unwrap_or_else(|| vec![PathBuf::from("/etc/xdg/")])
}

#[inline(always)]
pub fn cache_home() -> PathBuf {
    var_os("XDG_CACHE_HOME")
        .map(|x| PathBuf::from(x))
        .filter(|x| x.is_absolute())
        .unwrap_or_else(|| dirs::home_dir().unwrap().join(".cache"))
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeDirError {
    #[error("XDG_RUNTIME_DIR environment variable is not defined.")]
    Undefined,

    #[error("Could not resolve metadata for XDG_RUNTIME_DIR.")]
    Metadata(std::io::Error),

    #[error("Invalid ownership. Requires ownership by {}, got: {0}.")]
    InvalidOwnership(u32, u32),

    #[error("Invalid access mode. Requires 0700, got {0:o}.")]
    InvalidAccessMode(u32),
}

#[inline(always)]
pub fn runtime_dir() -> Result<PathBuf, RuntimeDirError> {
    let dir = var_os("XDG_RUNTIME_DIR")
        .map(|x| PathBuf::from(x))
        .filter(|x| x.is_absolute())
        .ok_or_else(|| RuntimeDirError::Undefined)?;

    use std::os::unix::fs::MetadataExt;

    let meta = std::fs::metadata(&dir).map_err(RuntimeDirError::Metadata)?;

    let uid = unsafe { libc::getuid() };
    if meta.uid() != uid {
        return Err(RuntimeDirError::InvalidOwnership(uid, meta.uid()));
    }

    if meta.mode() != 0o700 {
        return Err(RuntimeDirError::InvalidAccessMode(meta.mode()));
    }

    Ok(dir)
}

#[inline]
pub fn app_data_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    data_home().join(prefix.as_ref())
}

#[inline]
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    config_home().join(prefix.as_ref())
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    data_home().join(prefix.as_ref()).join("log")
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    cache_home().join(prefix.as_ref())
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    cache_home().join(prefix.as_ref()).join("tmp")
}
