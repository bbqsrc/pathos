use once_cell::sync::{OnceCell, Lazy};
use std::path::PathBuf;

static CONTAINER_PATH: OnceCell<PathBuf> = OnceCell::new();

#[no_mangle]
extern "C" fn pathos_set_container_path() {
    
    let _ = CONTAINER_PATH.set(container_path).ok();
}