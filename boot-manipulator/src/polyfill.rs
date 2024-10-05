//! Polyfills for various APIs that should be supported.

use core::marker::PhantomData;

/// A marker type which does not implement [`Send`].
pub struct PhantomNotSend(PhantomData<*mut ()>);

// SAFETY:
// This is a marker trait and so it is safe to implement `Sync`.
unsafe impl Sync for PhantomNotSend {}

/// A marker type which does not implement [`Sync`].
pub struct PhatomNotSync(PhantomData<*mut ()>);

// SAFETY:
// This is a marker trait and so it is safe to implement `Send`.
unsafe impl Send for PhatomNotSync {}
