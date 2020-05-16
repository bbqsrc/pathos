use std::path::{Path, PathBuf};

#[inline]
pub fn home_dir() -> PathBuf {
    dirs::home_dir().expect("no home directory")
}

#[inline]
pub fn create_app_dirs<P: AsRef<Path>>(prefix: P) -> Result<(), std::io::Error> {
    let p = prefix.as_ref();
    std::fs::create_dir_all(app_config_dir(p))?;
    std::fs::create_dir_all(app_log_dir(p))?;
    std::fs::create_dir_all(app_cache_dir(p))?;

    // TODO: set tmp writable only by creator.
    std::fs::create_dir_all(app_temporary_dir(p))?;

    Ok(())
}

#[inline]
pub fn data_dir() -> PathBuf {
    home_dir().join("Library/Application Support/")
}

#[inline]
pub fn config_dir() -> PathBuf {
    home_dir().join("Library/Preferences/")
}

#[inline]
pub fn cache_dir() -> PathBuf {
    home_dir().join("Library/Caches/")
}

#[inline]
pub fn log_dir() -> PathBuf {
    home_dir().join("Library/Logs/")
}

#[inline]
pub fn app_data_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    data_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    config_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    log_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    cache_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_cache_dir(prefix).join("tmp")
}

pub mod iri {
    use iref::IriBuf;
    use std::path::Path;

    #[inline]
    pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> IriBuf {
        crate::file_path(super::app_cache_dir(prefix).to_string_lossy())
    }

    #[inline]
    pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> IriBuf {
        crate::file_path(super::app_cache_dir(prefix).to_string_lossy())
    }

    pub fn resolve(iri: &iref::IriBuf) -> Result<std::path::PathBuf, crate::ResolveError> {
        match iri.scheme().as_str() {
            "file" => Ok(std::path::PathBuf::from(iri.path().as_pct_str().decode())),
            "container" => {
                let mut path = iri.path().as_pct_str().decode();
                if path.starts_with("/") {
                    path = path.chars().skip(1).collect::<String>();
                }
                Ok(super::home_dir().join(path))
            }
            unhandled => Err(crate::ResolveError::InvalidScheme(unhandled.to_string())),
        }
    }
}
