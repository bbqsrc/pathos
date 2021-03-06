use std::{borrow::Cow, ffi::OsString, path::PathBuf};

use iref::IriBuf;
use os_str_bytes::OsStringBytes;
use percent_encoding::percent_decode_str;

use crate::path::absolute::AbsolutePathBuf;

pub trait IriBufExt {
    fn to_path_buf(&self) -> Result<PathBuf, Error>;
}

#[cfg(not(any(target_os = "android", target_os = "ios",)))]
impl IriBufExt for IriBuf {
    fn to_path_buf(&self) -> Result<PathBuf, Error> {
        Ok(resolve_file_iri(self)?.to_path_buf())
    }
}

#[cfg(any(target_os = "android", target_os = "ios",))]
impl IriBufExt for IriBuf {
    fn to_path_buf(&self) -> Result<PathBuf, Error> {
        match self.scheme().as_str() {
            "file" => Ok(resolve_file_iri(self)?.to_path_buf()),
            "container" => Ok(resolve_container_iri(container_path()?, self)?.to_path_buf()),
            unhandled => Err(Error::InvalidScheme(
                unhandled.to_string(),
                &["file", "container"],
            )),
        }
    }
}

#[cfg(target_os = "android")]
#[inline(always)]
fn container_path() -> Result<PathBuf, Error> {
    let p = crate::android::user::CONTAINER_PATH.get().ok_or_else(|| {
        Error::UnresolvableContainer("No path set for container; call `set_container_path`.".into())
    })?;
    Ok(p.to_path_buf())
}

#[cfg(target_os = "ios")]
#[inline(always)]
fn container_path() -> Result<PathBuf, Error> {
    Ok(crate::ios::user::home_dir()
        .map_err(|_| Error::UnresolvableContainer("Could not resolve home directory".into()))?
        .to_path_buf())
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not convert path component to UTF-8 representation.")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("Path must not contain parent or current directory components (e.g. `.` or `..`)")]
    InvalidComponent,

    #[error("Unsupported prefix.")]
    UnsupportedPrefix,

    #[error("Relative paths cannot be converted into file:// IRIs.")]
    NotAbsolute,

    #[error("Failed to parse input as an IRI.")]
    InvalidIri(iref::Error),

    #[error("IRI is empty")]
    EmptyIri,

    #[error("IRI does not have a supported scheme. Got: '{0}', Supported: {1:?}")]
    InvalidScheme(String, &'static [&'static str]),

    #[error("Could not resolve container to path: {0}")]
    UnresolvableContainer(String),
}

#[inline]
fn resolve_file_iri(iri: &IriBuf) -> Result<AbsolutePathBuf, Error> {
    if iri.scheme() != "file" {
        return Err(crate::iri::Error::InvalidScheme(
            iri.scheme().to_string(),
            &["file"],
        ));
    }

    if iri.path().first().is_some() {
        let mut segments = iri.path().into_iter().map(|segment| -> OsString {
            let bytes: Cow<'_, [u8]> = percent_decode_str(segment.as_str()).into();
            // This should never panic, and according to the documentation,
            // panicking here is the correct behaviour if it _does_ break an invariant and fail.
            OsString::from_bytes(bytes)
                .expect("Invariant failed to be upheld: invalid OS string data")
        });

        let mut start = OsString::new();
        if !cfg!(unix) {
            start.push(segments.next().unwrap())
        }

        let sep = OsString::from(std::path::MAIN_SEPARATOR.to_string());

        let os_string = segments.fold(start, |mut acc: OsString, cur: OsString| {
            acc.push(&sep);
            acc.push(cur);
            acc
        });

        if !std::path::Path::new(&os_string).is_absolute() {
            return Err(crate::iri::Error::NotAbsolute);
        }

        Ok(crate::path::absolute::AbsolutePathBuf(os_string))
    } else {
        Err(crate::iri::Error::NotAbsolute)
    }
}

#[inline]
#[cfg(any(target_os = "android", target_os = "ios"))]
fn resolve_container_iri(prefix: PathBuf, iri: &IriBuf) -> Result<AbsolutePathBuf, Error> {
    let segments = iri.path().into_iter().map(|segment| -> OsString {
        let bytes: Cow<'_, [u8]> = percent_decode_str(segment.as_str()).into();
        // This should never panic, and according to the documentation,
        // panicking here is the correct behaviour if it _does_ break an invariant and fail.
        OsString::from_bytes(bytes).expect("Invariant failed to be upheld: invalid OS string data")
    });

    let sep = OsString::from(std::path::MAIN_SEPARATOR.to_string());

    let os_string = segments.fold(
        prefix.into_os_string(),
        |mut acc: OsString, cur: OsString| {
            acc.push(&sep);
            acc.push(cur);
            acc
        },
    );

    // let os_string = segments
    //     .fold(prefix, |mut acc: PathBuf, cur: OsString| {
    //         acc.push(cur);
    //         acc
    //     });

    if !std::path::Path::new(&os_string).is_absolute() {
        return Err(Error::NotAbsolute);
    }

    Ok(AbsolutePathBuf(os_string))
}

#[cfg(test)]
mod tests {
    use crate::{iri::IriBufExt, path::absolute::AbsolutePathBufExt};
    use iref::IriBuf;
    use std::path::PathBuf;

    #[test]
    #[cfg(unix)]
    fn file_iri() {
        let iri = IriBuf::new("file:///Library/Caches/Pahkat").unwrap();
        let value = iri.to_path_buf().unwrap();
        println!("{:?}", value);
        assert_eq!(value, PathBuf::from("/Library/Caches/Pahkat"))
    }

    #[test]
    #[cfg(windows)]
    fn file_iri() {
        let iri = IriBuf::new("file:///C:/ProgramData/Pahkat/cache").unwrap();
        let value = iri.to_path_buf().unwrap();
        println!("{:?}", value);
        assert_eq!(value, PathBuf::from(r"C:\ProgramData\Pahkat\cache"))
    }

    #[test]
    fn iri_from_path() {
        let path = PathBuf::from("///////Library/Caches/Pahkat").to_absolute_path_buf();
        println!("{:?}", path);
        #[cfg(windows)]
        assert!(path.is_err());
        #[cfg(unix)]
        assert_eq!(
            path.unwrap().to_path_buf(),
            PathBuf::from("/Library/Caches/Pahkat")
        );

        let path = PathBuf::from(r"C:\ProgramData\Pahkat\logs").to_absolute_path_buf();
        #[cfg(windows)]
        assert!(path.is_ok());
        #[cfg(unix)]
        assert!(path.is_err());

        println!("{:?}", path);
    }
}
