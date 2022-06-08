#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-preview-2021-09-30")]
pub mod package_preview_2021_09_30;
#[cfg(all(feature = "package-preview-2021-09-30", not(feature = "no-default-tag")))]
pub use package_preview_2021_09_30::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2018-11-30")]
pub mod package_2018_11_30;
#[cfg(all(feature = "package-2018-11-30", not(feature = "no-default-tag")))]
pub use package_2018_11_30::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2015-08-31-preview")]
pub mod package_2015_08_31_preview;
#[cfg(all(feature = "package-2015-08-31-preview", not(feature = "no-default-tag")))]
pub use package_2015_08_31_preview::{models, operations, operations::Client, operations::ClientBuilder};
