use super::range::Range;
use std::cmp::PartialOrd;

#[derive(Debug)]
struct OrderedRanges<T> where T: PartialOrd{
    ranges: Vec<Range<T>>
}

impl<T> OrderedRanges<T> where T: PartialOrd {
    pub fn new() -> OrderedRanges<T> {
        OrderedRanges{
            ranges: Vec::new()
        }
    }
    pub fn add(mut self, range: Range<T>) -> UnorderedRanges<T> {
        self.ranges.push(range);
        UnorderedRanges{
            ranges: self.ranges
        }
    }
}

struct UnorderedRanges<T> where T: PartialOrd{
    ranges: Vec<Range<T>>
}

impl<T> UnorderedRanges<T> where T: PartialOrd {
    pub fn add(&mut self, range: Range<T>) {
        self.ranges.push(range);
    }

    pub fn new() -> UnorderedRanges<T> {
        UnorderedRanges{
            ranges: Vec::new()
        }
    }
    /*
    pub fn order(mut self) -> OrderedRanges<T> {
        self.ranges.sort_by(|a, b| a.from().partial_cmp(b.from()).unwrap()); //TODO: remove unwrap
        let last_ordered = 0;
        for curr in 1..self.ranges.len() {
            if self.ranges[last_ordered].intersects(&self.ranges[curr]){
                self.ranges[last_ordered] = Range::new(*self.ranges[last_ordered].from(), *self.ranges[curr].to());
            }
        }
        OrderedRanges{
            ranges: self.ranges
        }
    }
    */
}