use super::Class;
use crate::core::Arc;
use std::{cell::UnsafeCell, fmt, marker::PhantomData, panic::RefUnwindSafe, ptr::NonNull};

/// An automatically-reference-counted pointer to a type-erased Objective-C
/// object.
///
/// This is semantically equivalent to `id _Nonnull` in ARC-ed Objective-C.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/id).
#[allow(non_camel_case_types)]
pub type id<'data> = Arc<ObjCObject<'data>>;

/// A type-erased instance of any Objective-C class.
///
/// This is designed to be used behind a reference or smart pointer like
/// [`Arc`](../obj/struct.Arc.html). In the future, this will be defined as an
/// [`extern type`](https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md).
///
/// All Objective-C class types within this crate ultimately
/// [`Deref`](std::ops::Deref) to this type.
///
/// This is equivalent to [`objc_object`](https://developer.apple.com/documentation/objectivec/objc_object?language=objc).
///
/// # Distinction from `NSObject`
///
/// `NSObject` is the root of _almost_ all Objective-C classes. Although very
/// rare, it is possible for other root classes to exist, such as `NSProxy`.
#[repr(C)]
pub struct ObjCObject<'data> {
    // TODO: Figure out the correct lifetime variance for `'data`.
    _marker: PhantomData<&'data ()>,
    // Stores data that may be mutated behind a shared reference. Internal
    // mutability triggers undefined behavior without `UnsafeCell`.
    _data: UnsafeCell<[u8; 0]>,
}

impl crate::core::ObjectType for ObjCObject<'_> {
    #[inline]
    #[doc(alias = "objc_retain")]
    fn retain(obj: &Self) -> Arc<Self> {
        extern "C" {
            fn objc_retain<'data>(obj: &ObjCObject<'data>) -> Arc<ObjCObject<'data>>;
        }
        unsafe { objc_retain(obj) }
    }

    #[inline]
    #[doc(alias = "objc_release")]
    unsafe fn release(obj: NonNull<Self>) {
        extern "C" {
            fn objc_release(obj: NonNull<ObjCObject<'_>>);
        }
        objc_release(obj);
    }
}

impl<'data> super::ObjectType<'data> for ObjCObject<'data> {
    #[inline]
    fn class<'s>(&'s self) -> &'s Class
    where
        'data: 's,
    {
        // TODO: Call `_objc_opt_class` on:
        // - macOS 10.15+
        // - iOS (unknown)
        // - tvOS (unknown)
        // - watchOS (unknown)
        unsafe { _msg_send_strict_cached![self, class] }
    }
}

impl<'data> AsRef<ObjCObject<'data>> for ObjCObject<'data> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

// This type is used globally, so we must be able to share it across threads.
unsafe impl Sync for ObjCObject<'_> {}
unsafe impl Send for ObjCObject<'_> {}

// Although this uses `UnsafeCell`, it does not point to any Rust types.
impl RefUnwindSafe for ObjCObject<'_> {}

impl fmt::Debug for ObjCObject<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as *const Self).fmt(f)
    }
}
