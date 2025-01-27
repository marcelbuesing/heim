//! MacOS specific extensions for crate types.

use heim_common::units::Information;

/// MacOS-specific extension to [`Memory`]
#[heim_derive::os_ext_for(crate::Memory, cfg(target_os = "macos"))]
pub trait MemoryExt {
    /// Returns memory currently in use or very recently used, and so it is in RAM.
    fn active(&self) -> Information;

    /// Returns memory that is marked as not used.
    fn inactive(&self) -> Information;

    /// Returns memory that is marked to always stay in RAM. It is never moved to disk.
    fn wire(&self) -> Information;
}
