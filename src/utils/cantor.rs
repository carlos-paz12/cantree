pub fn gen_key(p: u32, q: u32) -> u64 {
    let (p, q) = (p as u64, q as u64);
    let sum = p + q;
    return (sum * (sum + 1)) / 2 + q;
}
