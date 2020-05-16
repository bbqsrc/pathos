use once_cell::sync::{Lazy, OnceCell};
use std::path::PathBuf;

static CONTAINER_PATH: OnceCell<PathBuf> = OnceCell::new();

#[no_mangle]
extern "C" fn pathos_set_container_path(container_path: *const std::ffi::c_char) {
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
