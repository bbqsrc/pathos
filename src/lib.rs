#![deny(rust_2018_idioms)]

macro_rules! dir {
    (| $x:ident | $($input:tt)*) => {
        DIRS.as_ref()
            .map(|$x| $($input)*)
            .map_err(|e| e.clone())
    }
}

pub mod iri;
pub mod path;

#[cfg(any(feature = "android", target_os = "android"))]
pub mod android;
#[cfg(any(feature = "ios", target_os = "ios"))]
pub mod ios;
#[cfg(any(feature = "linux", target_os = "linux"))]
pub mod linux;
#[cfg(any(feature = "macos", target_os = "macos", target_os = "ios"))]
pub mod macos;
#[cfg(any(feature = "windows", target_os = "windows"))]
pub mod windows;
#[cfg(any(unix, feature = "xdg"))]
pub mod xdg;

#[cfg(target_os = "android")]
pub use android::system;
#[cfg(target_os = "ios")]
pub use ios::system;
#[cfg(target_os = "linux")]
pub use linux::system;
#[cfg(target_os = "macos")]
pub use macos::system;
#[cfg(windows)]
pub use windows::system;

#[cfg(target_os = "android")]
pub use android::user;
#[cfg(target_os = "ios")]
pub use ios::user;
#[cfg(target_os = "linux")]
pub use linux::user;
#[cfg(target_os = "macos")]
pub use macos::user;
#[cfg(windows)]
pub use windows::user;

use std::path::{Path, PathBuf};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("No home directory found!")]
    NoHomeDirectory,

    #[error("Failed to create directory for path: '{}'", .1.display())]
    CreateDirectoryFailed(#[source] eieio::Error, PathBuf),

    #[error("Failed to convert path to IRI")]
    IriConversionFailed(#[from] crate::iri::Error),

    #[error("Path not absolute")]
    PathNotAbsolute(#[from] crate::path::absolute::TryFromError),
}

pub trait AppDirs: Sized {
    fn new<P>(prefix: P) -> Result<Self, Error>
    where
        P: Into<PathBuf>;
    fn create(&self) -> Result<(), Error>;
    fn data_dir(&self) -> &Path;
    fn config_dir(&self) -> &Path;
    fn cache_dir(&self) -> &Path;
    fn log_dir(&self) -> &Path;
    fn temporary_dir(&self) -> &Path;
}

pub trait UserDirs: Sized {
    fn new() -> Result<Self, Error>;
    fn home_dir(&self) -> &Path;
    fn data_dir(&self) -> &Path;
    fn cache_dir(&self) -> &Path;
}
