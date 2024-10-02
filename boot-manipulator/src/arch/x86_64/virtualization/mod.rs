//! Definitions and interfaces to interact with virtualization.

use crate::arch::VirtualizationOps;

/// Multiplexer over the various virtualization implementations offered on the `x86_64`
/// architecture.
pub struct Multiplexer;

impl VirtualizationOps for Multiplexer {}
