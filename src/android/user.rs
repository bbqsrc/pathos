use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use once_cell::sync::{Lazy, OnceCell};

// 31 |         pathos::user::iri::resolve(&self.0).ok()
// 56 |         pathos::user::iri::app_cache_dir(APP_PATH))
// 44 |     return pathos::user::app_log_dir(APP_PATH)

// 97 |     ConfigPath(pathos::user::iri::app_temporary_dir(APP_PATH))

#[inline]
pub fn app_cache_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    CONTAINER_PATH
        .get()
        .expect("no path set for container; call set_container_path.")
        .join("cache")
        .join(prefix)
}

#[inline]
pub fn app_temporary_dir<P: AsRef<Path>>(prefix: P) -> PathBuf {
    std::env::temp_dir()
}

pub mod iri {
    use super::CONTAINER_PATH;

    use iref::IriBuf;
    use std::convert::TryInto;

    #[inline]
    pub fn app_cache_dir<P: AsRef<str>>(prefix: P) -> IriBuf {
        let mut iri = IriBuf::new("container:/cache/").unwrap();
        iri.path_mut().push(prefix.as_ref().try_into().unwrap());
        iri.path_mut().open();
        iri
    }

    #[inline]
    pub fn app_temporary_dir<P: AsRef<str>>(prefix: P) -> IriBuf {
        crate::file_path(super::app_temporary_dir(prefix.as_ref()))
    }

    pub fn resolve(iri: &iref::IriBuf) -> Result<std::path::PathBuf, crate::ResolveError> {
        match iri.scheme().as_str() {
            "file" => Ok(std::path::PathBuf::from(iri.path().as_pct_str().decode())),
            "container" => {
                let mut path = iri.path().as_pct_str().decode();
                if path.starts_with("/") {
                    path = path.chars().skip(1).collect::<String>();
                }
                Ok(CONTAINER_PATH
                    .get()
                    .expect("no path set for container; call set_container_path.")
                    .join(path))
            }
            unhandled => Err(crate::ResolveError::InvalidScheme(unhandled.to_string())),
        }
    }
}

static CONTAINER_PATH: OnceCell<PathBuf> = OnceCell::new();

#[no_mangle]
extern "C" fn pathos_set_container_path(container_path: *const std::os::raw::c_char) {
    if (container_path.is_null()) {
        log::error!("Invalid `container_path` passed; no Android container set.");
        return;
    }

    let c_str = unsafe { std::ffi::CStr::from_ptr(container_path) };
    let os_str = std::ffi::OsStr::from_bytes(c_str.to_bytes());
    let path: &Path = os_str.as_ref();

    let _ = CONTAINER_PATH.set(path.to_path_buf()).ok();

    log::info!("Container path set to: '{}'", path.display());
}

pub fn set_container_path(path: PathBuf) {
    let _ = CONTAINER_PATH.set(path.to_path_buf()).ok();
    log::info!("Container path set to: '{}'", path.display());
}
