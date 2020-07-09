use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

static BASE_DIRS: Lazy<directories::BaseDirs> = Lazy::new(|| directories::BaseDirs::new().unwrap());

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
    BASE_DIRS.data_dir().join(prefix.as_ref())
}

#[inline]
pub fn app_config_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_data_dir(prefix).join("config")
}

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    BASE_DIRS
        .data_local_dir()
        .join(prefix.as_ref())
        .join("cache")
}

#[inline]
pub fn app_log_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    BASE_DIRS.data_local_dir().join(prefix.as_ref()).join("log")
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    app_cache_dir(prefix).join("tmp")
}

#[inline]
pub fn local_dir() -> PathBuf {
    BASE_DIRS.data_local_dir().to_path_buf()
}

#[inline]
pub fn roaming_dir() -> PathBuf {
    BASE_DIRS.data_dir().to_path_buf()
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

    pub fn resolve(iri: &iref::IriBuf) -> Result<std::path::PathBuf, crate::ResolveError> {
        match iri.scheme().as_str() {
            "file" => {
                if let Some(first) = iri.path().first() {
                    if first.len() != 2
                        || !first.as_str().chars().next().unwrap().is_ascii_alphabetic()
                        || first.as_str().chars().nth(1).unwrap() != ':'
                    {
                        return Err(crate::ResolveError::PlatformSpecific(format!(
                            "Invalid drive letter. Got: {}",
                            first.as_str()
                        )));
                    }
                } else {
                    return Err(crate::ResolveError::EmptyIri);
                }
                Ok(std::path::PathBuf::from(
                    iri.path()
                        .as_pct_str()
                        .decode()
                        .chars()
                        .skip(1)
                        .collect::<String>(),
                ))
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_app() {
        let user = whoami::username();
        let expected = PathBuf::from(format!(r"C:\Users\{}\", &user));

        assert_eq!(
            app_temporary_dir("Special Company/Bad App"),
            expected.join("AppData/Local/Special Company/Bad App/cache/tmp")
        )
    }

    #[test]
    fn basic_app2() {
        let user = whoami::username();
        let expected = PathBuf::from(format!(r"C:\Users\{}\", &user));

        assert_eq!(
            app_data_dir("Special Company/Bad App"),
            expected.join("AppData/Roaming/Special Company/Bad App/")
        )
    }

    #[test]
    fn iri_file() {
        let expected = PathBuf::from(r"C:\Program Files\Bad Idea");
        let iri = crate::file_path(r"C:\Program Files\Bad Idea");

        assert_eq!(expected, super::iri::resolve(&iri).unwrap());
    }

    #[test]
    fn iri_container() {
        let user = whoami::username();
        let expected = PathBuf::from(format!(
            r"C:\Users\{}\AppData\Local\Special Company\Bad App\log",
            user
        ));
        let iri =
            iref::IriBuf::new("container:/AppData/Local/Special%20Company/Bad%20App/log").unwrap();

        assert_eq!(expected, super::iri::resolve(&iri).unwrap());
    }
}
