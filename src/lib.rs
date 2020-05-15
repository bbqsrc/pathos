#![deny(rust_2018_idioms)]

#[cfg(any(feature = "android", target_os = "android"))]
pub mod android;
#[cfg(any(feature = "ios", target_os = "ios"))]
pub mod ios;
#[cfg(any(feature = "linux", target_os = "linux"))]
pub mod linux;
#[cfg(any(feature = "macos", target_os = "macos"))]
pub mod macos;
#[cfg(any(feature = "windows", target_os = "windows"))]
pub mod windows;
pub mod xdg;

#[cfg(windows)]
pub use windows::system as system;
#[cfg(target_os = "macos")]
pub use macos::system as system;
#[cfg(target_os = "linux")]
pub use linux::system as system;
#[cfg(target_os = "ios")]
pub use ios::system as system;
#[cfg(target_os = "android")]
pub use android::system as system;

#[cfg(windows)]
pub use windows::user as user;
#[cfg(target_os = "macos")]
pub use macos::user as user;
#[cfg(target_os = "linux")]
pub use linux::user as user;
#[cfg(target_os = "ios")]
pub use ios::user as user;
#[cfg(target_os = "android")]
pub use android::user as user;

use std::convert::TryInto;

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("IRI does not have `container` or `file` scheme. Got: {0}")]
    InvalidScheme(String),
}

pub(crate) fn file_path<P: AsRef<str>>(path: P) -> iref::IriBuf {
    let mut iri = iref::IriBuf::new("file:///").unwrap();
    iri.set_path(path.as_ref().try_into().unwrap());
    iri
}
