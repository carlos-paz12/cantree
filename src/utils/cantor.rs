pub fn gen_key(p: u32, q: u32) -> u64 {
    let (p, q) = (p as u64, q as u64);
    let sum = p + q;
    return (sum * (sum + 1)) / 2 + q;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // [!] Test key generation for known pairs.
    fn test_some_pairs() {
        assert_eq!(gen_key(0, 0), 0);
        assert_eq!(gen_key(1, 0), 1);
        assert_eq!(gen_key(0, 1), 2);
        assert_eq!(gen_key(1, 1), 4);
        assert_eq!(gen_key(2, 2), 12);
    }

    #[test]
    // [!] Test non-commutativity of pairs.
    fn test_commutativity_failure() {
        // [!] `gen_key` is NOT commutative, i.e., `gen_key(p, q)` != `gen_key(q, p)`.
        assert_ne!(gen_key(0, 1), gen_key(1, 0));
        assert_ne!(gen_key(1, 2), gen_key(2, 1));
    }
}
