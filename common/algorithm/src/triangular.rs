/// Some utility around triangular numbers
use std::mem;

// Triangular numbers sequence iterator.
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub struct TriangularNums {
    diff: u64,
    next: u64,
}

impl TriangularNums {
    pub fn new() -> TriangularNums {
        TriangularNums { diff: 2, next: 1 }
    }
}

impl Iterator for TriangularNums {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.next + self.diff;
        self.diff += 1;
        Some(mem::replace(&mut self.next, new_next))
    }
}

// Triangular numbers utils.
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn nth_triangular_number(n: u64) -> u64 {
    n*(n+1) /2
}


#[inline]
fn _is_triangular_helper(n: u64) -> f64 {
    (((8 * n + 1) as f64).sqrt() - 1f64) / 2f64
}

pub fn is_triangular(n: u64) -> bool {
    let x = _is_triangular_helper(n);
    x == (x as u64) as f64
}

pub fn reverse_triangular(n: u64) -> Option<u64> {
    let x = _is_triangular_helper(n);
    match x == (x as u64) as f64 {
        true => Some(x as u64),
        false => None
    }
}

// Tests
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[test]
fn nth_triangular_number_test() {
    assert_eq!(28, nth_triangular_number(7));
}

#[test]
fn is_triangular_test() {
    assert_eq!(true, is_triangular(28));
    assert_eq!(false, is_triangular(27));
}


#[test]
fn reverse_triangular_test() {
    assert_eq!(7, reverse_triangular(28).unwrap());
    assert_eq!(8, reverse_triangular(nth_triangular_number(8)).unwrap());
    assert_eq!(None, reverse_triangular(27));
}
