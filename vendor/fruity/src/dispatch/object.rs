use super::{sys, DispatchQueue};
use crate::core::{Arc, ObjectType};
use std::{
    cell::UnsafeCell,
    ffi::c_void,
    panic::RefUnwindSafe,
    ptr::{self, NonNull},
};

/// The base type for dispatch objects.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/dispatch/dispatchobject) |
/// [Objective-C](https://developer.apple.com/documentation/dispatch/&DispatchObject)
#[repr(C)]
pub struct DispatchObject {
    // Stores data that may be mutated behind a shared reference. Internal
    // mutability triggers undefined behavior without `UnsafeCell`.
    _data: UnsafeCell<[u8; 0]>,
}

impl ObjectType for DispatchObject {
    #[inline]
    #[doc(alias = "dispatch_retain")]
    fn retain(obj: &Self) -> Arc<Self> {
        unsafe {
            sys::dispatch_retain(obj);
            Arc::from_raw(obj)
        }
    }

    #[inline]
    #[doc(alias = "dispatch_release")]
    unsafe fn release(obj: NonNull<Self>) {
        sys::dispatch_release(obj.as_ptr());
    }
}

unsafe impl Send for DispatchObject {}
unsafe impl Sync for DispatchObject {}

// Although this uses `UnsafeCell`, it does not point to any Rust types.
impl RefUnwindSafe for DispatchObject {}

impl AsRef<DispatchObject> for DispatchObject {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Dispatch operations.
impl DispatchObject {
    /// Activates `self`.
    ///
    /// Once a dispatch object has been activated, it cannot change its target
    /// queue.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1641002-dispatch_activate).
    #[inline]
    #[doc(alias = "dispatch_activate")]
    pub fn activate(&self) {
        unsafe { sys::dispatch_activate(self) };
    }

    /// Resumes the invocation of block objects on `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452929-dispatch_resume).
    #[inline]
    #[doc(alias = "dispatch_resume")]
    pub fn resume(&self) {
        unsafe { sys::dispatch_resume(self) };
    }

    /// Suspends the invocation of block objects on `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452801-dispatch_suspend).
    #[inline]
    #[doc(alias = "dispatch_suspend")]
    pub fn suspend(&self) {
        unsafe { sys::dispatch_suspend(self) };
    }

    /// Specifies the dispatch queue on which to perform work associated with
    /// `self`.
    ///
    /// If this queue has already been activated, then this method will fail.
    ///
    /// **Important:** When setting up target queues, it is a programmer error
    /// to create cycles in the dispatch queue hierarchy. In other words, don't
    /// set the target of queue A to queue B and set the target of queue B to
    /// queue A.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchobject/1452989-settarget) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452989-dispatch_set_target_queue)
    ///
    /// # Safety
    ///
    /// The target queue must be expected for this dispatch object. For example,
    /// an object may expect to dispatch to the main queue.
    #[inline]
    #[doc(alias = "dispatch_set_target_queue")]
    pub unsafe fn set_target<Q>(&self, queue: Option<&DispatchQueue>) {
        let queue = match queue {
            Some(queue) => queue,
            None => ptr::null(),
        };
        sys::dispatch_set_target_queue(self, queue);
    }

    /// Returns the application-defined context of an object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1453005-dispatch_get_context).
    #[inline]
    #[doc(alias = "dispatch_get_context")]
    pub fn context(&self) -> *mut c_void {
        unsafe { sys::dispatch_get_context(self) }
    }

    /// Associates an application-defined context with the object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452807-dispatch_set_context).
    ///
    /// # Safety
    ///
    /// The object or dependent code may make assumptions about its current
    /// context. Callers must be careful to not invalidate any assumptions.
    #[inline]
    #[doc(alias = "dispatch_set_context")]
    pub unsafe fn set_context(&self, context: *mut c_void) {
        sys::dispatch_set_context(self, context);
    }
}
