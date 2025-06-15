//! ## cantor module
//!
//! This module defines the `gen_key` function.
use super::pair::OrderedPair;

/// Generates a unique key for the given `OrderedPair` using the Cantor
/// pairing function.
///
/// The Cantor pairing function maps two non-negative integers into a single
/// unique `u64`. This ensures that each distinct pair corresponds to a distinct
/// key.
///
/// ### Arguments
/// * `pair` - A reference to the `OrderedPair` for which to generate a key.
///
/// ### Returns
/// A unique `u64` key corresponding to the provided pair.
pub fn gen_key(pair: &OrderedPair) -> u64
{
  let (p, q) = (pair.first as u64, pair.second as u64);
  let sum = p + q;
  (sum * (sum + 1)) / 2 + q
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  /// Tests that `gen_key` produces expected keys for specific known pairs.
  fn test_some_pairs()
  {
    assert_eq!(gen_key(&OrderedPair::new(0, 0)), 0);
    assert_eq!(gen_key(&OrderedPair::new(1, 0)), 1);
    assert_eq!(gen_key(&OrderedPair::new(0, 1)), 2);
    assert_eq!(gen_key(&OrderedPair::new(1, 1)), 4);
    assert_eq!(gen_key(&OrderedPair::new(2, 2)), 12);
  }

  #[test]
  /// Tests that `gen_key` is not commutative.
  ///
  /// This verifies that the order of elements in the pair affects the generated
  /// key: `gen_key(p, q) != gen_key(q, p)` when `p != q`.
  fn test_commutativity_failure()
  {
    assert_ne!(
      gen_key(&OrderedPair::new(0, 1)),
      gen_key(&OrderedPair::new(1, 0))
    );
    assert_ne!(
      gen_key(&OrderedPair::new(1, 2)),
      gen_key(&OrderedPair::new(2, 1))
    );
  }
}
