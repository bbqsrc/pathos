#![deny(rust_2018_idioms)]

macro_rules! dir {
    (| $x:ident | $($input:tt)*) => {
        DIRS.as_ref()
            .map(|$x| $($input)*)
            .map_err(|e| e.clone())
    }
}

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

use iref::IriBuf;
use os_str_bytes::{OsStrBytes, OsStringBytes};
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    iter::once,
    path::{Component, Path, PathBuf, Prefix},
};

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("IRI does not have a supported scheme. Got: '{0}', Supported: {1:?}")]
    InvalidScheme(String, &'static [&'static str]),

    #[error("IRI is empty")]
    EmptyIri,

    #[error("Failed to convert path to IRI.")]
    ConversionFailed(#[from] IriError),

    #[error("Error")]
    Error(#[from] Error),
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum IriError {
    #[error("Could not convert path component to UTF-8 representation.")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("Path must not contain parent or current directory components (e.g. `.` or `..`)")]
    InvalidComponent,

    #[error("Unsupported prefix.")]
    UnsupportedPrefix,

    #[error("Failed to parse input as an IRI.")]
    InvalidIri(iref::Error),
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("No home directory found!")]
    NoHomeDirectory,

    #[error("Failed to create directory for path: '{}'", .1.display())]
    CreateDirectoryFailed(#[source] eieio::Error, PathBuf),

    #[error("Failed to convert path to IRI")]
    IriConversionFailed(#[from] IriError),
}

#[inline(always)]
fn os_str_to_cow_str<'a>(os_str: &'a OsStr) -> Cow<'a, str> {
    match os_str.to_str() {
        Some(v) => Cow::Borrowed(v),
        None => {
            let bytes = os_str.to_bytes();
            let mut iter = percent_encode(&bytes, NON_ALPHANUMERIC);
            match iter.next() {
                None => "".into(),
                Some(first) => match iter.next() {
                    None => unreachable!(),
                    Some(second) => {
                        let mut string = first.to_owned();
                        string.push_str(second);
                        string.extend(iter);
                        string.into()
                    }
                },
            }
        }
    }
}

#[inline]
fn resolve_file_iri(iri: &IriBuf) -> Result<PathBuf, ResolveError> {
    if iri.path().first().is_some() {
        let mut segments = iri.path().into_iter().map(|segment| -> OsString {
            let bytes: Cow<'_, [u8]> = percent_decode_str(segment.as_str()).into();
            // This should never panic, and according to the documentation,
            // panicking here is the correct behaviour if it _does_ break an invariant and fail.
            OsString::from_bytes(bytes)
                .expect("Invariant failed to be upheld: invalid OS string data")
        });
        let start = segments.next().unwrap();
        Ok(segments
            .fold(start, |mut acc: OsString, cur: OsString| {
                acc.push(cur);
                acc
            })
            .into())
    } else {
        Err(ResolveError::EmptyIri)
    }
}

#[inline]
#[cfg(any(
    feature = "android",
    target_os = "android",
    feature = "ios",
    target_os = "ios",
))]
fn resolve_container_iri(prefix: PathBuf, iri: &IriBuf) -> Result<PathBuf, ResolveError> {
    let segments = iri.path().into_iter().map(|segment| -> OsString {
        let bytes: Cow<'_, [u8]> = percent_decode_str(segment.as_str()).into();
        // This should never panic, and according to the documentation,
        // panicking here is the correct behaviour if it _does_ break an invariant and fail.
        OsString::from_bytes(bytes).expect("Invariant failed to be upheld: invalid OS string data")
    });
    Ok(segments
        .fold(prefix, |mut acc: PathBuf, cur: OsString| {
            acc.push(cur);
            acc
        })
        .into())
}

pub fn file_path<P: AsRef<Path>>(path: P) -> Result<IriBuf, IriError> {
    let input = once(Ok(Cow::Borrowed("file:///")))
        .chain(path.as_ref().components().map(|c| {
            Ok(match c {
                Component::Prefix(prefix) => match prefix.kind() {
                    Prefix::Verbatim(verbatim) => os_str_to_cow_str(verbatim),
                    Prefix::VerbatimUNC(server, share) => Cow::Owned(format!(
                        "{}/{}",
                        os_str_to_cow_str(server),
                        os_str_to_cow_str(share)
                    )),
                    Prefix::VerbatimDisk(disk) => {
                        Cow::Owned(unsafe { std::str::from_utf8_unchecked(&[disk]) }.to_string())
                    }
                    Prefix::DeviceNS(_) => return Err(IriError::UnsupportedPrefix),
                    Prefix::UNC(server, share) => Cow::Owned(format!(
                        "{}/{}",
                        os_str_to_cow_str(server),
                        os_str_to_cow_str(share)
                    )),
                    Prefix::Disk(disk) => {
                        Cow::Owned(unsafe { std::str::from_utf8_unchecked(&[disk]) }.to_string())
                    }
                },
                Component::RootDir => Cow::Borrowed(""),
                Component::CurDir => return Err(IriError::InvalidComponent),
                Component::ParentDir => return Err(IriError::InvalidComponent),
                Component::Normal(value) => os_str_to_cow_str(value),
            })
        }))
        .collect::<Result<Vec<_>, _>>()?
        .join("/");
    IriBuf::new(&input).map_err(IriError::InvalidIri)
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
