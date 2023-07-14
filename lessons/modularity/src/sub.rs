//! This module contains a private and a public sub modules.
//! This is a simple module level acting as inner documentation.

// make sub::private only visible to sibling modules
mod private;
// make sub::public visible as public
pub mod public;
// not adding sub::hidden on purpose to hide it from siblings

// sub.rs replaces the old way: sub/mod.rs
