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

pub mod iri {
    use iref::IriBuf;
    use std::path::Path;

    #[inline]
    pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> IriBuf {
        crate::file_path(super::app_temporary_dir(prefix))
    }

    #[inline]
    pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> IriBuf {
        crate::file_path(super::app_cache_dir(prefix))
    }
}
