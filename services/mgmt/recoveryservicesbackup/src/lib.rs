#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-passivestamp-2021-11-15")]
pub mod package_passivestamp_2021_11_15;
#[cfg(all(feature = "package-passivestamp-2021-11-15", not(feature = "no-default-tag")))]
pub use package_passivestamp_2021_11_15::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-passivestamp-2018-12-20")]
pub mod package_passivestamp_2018_12_20;
#[cfg(all(feature = "package-passivestamp-2018-12-20", not(feature = "no-default-tag")))]
pub use package_passivestamp_2018_12_20::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "no-default-tag")))]
pub use package_2022_03::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2022-02")]
pub mod package_2022_02;
#[cfg(all(feature = "package-2022-02", not(feature = "no-default-tag")))]
pub use package_2022_02::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2022-01")]
pub mod package_2022_01;
#[cfg(all(feature = "package-2022-01", not(feature = "no-default-tag")))]
pub use package_2022_01::{models, operations, operations::Client, operations::ClientBuilder};
