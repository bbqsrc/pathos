use fruity::foundation::{
    NSApplicationSupportDirectory, NSCachesDirectory, NSLibraryDirectory, NSUserDomainMask,
};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

use crate::{AppDirs as _, Error, UserDirs};

static DIRS: Lazy<Result<Dirs, Error>> = Lazy::new(|| Dirs::new());

pub struct Dirs {
    home_dir: PathBuf,
    data_dir: &'static PathBuf,
    cache_dir: &'static PathBuf,
    log_dir: &'static PathBuf,
    prefs_dir: &'static PathBuf,
    services_dir: &'static PathBuf,
}

impl Dirs {
    fn application_support_dir(&self) -> &Path {
        &self.data_dir
    }

    fn log_dir(&self) -> &Path {
        &self.log_dir
    }

    fn preferences_dir(&self) -> &Path {
        &self.prefs_dir
    }

    fn services_dir(&self) -> &Path {
        &self.services_dir
    }
}

impl UserDirs for Dirs {
    fn new() -> Result<Self, Error> {
        let home_dir = PathBuf::from(
            super::FILE_MANAGER
                .home_directory_for_current_user()
                .path()
                .to_string(),
        );

        let dirs = (|| -> Result<Dirs, &Error> {
            // let lib_dir = static_path!(NSLibraryDirectory, NSUserDomainMask).as_ref()?,

            Ok(Self {
                data_dir: static_path!(NSApplicationSupportDirectory, NSUserDomainMask).as_ref()?,
                cache_dir: static_path!(NSCachesDirectory, NSUserDomainMask).as_ref()?,
                log_dir: static_path!(NSLibraryDirectory, NSUserDomainMask, "Logs").as_ref()?,
                prefs_dir: static_path!(NSLibraryDirectory, NSUserDomainMask, "Preferences")
                    .as_ref()?,
                services_dir: static_path!(NSLibraryDirectory, NSUserDomainMask, "Services")
                    .as_ref()?,
                home_dir,
            })
        })()
        .map_err(|e| e.clone())?;

        Ok(dirs)
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
        let dirs = Dirs::new()?;

        let data_dir = dirs.application_support_dir().join(&prefix);
        let cache_dir = dirs.cache_dir().join(&prefix);

        let user_dirs = Self {
            config_dir: data_dir.join("config"),
            temporary_dir: cache_dir.join("tmp"),
            log_dir: dirs.log_dir().join(&prefix),
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
pub fn home_dir() -> Result<&'static Path, Error> {
    dir!(|x| x.home_dir())
}

#[inline]
pub fn application_support_dir() -> Result<&'static Path, Error> {
    dir!(|x| x.application_support_dir())
}

#[inline]
pub fn data_dir() -> Result<&'static Path, Error> {
    application_support_dir()
}

#[inline]
pub fn preferences_dir() -> Result<&'static Path, Error> {
    dir!(|x| x.preferences_dir())
}

#[inline]
pub fn cache_dir() -> Result<&'static Path, Error> {
    dir!(|x| x.cache_dir())
}

#[inline]
pub fn log_dir() -> Result<&'static Path, Error> {
    dir!(|x| x.log_dir())
}

#[inline]
pub fn services_dir() -> Result<&'static Path, Error> {
    dir!(|x| x.services_dir())
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
    use crate::path::absolute::AbsolutePathBufExt;
    use crate::Error;
    use iref::IriBuf;
    use std::path::PathBuf;

    #[inline]
    pub fn app_cache_dir<P: Into<PathBuf>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(super::app_cache_dir(prefix)?
            .to_absolute_path_buf()?
            .to_file_iri()?)
    }

    #[inline]
    pub fn app_temporary_dir<P: Into<PathBuf>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(super::app_temporary_dir(prefix)?
            .to_absolute_path_buf()?
            .to_file_iri()?)
    }
}
