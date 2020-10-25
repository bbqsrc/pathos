use std::path::{Path, PathBuf};

use crate::{AppDirs as _, Error, UserDirs};
use once_cell::sync::Lazy;

static DIRS: Lazy<Result<Dirs, Error>> = Lazy::new(|| Dirs::new());

#[inline]
pub(crate) fn home_dir() -> Result<PathBuf, Error> {
    home::home_dir().ok_or_else(|| Error::NoHomeDirectory)
}

pub struct Dirs {
    home_dir: PathBuf,
    roaming_dir: PathBuf,
    local_dir: PathBuf,
}

impl UserDirs for Dirs {
    fn new() -> Result<Self, Error> {
        let home_dir = home_dir()?;
        let appdata_dir = home_dir.join("AppData");
        let roaming_dir = appdata_dir.join("Roaming");
        let local_dir = appdata_dir.join("Local");

        Ok(Self {
            local_dir,
            roaming_dir,
            home_dir,
        })
    }

    fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    fn data_dir(&self) -> &Path {
        &self.roaming_dir
    }

    fn cache_dir(&self) -> &Path {
        &self.local_dir
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

        let data_dir = dirs.data_dir().join(&prefix);
        let cache_dir = dirs.cache_dir().join(&prefix).join("cache");

        let user_dirs = Self {
            config_dir: data_dir.join("config"),
            temporary_dir: cache_dir.join("tmp"),
            log_dir: data_dir.join("log"),
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
pub fn roaming_dir() -> Result<&'static Path, Error> {
    data_dir()
}

#[inline]
pub fn local_dir() -> Result<&'static Path, Error> {
    cache_dir()
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

#[cfg(all(windows, test))]
mod tests {
    use super::*;

    #[test]
    fn basic_app() {
        let user = whoami::username();
        let expected = PathBuf::from(format!(r"C:\Users\{}\", &user));

        assert_eq!(
            app_temporary_dir("Special Company/Bad App").unwrap(),
            expected.join("AppData/Local/Special Company/Bad App/cache/tmp")
        )
    }

    #[test]
    fn basic_app2() {
        let user = whoami::username();
        let expected = PathBuf::from(format!(r"C:\Users\{}\", &user));

        assert_eq!(
            app_data_dir("Special Company/Bad App").unwrap(),
            expected.join("AppData/Roaming/Special Company/Bad App/")
        )
    }

    #[test]
    fn iri_file() {
        let expected = PathBuf::from(r"C:\Program Files\Bad Idea");
        let iri = crate::file_path(r"C:\Program Files\Bad Idea").unwrap();

        assert_eq!(expected, super::iri::resolve(&iri).unwrap());
    }

    #[test]
    fn iri_unc_path() {
        let expected = PathBuf::from(r"C:\Program Files\Bad Idea");
        let iri = crate::file_path(r"\\?\C:\Program Files\Bad Idea").unwrap();

        assert_eq!(expected, super::iri::resolve(&iri).unwrap());
    }

    #[test]
    fn iri_container() {
        let expected = crate::ResolveError::InvalidScheme("container".into(), &["file"]);
        let iri =
            iref::IriBuf::new("container:/AppData/Local/Special%20Company/Bad%20App/log").unwrap();

        assert_eq!(
            std::mem::discriminant(&expected),
            std::mem::discriminant(&super::iri::resolve(&iri).unwrap_err())
        );
    }
}
