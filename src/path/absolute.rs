use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    ffi::{OsStr, OsString},
    iter::once,
    ops::Deref,
};

use iref::IriBuf;
use os_str_bytes::OsStrBytes;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

#[derive(Debug, Clone)]
pub enum TryFromError {
    NotAbsolute,
    ContainsRelComponents,
}

impl std::fmt::Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryFromError::NotAbsolute => f.write_str("path not absolute"),
            TryFromError::ContainsRelComponents => {
                f.write_str("contains relative components (i.e. '..' or '.')")
            }
        }
    }
}

impl std::error::Error for TryFromError {}

#[repr(transparent)]
#[derive(Debug)]
pub struct AbsolutePath(OsStr);

impl<'a> From<&'a AbsolutePath> for &'a std::path::Path {
    fn from(abs_path: &'a AbsolutePath) -> Self {
        &std::path::Path::new(&abs_path.0)
    }
}

impl<'a> TryFrom<&'a std::path::Path> for &'a AbsolutePath {
    type Error = TryFromError;

    fn try_from(value: &'a std::path::Path) -> Result<Self, Self::Error> {
        use std::path::Component;

        if !value.is_absolute() {
            return Err(TryFromError::NotAbsolute);
        }

        let has_rel_components = value.components().any(|x| match x {
            Component::CurDir | Component::ParentDir => true,
            _ => false,
        });

        if has_rel_components {
            return Err(TryFromError::ContainsRelComponents);
        }

        Ok(AbsolutePath::new_unchecked(value))
    }
}

impl AbsolutePath {
    #[inline]
    fn new_unchecked<S: AsRef<OsStr> + ?Sized>(s: &S) -> &AbsolutePath {
        unsafe { &*(s.as_ref() as *const OsStr as *const AbsolutePath) }
    }

    pub fn to_path(&self) -> &std::path::Path {
        &std::path::Path::new(&self.0)
    }

    pub fn to_path_buf(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.0)
    }

    pub fn to_absolute_path_buf(&self) -> AbsolutePathBuf {
        AbsolutePathBuf(self.0.to_os_string())
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AbsolutePathBuf(pub(crate) OsString);

impl TryFrom<std::path::PathBuf> for AbsolutePathBuf {
    type Error = TryFromError;

    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        use std::path::Component;

        if !value.is_absolute() {
            return Err(TryFromError::NotAbsolute);
        }

        let has_rel_components = value.components().any(|x| match x {
            Component::CurDir | Component::ParentDir => true,
            _ => false,
        });

        if has_rel_components {
            return Err(TryFromError::ContainsRelComponents);
        }

        Ok(AbsolutePathBuf(value.into_os_string()))
    }
}

impl From<AbsolutePathBuf> for std::path::PathBuf {
    fn from(path: AbsolutePathBuf) -> Self {
        std::path::PathBuf::from(path.0)
    }
}

impl Deref for AbsolutePathBuf {
    type Target = AbsolutePath;

    fn deref(&self) -> &Self::Target {
        AbsolutePath::new_unchecked(&self.0)
    }
}

impl AbsolutePathBuf {
    pub fn to_absolute_path(&self) -> &AbsolutePath {
        &*self
    }

    pub fn to_path(&self) -> &std::path::Path {
        &std::path::Path::new(&self.0)
    }

    pub fn to_path_buf(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.0)
    }

    pub fn to_file_iri(&self) -> Result<IriBuf, crate::iri::Error> {
        file_path(self.to_path())
    }
}

#[inline(always)]
fn os_str_to_cow_str<'a>(os_str: &'a OsStr) -> Cow<'a, str> {
    let bytes = os_str.to_bytes();
    let iter = percent_encode(&bytes, NON_ALPHANUMERIC);
    // TODO: optimise this again, was buggy before and so we only do Owned for now.
    Cow::Owned(iter.to_string())
}

fn file_path<P: AsRef<std::path::Path>>(path: P) -> Result<IriBuf, crate::iri::Error> {
    use std::path::{Component, Prefix};

    if !path.as_ref().is_absolute() {
        return Err(crate::iri::Error::NotAbsolute);
    }
    let input = once(Ok(Cow::Borrowed("file:/")))
        .chain(path.as_ref().components().filter_map(|c| {
            Some(Ok(match c {
                Component::Prefix(prefix) => match prefix.kind() {
                    Prefix::Verbatim(verbatim) => os_str_to_cow_str(verbatim),
                    Prefix::DeviceNS(_) => return Some(Err(crate::iri::Error::UnsupportedPrefix)),
                    Prefix::UNC(server, share) | Prefix::VerbatimUNC(server, share) => Cow::Owned(
                        format!("{}/{}", os_str_to_cow_str(server), os_str_to_cow_str(share)),
                    ),
                    Prefix::Disk(disk) | Prefix::VerbatimDisk(disk) => {
                        Cow::Owned(format!("/{}:", unsafe {
                            std::str::from_utf8_unchecked(&[disk])
                        }))
                    }
                },
                #[cfg(windows)]
                Component::RootDir => return None,
                #[cfg(unix)]
                Component::RootDir => Cow::Borrowed(""),
                Component::CurDir => return Some(Err(crate::iri::Error::InvalidComponent)),
                Component::ParentDir => return Some(Err(crate::iri::Error::InvalidComponent)),
                Component::Normal(value) => os_str_to_cow_str(value),
            }))
        }))
        .collect::<Result<Vec<_>, _>>()?
        .join("/");

    IriBuf::new(&input).map_err(crate::iri::Error::InvalidIri)
}

pub trait AbsolutePathExt {
    fn to_absolute_path(&self) -> Result<&AbsolutePath, TryFromError>;
}

impl AbsolutePathExt for std::path::Path {
    fn to_absolute_path(&self) -> Result<&AbsolutePath, TryFromError> {
        self.try_into()
    }
}

pub trait AbsolutePathBufExt {
    fn to_absolute_path_buf(&self) -> Result<AbsolutePathBuf, TryFromError>;
}

impl AbsolutePathBufExt for std::path::PathBuf {
    fn to_absolute_path_buf(&self) -> Result<AbsolutePathBuf, TryFromError> {
        self.clone().try_into()
    }
}
