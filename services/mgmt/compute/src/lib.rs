#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2022-03-02")]
pub mod package_2022_03_02;
#[cfg(all(feature = "package-2022-03-02", not(feature = "no-default-tag")))]
pub use package_2022_03_02::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2022-03-01")]
pub mod package_2022_03_01;
#[cfg(all(feature = "package-2022-03-01", not(feature = "no-default-tag")))]
pub use package_2022_03_01::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2022-01-03")]
pub mod package_2022_01_03;
#[cfg(all(feature = "package-2022-01-03", not(feature = "no-default-tag")))]
pub use package_2022_01_03::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-12-01")]
pub mod package_2021_12_01;
#[cfg(all(feature = "package-2021-12-01", not(feature = "no-default-tag")))]
pub use package_2021_12_01::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-11-01")]
pub mod package_2021_11_01;
#[cfg(all(feature = "package-2021-11-01", not(feature = "no-default-tag")))]
pub use package_2021_11_01::{models, operations, operations::Client, operations::ClientBuilder};
