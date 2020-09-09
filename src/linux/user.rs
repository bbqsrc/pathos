use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

use crate::{xdg, xdg::home_dir, AppDirs as _, Error, UserDirs};

static DIRS: Lazy<Result<Dirs, Error>> = Lazy::new(|| UserDirs::new());

pub struct Dirs {
    home_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
}

impl UserDirs for Dirs {
    fn new() -> Result<Self, Error> {
        Ok(Self {
            home_dir: home_dir()?,
            data_dir: xdg::data_home()?,
            cache_dir: xdg::cache_home()?,
        })
    }

    fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}

struct AppDirs {
    data_dir: PathBuf,
    config_dir: PathBuf,
    cache_dir: PathBuf,
    log_dir: PathBuf,
    temporary_dir: PathBuf,
}

impl crate::AppDirs for AppDirs {
    fn new<P>(prefix: P) -> Result<Self, Error>
    where
        P: Into<PathBuf>,
    {
        let prefix = prefix.into();

        let data_dir = xdg::data_home()?.join(&prefix);
        let config_dir = xdg::config_home()?.join(&prefix);
        let cache_dir = xdg::cache_home()?.join(&prefix);
        let temporary_dir = cache_dir.join("tmp");
        let log_dir = data_dir.join("log");

        let user_dirs = Self {
            config_dir,
            temporary_dir,
            log_dir,
            data_dir,
            cache_dir,
        };

        user_dirs.create()?;

        Ok(user_dirs)
    }

    fn create(&self) -> Result<(), Error> {
        let dirs = [
            self.data_dir(),
            self.config_dir(),
            self.cache_dir(),
            self.temporary_dir(),
            self.log_dir(),
        ];

        for dir in dirs.iter() {
            std::fs::create_dir_all(dir).map_err(|e| {
                Error::CreateDirectoryFailed(eieio::Error::from(e), dir.to_path_buf())
            })?;
        }
        // TODO: set tmp writable only by creator.

        Ok(())
    }

    fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    fn log_dir(&self) -> &Path {
        &self.log_dir
    }

    fn temporary_dir(&self) -> &Path {
        &self.temporary_dir
    }
}

#[inline]
pub fn data_dir() -> Result<&'static Path, Error> {
    dir!(|x| x.data_dir())
}

#[inline]
pub fn cache_dir() -> Result<&'static Path, Error> {
    dir!(|x| x.cache_dir())
}

#[inline]
pub fn app_data_dir<P: Into<PathBuf>>(prefix: P) -> Result<PathBuf, Error> {
    AppDirs::new(prefix).map(|x| x.data_dir().to_path_buf())
}

#[inline]
pub fn app_config_dir<P: Into<PathBuf>>(prefix: P) -> Result<PathBuf, Error> {
    AppDirs::new(prefix).map(|x| x.config_dir().to_path_buf())
}

#[inline]
pub fn app_log_dir<P: Into<PathBuf>>(prefix: P) -> Result<PathBuf, Error> {
    AppDirs::new(prefix).map(|x| x.log_dir().to_path_buf())
}

#[inline]
pub fn app_cache_dir<P: Into<PathBuf>>(prefix: P) -> Result<PathBuf, Error> {
    AppDirs::new(prefix).map(|x| x.cache_dir().to_path_buf())
}

#[inline]
pub fn app_temporary_dir<P: Into<PathBuf>>(prefix: P) -> Result<PathBuf, Error> {
    AppDirs::new(prefix).map(|x| x.temporary_dir().to_path_buf())
}

pub mod iri {
    use crate::{Error, ResolveError};
    use iref::IriBuf;
    use std::path::PathBuf;

    #[inline]
    pub fn app_cache_dir<P: Into<PathBuf>>(prefix: P) -> Result<IriBuf, Error> {
        super::app_cache_dir(prefix).and_then(|x| Ok(crate::file_path(x)?))
    }

    #[inline]
    pub fn app_temporary_dir<P: Into<PathBuf>>(prefix: P) -> Result<IriBuf, Error> {
        super::app_cache_dir(prefix).and_then(|x| Ok(crate::file_path(x)?))
    }

    pub fn resolve(iri: &iref::IriBuf) -> Result<PathBuf, ResolveError> {
        match iri.scheme().as_str() {
            "file" => crate::resolve_file_iri(iri),
            unhandled => Err(ResolveError::InvalidScheme(
                unhandled.to_string(),
                &["file"],
            )),
        }
    }
}
