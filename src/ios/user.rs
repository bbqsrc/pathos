// Re-export all macOS items as iOS items.
pub use crate::macos::user::*;

pub mod iri {
    use crate::Error;
    use iref::IriBuf;
    use std::convert::TryInto;

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke_test() {
        println!("{:?}", super::iri::app_temporary_dir("meow").unwrap());
    }
}
