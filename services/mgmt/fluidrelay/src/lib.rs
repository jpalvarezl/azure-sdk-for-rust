#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2022-05-11")]
pub mod package_2022_05_11;
#[cfg(all(feature = "package-2022-05-11", not(feature = "no-default-tag")))]
pub use package_2022_05_11::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2022-04-21")]
pub mod package_2022_04_21;
#[cfg(all(feature = "package-2022-04-21", not(feature = "no-default-tag")))]
pub use package_2022_04_21::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2022-02-15")]
pub mod package_2022_02_15;
#[cfg(all(feature = "package-2022-02-15", not(feature = "no-default-tag")))]
pub use package_2022_02_15::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-09-10-preview")]
pub mod package_2021_09_10_preview;
#[cfg(all(feature = "package-2021-09-10-preview", not(feature = "no-default-tag")))]
pub use package_2021_09_10_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-08-30-preview")]
pub mod package_2021_08_30_preview;
#[cfg(all(feature = "package-2021-08-30-preview", not(feature = "no-default-tag")))]
pub use package_2021_08_30_preview::{models, operations, operations::Client, operations::ClientBuilder};
