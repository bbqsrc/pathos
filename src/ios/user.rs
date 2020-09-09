pub use crate::macos::user::*;

pub mod iri {
    use crate::ResolveError;
    use iref::IriBuf;
    use std::{convert::TryInto, path::PathBuf};

    #[inline]
    pub fn home_dir() -> IriBuf {
        IriBuf::new("container:/").unwrap()
    }

    #[inline]
    pub(super) fn library_dir<P: AsRef<str>>(x: &str, prefix: P) -> IriBuf {
        let mut iri = IriBuf::new("container:/Library/").unwrap();

        for item in x.split("/") {
            iri.path_mut().push(item.try_into().unwrap());
        }

        for item in prefix.as_ref().split("/") {
            iri.path_mut().push(item.try_into().unwrap());
        }

        iri.path_mut().open();
        iri
    }

    #[inline]
    pub fn app_config_dir<P: AsRef<str>>(prefix: P) -> IriBuf {
        library_dir("Preferences", prefix)
    }

    #[inline]
    pub fn app_cache_dir<P: AsRef<str>>(prefix: P) -> IriBuf {
        library_dir("Caches", prefix)
    }

    #[inline]
    pub fn app_log_dir<P: AsRef<str>>(prefix: P) -> IriBuf {
        library_dir("Logs", prefix)
    }

    #[inline]
    pub fn app_temporary_dir<P: AsRef<str>>(prefix: P) -> IriBuf {
        library_dir("Caches/tmp", prefix)
    }

    pub fn resolve(iri: &iref::IriBuf) -> Result<PathBuf, ResolveError> {
        match iri.scheme().as_str() {
            "file" => crate::resolve_file_iri(iri),
            "container" => crate::resolve_container_iri(super::home_dir()?, iri),
            unhandled => Err(ResolveError::InvalidScheme(
                unhandled.to_string(),
                &["file", "container"],
            )),
        }
    }
}
