//! ## utils module
//!
//! This module provides utility components used throughout the crate,
//! including mathematical helpers and data structures like ordered pairs.

/// Defines the Cantor pairing function used to generate unique keys
/// from ordered pairs.
pub mod cantor;

/// Defines the `OrderedPair` struct, representing a pair of unsigned integers
/// with ordering guarantees and related methods.
pub mod pair;
