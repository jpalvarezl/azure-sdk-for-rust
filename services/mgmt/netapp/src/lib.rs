#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-netapp-2022-11-01")]
pub mod package_netapp_2022_11_01;
#[cfg(all(feature = "package-netapp-2022-11-01", not(feature = "no-default-tag")))]
pub use package_netapp_2022_11_01::*;
#[cfg(feature = "package-netapp-2022-09-01")]
pub mod package_netapp_2022_09_01;
#[cfg(all(feature = "package-netapp-2022-09-01", not(feature = "no-default-tag")))]
pub use package_netapp_2022_09_01::*;
#[cfg(feature = "package-netapp-2022-05-01")]
pub mod package_netapp_2022_05_01;
#[cfg(all(feature = "package-netapp-2022-05-01", not(feature = "no-default-tag")))]
pub use package_netapp_2022_05_01::*;
#[cfg(feature = "package-netapp-2022-03-01")]
pub mod package_netapp_2022_03_01;
#[cfg(all(feature = "package-netapp-2022-03-01", not(feature = "no-default-tag")))]
pub use package_netapp_2022_03_01::*;
#[cfg(feature = "package-netapp-2022-01-01")]
pub mod package_netapp_2022_01_01;
#[cfg(all(feature = "package-netapp-2022-01-01", not(feature = "no-default-tag")))]
pub use package_netapp_2022_01_01::*;
