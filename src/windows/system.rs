use crate::Error;
use std::path::{Path, PathBuf};

#[inline]
fn config_dir() -> Result<&'static Path, Error> {
    windows_path!(windirs::FolderId::ProgramData)
        .as_ref()
        .map(|x| &**x)
        .ok_or_else(|| Error::NotFound("ProgramData"))
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

        let config_dir = config_dir()?;

        let data_dir = config_dir.join(&prefix);
        let cache_dir = data_dir.join("cache");

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
pub fn app_data_dir<P: AsRef<Path>>(prefix: P) -> Result<PathBuf, Error> {
    config_dir().map(|x| x.join(prefix))
}

#[inline]
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> Result<PathBuf, Error> {
    app_data_dir(prefix).map(|x| x.join("config"))
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> Result<PathBuf, Error> {
    app_data_dir(prefix).map(|x| x.join("cache"))
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> Result<PathBuf, Error> {
    app_data_dir(prefix).map(|x| x.join("log"))
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> Result<PathBuf, Error> {
    app_cache_dir(prefix).map(|x| x.join("tmp"))
}

pub mod iri {
    use crate::path::absolute::AbsolutePathBufExt;
    use crate::Error;
    use iref::IriBuf;
    use std::path::Path;

    #[inline]
    pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(super::app_temporary_dir(prefix)?
            .to_absolute_path_buf()?
            .to_file_iri()?)
    }

    #[inline]
    pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(super::app_cache_dir(prefix)?
            .to_absolute_path_buf()?
            .to_file_iri()?)
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn basic_app() {
        assert_eq!(
            app_temporary_dir("Special Company/Bad App").unwrap(),
            Path::new(r"C:\ProgramData\Special Company\Bad App\cache\tmp\")
        )
    }
}
