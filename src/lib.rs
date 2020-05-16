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

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("IRI does not have `container` or `file` scheme. Got: {0}")]
    InvalidScheme(String),

    #[error("IRI is empty")]
    EmptyIri,

    #[error("{0}")]
    PlatformSpecific(String),
}

pub(crate) fn file_path<P: AsRef<std::path::Path>>(path: P) -> iref::IriBuf {
    let path = path
        .as_ref()
        .to_string_lossy()
        .replace(r"\", "/")
        .replace(" ", "%20");
    iref::IriBuf::new(&format!("file:///{}", path)).expect("invalid IRI")
}
