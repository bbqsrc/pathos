use std::{
    convert::TryFrom,
    ffi::{OsStr, OsString},
    ops::Deref,
};

#[derive(Debug)]
pub enum TryFromError {
    NotAbsolute,
}

impl std::fmt::Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryFromError::NotAbsolute => f.write_str("path not absolute"),
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
        if !value.is_absolute() {
            Err(TryFromError::NotAbsolute)
        } else {
            Ok(AbsolutePath::new_unchecked(value))
        }
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
pub struct AbsolutePathBuf(OsString);

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
}
