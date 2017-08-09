use std::cmp::{PartialOrd, PartialEq};

///Range from one value to the other
#[derive(Debug, PartialEq)]
pub struct Range<T> where T: PartialOrd{
    from_val: T,
    to_val: T
}

impl<T> Range<T> where T: PartialOrd{
    ///Creates a new range from 'from' to 'to'.
    /// Panics if to is less than from
    pub fn new(from: T, to: T) -> Range<T>{
        if to<from {
            panic!("Begining of the range needs to be smaller or equal end of the range");
        }
        Range{from_val: from, to_val: to}
    }

    ///Checks if two ranges intersect.
    ///
    /// Similar to overlap().Range::new(1,5) intersects with Range::new(5,9) but it does not overlap.
    pub fn intersects(&self, other: &Range<T>) -> bool {
        !(other.to_val < self.from_val || other.from_val > self.to_val)
    }

    ///Checks if provided ranges overlap.
    ///
    /// Similar to intersects(). Range::new(1,5) intersects with Range::new(5,9) but it does not overlap.
    pub fn overlaps(&self, other: &Range<T>) -> bool {
        !(other.to_val <= self.from_val || other.from_val >= self.to_val)
    }

    ///Accessor that gets the 'from' value of the range.
    pub fn from(&self) -> &T {
        &self.from_val
    }

    ///Accessor that get the 'to' value of the range.
    pub fn to(&self) -> &T {
        &self.to_val
    }

    ///Checks if the given value is withing the range.
    pub fn contains(&self, value: &T) -> bool {
        self.from_val <= *value && self.to_val >= *value
    }

    ///Check if the range contains the given range.
    pub fn contains_range(&self, other: &Range<T>) -> bool {
        self.from_val <= other.from_val && self.to_val >= other.to_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let range = Range::new(3,5);
        assert_eq!(*range.from(), 3);
        assert_eq!(*range.to(), 5);
    }

    #[test]
    fn create_single() {
        let range = Range::new(8, 8);
    }

    #[test]
    fn equality() {
        assert!(Range::new(1,4) == Range::new(1,4));
        assert!(Range::new(5,6) != Range::new(1,4));
    }

    #[test]
    #[should_panic]
    fn invalid_arguments(){
        let rng = Range::new(8,6);
    }

    #[test]
    fn intersects() {
        assert!(Range::new(1,3).intersects(&Range::new(2,5)));
        assert!(Range::new(1,3).intersects(&Range::new(3,5)));
        assert!(!Range::new(1,3).intersects(&Range::new(4,5)));
    }

    #[test]
    fn overlaps() {
        assert!(Range::new(1,3).overlaps(&Range::new(2,5)));
        assert!(!Range::new(1,3).overlaps(&Range::new(3,5)));
        assert!(!Range::new(1,3).overlaps(&Range::new(4,5)));
    }

    #[test]
    fn contains() {
        assert!(Range::new(1,3).contains(&2));
        assert!(!Range::new(1,3).contains(&5));
    }

    #[test]
    fn contains_range() {
        assert!(Range::new(1,5).contains_range(&Range::new(2,4)));
        assert!(Range::new(1,5).contains_range(&Range::new(1,5)));
        assert!(!Range::new(2,3).contains_range(&Range::new(1,5)));
        assert!(!Range::new(2,4).contains_range(&Range::new(3,5)));
        assert!(!Range::new(2,3).contains_range(&Range::new(4,5)));
    }

}
