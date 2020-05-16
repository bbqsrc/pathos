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
    app_data_dir(prefix).join("cache").join("tmp")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_app() {
        assert_eq!(
            app_temporary_dir("Special Company/Bad App"),
            Path::new(r"C:\ProgramData\Special Company\Bad App\cache\tmp\")
        )
    }
}
