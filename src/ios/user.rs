pub use crate::macos::user::*;

pub mod iri {
    use std::convert::TryInto;
    use iref::IriBuf;

    #[inline]
    pub fn home_dir() -> IriBuf {
        IriBuf::new("container:/").unwrap()
    }

    #[inline]
    fn library_dir<P: AsRef<str>>(x: &str, prefix: P) -> IriBuf {
        let mut iri = IriBuf::new("container:/Library/").unwrap();
        iri.path_mut().push(x.try_into().unwrap());
        iri.path_mut().push(prefix.as_ref().try_into().unwrap());
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

    pub fn resolve(iri: &iref::IriBuf) -> Result<std::path::PathBuf, crate::ResolveError> {
        match iri.scheme().as_str() {
            "file" => Ok(std::path::PathBuf::from(iri.path().into_str())),
            "container" => {
                Ok(super::home_dir().join(iri.path().into_str()))
            }
            unhandled => {
                Err(crate::ResolveError::InvalidScheme(unhandled.to_string()))
            }
        }
    }
}