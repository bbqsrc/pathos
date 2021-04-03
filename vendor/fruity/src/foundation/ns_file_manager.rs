#![allow(missing_docs)]

use super::{NSSearchPathDirectory, NSSearchPathDomainMask};
use crate::core::Arc;
use crate::foundation::ns_string::NSString;
use crate::objc::{ClassType, NSObject, BOOL};

objc_subclass! {
    /// A convenient interface to the contents of the file system, and the
    /// primary means of interacting with it.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsfilemanager).
    pub class NSFileManager<'data>: NSObject<'data>;
}

objc_subclass! {
    /// URL
    pub class NSURL<'data>: NSObject<'data>;
}

impl NSURL<'_> {
    #[inline]
    pub fn absolute_string(&self) -> &NSString<'_> {
        unsafe { _msg_send_any![self, absoluteString] }
    }

    #[inline]
    pub fn path(&self) -> &NSString<'_> {
        unsafe { _msg_send_any![self, path] }
    }
}

impl NSFileManager<'_> {
    #[inline]
    #[doc(alias = "defaultManager")]
    pub fn default_manager() -> Arc<Self> {
        unsafe { Arc::retain(_msg_send_any![Self::class(), defaultManager]) }
    }

    #[inline]
    #[doc(alias = "URLForDirectory:inDomain:appropriateForURL:create:error:")]
    pub fn url_for_directory(
        &self,
        directory: NSSearchPathDirectory,
        domain: NSSearchPathDomainMask,
        _appropriate_for_url: Option<&NSURL>,
        _create: bool,
    ) -> Result<&NSURL<'_>, ()> {
        Ok(unsafe {
            _msg_send_any![self,
                URLForDirectory: directory
                inDomain: domain
                appropriateForURL: None::<&NSURL>
                create: BOOL::NO
                error: None::<&NSObject<'_>>]
        })
    }

    #[inline]
    #[doc(alias = "currentDirectoryPath")]
    pub fn current_directory_path(&self) -> Arc<NSString<'_>> {
        unsafe { _msg_send_any![self, currentDirectoryPath] }
    }

    #[inline]
    #[doc(alias = "temporaryDirectory")]
    pub fn temporary_directory(&self) -> &NSURL<'_> {
        unsafe { _msg_send_any![self, temporaryDirectory] }
    }

    #[inline]
    #[doc(alias = "homeDirectoryForCurrentUser")]
    pub fn home_directory_for_current_user(&self) -> &NSURL<'_> {
        unsafe { _msg_send_any![self, homeDirectoryForCurrentUser] }
    }
}

#[cfg(test)]
mod tests {
    use crate::foundation::ns_path_utilities::*;

    #[test]
    fn default_manager() {
        let mgr = super::NSFileManager::default_manager();
        println!("{}", mgr.hash());
        let cdp = mgr.temporary_directory();
        println!("{}", cdp);

        let ahhh = mgr
            .url_for_directory(
                NSDeveloperDirectory,
                NSUserDomainMask,
                None,
                false,
            )
            .unwrap();
        println!("{}", ahhh.path());
    }
}
