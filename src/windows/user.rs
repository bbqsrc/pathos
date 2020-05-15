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
pub fn app_data_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    let base = directories::BaseDirs::new().unwrap();
    base.data_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_data_dir(prefix).join("config")
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    let base = directories::BaseDirs::new().unwrap();
    base.data_local_dir().join(prefix.as_ref()).join("cache")
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    let base = directories::BaseDirs::new().unwrap();
    base.data_local_dir().join(prefix.as_ref()).join("log")
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_cache_dir(prefix).join("tmp")
}

pub mod iri {
    pub fn resolve(iri: &iref::IriBuf) -> Result<std::path::PathBuf, crate::ResolveError> {
        match iri.scheme().as_str() {
            "file" => Ok(std::path::PathBuf::from(iri.path().into_str())),
            "container" => {
                Ok(super::home_dir().join(iri.path().into_str()))
            }
            unhandled => {
                Err(crate::ResolveError::InvalidScheme(unhandled.to_string()))
            }
        }
    }
}