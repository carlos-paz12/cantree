//! ## OrderedPair module
//!
//! This module defines the `OrderedPair` struct

/// Represents an ordered pair of 32-bit unsigned integers (`u32`).
#[derive(Debug, Clone, Copy)]
pub struct OrderedPair
{
  /// The first element of the ordered pair.
  pub first: u32,
  /// The second element of the ordered pair.
  pub second: u32,
}

impl OrderedPair
{
  /// ### brief
  /// Creates a new OrderedPair with the specified elements.
  ///
  /// ### Arguments
  /// * `first` - The first `u32` value.
  /// * `second` - The second `u32` value.
  ///
  /// ### Returns
  /// A new OrderedPair containing the provided values.
  pub fn new(first: u32, second: u32) -> Self
  {
    return OrderedPair { first, second };
  }
}
