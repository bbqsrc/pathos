//! [AppKit](https://developer.apple.com/documentation/appkit) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`app_kit`**
//! [feature flag](../index.html#feature-flags).

// `mac_catalyst` is enabled by `build.rs` for `x86_64-apple-ios-macabi`.
#![cfg(all(feature = "app_kit", any(target_os = "macos", mac_catalyst)))]

mod version;

pub use version::*;

#[doc(inline)]
pub use crate::common::NSDirectionalEdgeInsets;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}
