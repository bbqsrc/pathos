pub use crate::macos::user::*;

pub mod iri {
    use crate::{Error, ResolveError};
    use iref::IriBuf;
    use std::{convert::TryInto, path::PathBuf};

    #[inline]
    pub fn home_dir() -> IriBuf {
        IriBuf::new("container:/").unwrap()
    }

    #[inline]
    pub(super) fn library_dir<P: AsRef<str>>(x: &str, prefix: P) -> Result<IriBuf, Error> {
        let mut iri = IriBuf::new("container:/Library/").unwrap();

        for item in x.split("/") {
            iri.path_mut().push(item.try_into().unwrap());
        }

        for item in prefix.as_ref().split("/") {
            iri.path_mut().push(item.try_into().unwrap());
        }

        iri.path_mut().open();
        Ok(iri)
    }

    #[inline]
    pub fn app_config_dir<P: AsRef<str>>(prefix: P) -> Result<IriBuf, Error> {
        library_dir("Preferences", prefix)
    }

    #[inline]
    pub fn app_cache_dir<P: AsRef<str>>(prefix: P) -> Result<IriBuf, Error> {
        library_dir("Caches", prefix)
    }

    #[inline]
    pub fn app_log_dir<P: AsRef<str>>(prefix: P) -> Result<IriBuf, Error> {
        library_dir("Logs", prefix)
    }

    #[inline]
    pub fn app_temporary_dir<P: AsRef<str>>(prefix: P) -> Result<IriBuf, Error> {
       let mut path = app_cache_dir(prefix)?;
       path.path_mut().push("tmp".try_into().unwrap());
       path.path_mut().open();
       Ok(path)
    }

    pub fn resolve(iri: &iref::IriBuf) -> Result<PathBuf, ResolveError> {
        match iri.scheme().as_str() {
            "file" => crate::resolve_file_iri(iri),
            "container" => crate::resolve_container_iri(super::home_dir()?.to_path_buf(), iri),
            unhandled => Err(ResolveError::InvalidScheme(
                unhandled.to_string(),
                &["file", "container"],
            )),
        }
    }

    #[test]
    fn smoke_test() {
        println!("{:?}", app_temporary_dir("meow").unwrap());
    }
}
