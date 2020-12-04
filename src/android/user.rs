use std::os::{raw::c_char, unix::ffi::OsStrExt};
use std::{
    ffi::{CStr, OsStr},
    path::{Path, PathBuf},
};

use once_cell::sync::OnceCell;

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
    std::env::temp_dir().join(prefix)
}

pub mod iri {
    use crate::{path::absolute::AbsolutePathBufExt, Error};
    use iref::IriBuf;
    use std::convert::TryInto;

    #[inline]
    pub fn app_cache_dir<P: AsRef<str>>(prefix: P) -> Result<IriBuf, Error> {
        let mut iri = IriBuf::new("container:/cache/").unwrap();
        for item in prefix.as_ref().split("/") {
            iri.path_mut().push(item.try_into().unwrap());
        }
        iri.path_mut().open();
        Ok(iri)
    }

    #[inline]
    pub fn app_temporary_dir<P: AsRef<str>>(prefix: P) -> Result<IriBuf, Error> {
        Ok(super::app_temporary_dir(prefix.as_ref())
            .to_absolute_path_buf()?
            .to_file_iri()?)
    }
}

pub(crate) static CONTAINER_PATH: OnceCell<PathBuf> = OnceCell::new();

#[no_mangle]
unsafe extern "C" fn pathos_set_container_path(container_path: *const c_char) {
    if container_path.is_null() {
        log::error!("Invalid `container_path` passed; no Android container set.");
        return;
    }

    let c_str = CStr::from_ptr(container_path);
    let os_str = OsStr::from_bytes(c_str.to_bytes());
    let path: &Path = os_str.as_ref();

    let _ = CONTAINER_PATH.set(path.to_path_buf()).ok();

    log::info!("Container path set to: '{}'", path.display());
}

pub fn set_container_path(path: PathBuf) {
    let _ = CONTAINER_PATH.set(path.to_path_buf()).ok();
    log::info!("Container path set to: '{}'", path.display());
}
