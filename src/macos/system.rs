use std::path::{Path, PathBuf};

use fruity::foundation::{
    NSApplicationSupportDirectory, NSCachesDirectory, NSLibraryDirectory, NSSystemDomainMask,
};

use crate::Error;

#[inline]
pub fn services_dir() -> &'static Path {
    static_path!(NSLibraryDirectory, NSSystemDomainMask, "Services")
        .as_ref()
        .unwrap()
}

#[inline]
pub fn application_support_dir() -> &'static Path {
    static_path!(NSApplicationSupportDirectory, NSSystemDomainMask)
        .as_ref()
        .unwrap()
}

#[inline]
pub fn data_dir() -> &'static Path {
    application_support_dir()
}

#[inline]
pub fn cache_dir() -> &'static Path {
    static_path!(NSCachesDirectory, NSSystemDomainMask)
        .as_ref()
        .unwrap()
}

#[inline]
pub fn log_dir() -> &'static Path {
    static_path!(NSLibraryDirectory, NSSystemDomainMask, "Logs")
        .as_ref()
        .unwrap()
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

        let data_dir = application_support_dir().join(&prefix);
        let cache_dir = cache_dir().join(&prefix);

        let app_dirs = Self {
            config_dir: data_dir.join("config"),
            temporary_dir: cache_dir.join("tmp"),
            log_dir: log_dir().join(&prefix),
            data_dir,
            cache_dir,
        };

        app_dirs.create()?;

        Ok(app_dirs)
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
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    data_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    cache_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_cache_dir(prefix).join("tmp")
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    log_dir().join(prefix.as_ref())
}

pub mod iri {
    use crate::path::absolute::AbsolutePathBufExt;
    use crate::Error;
    use iref::IriBuf;
    use std::path::Path;

    #[inline]
    pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(super::app_temporary_dir(prefix)
            .to_absolute_path_buf()?
            .to_file_iri()?)
    }

    #[inline]
    pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(super::app_cache_dir(prefix)
            .to_absolute_path_buf()?
            .to_file_iri()?)
    }
}
