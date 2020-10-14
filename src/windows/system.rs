use crate::Error;
use std::path::{Path, PathBuf};

#[inline]
fn config_dir() -> &'static Path {
    // TODO: use winapi to get the drive letter.
    r"C:\ProgramData\".as_ref()
}

pub struct AppDirs {
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

        let data_dir = config_dir().join(&prefix);
        let cache_dir = config_dir().join(&prefix).join("cache");

        let user_dirs = Self {
            config_dir: data_dir.join("config"),
            log_dir: data_dir.join("log"),
            temporary_dir: cache_dir.join("tmp"),
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
    use crate::Error;
    use iref::IriBuf;
    use std::path::Path;

    #[inline]
    pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(crate::file_path(super::app_temporary_dir(prefix))?)
    }

    #[inline]
    pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(crate::file_path(super::app_cache_dir(prefix))?)
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
