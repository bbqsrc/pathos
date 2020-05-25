pub use crate::macos::user::*;

pub mod iri {
    use iref::IriBuf;
    use std::convert::TryInto;

    #[inline]
    pub fn home_dir() -> IriBuf {
        IriBuf::new("container:/").unwrap()
    }

    #[inline]
    pub(in super) fn library_dir<P: AsRef<str>>(x: &str, prefix: P) -> IriBuf {
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

    pub fn resolve(iri: &iref::IriBuf) -> Result<std::path::PathBuf, crate::ResolveError> {
        match iri.scheme().as_str() {
            "file" => Ok(std::path::PathBuf::from(iri.path().as_pct_str().decode())),
            "container" => {
                let mut path = iri.path().as_pct_str().decode();
                if path.starts_with("/") {
                    path = path.chars().skip(1).collect::<String>();
                }
                Ok(super::home_dir().join(path))
            }
            unhandled => Err(crate::ResolveError::InvalidScheme(unhandled.to_string())),
        }
    }
}
