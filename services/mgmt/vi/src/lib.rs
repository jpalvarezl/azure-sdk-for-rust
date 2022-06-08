#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2022-04-13-preview")]
pub mod package_2022_04_13_preview;
#[cfg(all(feature = "package-2022-04-13-preview", not(feature = "no-default-tag")))]
pub use package_2022_04_13_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-11-10-preview")]
pub mod package_2021_11_10_preview;
#[cfg(all(feature = "package-2021-11-10-preview", not(feature = "no-default-tag")))]
pub use package_2021_11_10_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-10-27-preview")]
pub mod package_2021_10_27_preview;
#[cfg(all(feature = "package-2021-10-27-preview", not(feature = "no-default-tag")))]
pub use package_2021_10_27_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-10-18-preview")]
pub mod package_2021_10_18_preview;
#[cfg(all(feature = "package-2021-10-18-preview", not(feature = "no-default-tag")))]
pub use package_2021_10_18_preview::{models, operations, operations::Client, operations::ClientBuilder};
