use super::pair::OrderedPair;

pub fn gen_key(pair: &OrderedPair) -> u64
{
  let (p, q) = (pair.first as u64, pair.second as u64);
  let sum = p + q;
  return (sum * (sum + 1)) / 2 + q;
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  /// Test key generation for known pairs.
  fn test_some_pairs()
  {
    assert_eq!(gen_key(&OrderedPair::new(0, 0)), 0);
    assert_eq!(gen_key(&OrderedPair::new(1, 0)), 1);
    assert_eq!(gen_key(&OrderedPair::new(0, 1)), 2);
    assert_eq!(gen_key(&OrderedPair::new(1, 1)), 4);
    assert_eq!(gen_key(&OrderedPair::new(2, 2)), 12);
  }

  #[test]
  /// Test non-commutativity of pairs.
  /// `gen_key` is NOT commutative, i.e., `gen_key(p q)!=gen_key(q,p)`.
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
