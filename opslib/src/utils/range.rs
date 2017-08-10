use std::cmp::{PartialOrd, Ord};
use std::iter::FromIterator;

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
    ///Merges two ranges into one
    ///
    ///Panics if two ranges do not intersect.
    pub fn merge(self, other: Range<T>) -> Range<T>{
        assert!(self.intersects(&other));
        Range{
            from_val: if self.from_val<other.from_val{
                    self.from_val
                } else {
                    other.from_val
                },
            to_val: if self.to_val > other.to_val {
                    self.to_val
                } else {
                    other.to_val
                }
        }
    }
}

///Simplifies collection of ranges - merges them into ordered and smalles possible set of ranges
pub fn simplify<T, I: IntoIterator<Item=Range<T>>>(iter: I) -> Vec<Range<T>> where T: Clone+Ord {
    let mut ranges:Vec<Range<T>> = Vec::from_iter(iter);
    ranges.sort_by(|a, b| a.from().cmp(b.from()));
    let mut last_ordered = 0;
    //this could be done better with ranges.iter().skip(1).enumerate() but compiler complains
    for curr in 1..ranges.len() {
        if ranges[last_ordered].intersects(&ranges[curr]){
            let max_to = if ranges[curr].to() > ranges[last_ordered].to() {
                ranges[curr].to().clone()
            } else {
                ranges[last_ordered].to().clone()
            };
            let new_val = Range::new(ranges[last_ordered].from().clone(), max_to);
            ranges[last_ordered] = new_val;
        } else {
            last_ordered += 1;
        }
    }
    ranges.truncate(last_ordered + 1);
    ranges
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

    #[test]
    fn simplify_separate(){
        let mut input = vec![Range::new(4,6), Range::new(1,3), Range::new(8,9)];
        let result = simplify(input);
        let expected = vec![Range::new(1,3), Range::new(4,6), Range::new(8,9)];
        assert_eq!(result, expected);
    }

    #[test]
    fn order_merge_one_big(){
        let input = vec![Range::new(4,6), Range::new(1,4), Range::new(0,9)];
        let result = simplify(input);
        let expected = vec![Range::new(0,9)];
        assert_eq!(result, expected);
    }

    #[test]
    fn order_merge_three_small(){
        let input = vec![Range::new(4,6), Range::new(1,4), Range::new(5,9)];
        let result = simplify(input);
        let expected = vec![Range::new(1,9)];
        assert_eq!(result, expected);
    }

    #[test]
    fn order_merge_mix(){
        let input = vec![Range::new(4,6), Range::new(1,3), Range::new(6,9)];
        let result = simplify(input);
        let expected = vec![Range::new(1,3), Range::new(4,9)];
        assert_eq!(result, expected);
    }

}
