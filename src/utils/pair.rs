#[derive(Debug, Clone)]
pub struct OrderedPair
{
    pub first: u32,
    pub second: u32,
}

impl OrderedPair
{
    pub fn new(first: u32, second: u32) -> Self
    {
        OrderedPair { first, second }
    }

    pub fn invert(&mut self)
    {
        let tmp = self.first;
        self.first = self.second;
        self.second = tmp;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_new_pair()
    {
        let p = OrderedPair::new(1, 2);
        assert_eq!(p.first, 1);
        assert_eq!(p.second, 2);
    }

    #[test]
    fn test_invert()
    {
        let mut p = OrderedPair::new(3, 4);
        p.invert();
        assert_eq!(p.first, 4);
        assert_eq!(p.second, 3);
    }
}
