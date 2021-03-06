/// Some utility around triangular numbers

use std::cmp::Ord;
use std::cmp;
use std::mem;
use std::ops::Add;

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

pub fn reverse_triangular(n: u64) -> Result<u64, String> {
    let x = _is_triangular_helper(n);
    match x == (x as u64) as f64 {
        true => Ok(x as u64),
        false => Err("Not a triangular number".to_string()),
    }
}

// Triangle Of Number
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct NumberTriangle<T> {
    numbers: Vec<T>,
    side: usize,
}

impl<T: Clone> NumberTriangle<T> {
    fn new(numbers: Vec<T>) -> NumberTriangle<T> {
        let length = numbers.len();
        NumberTriangle{
            numbers: numbers,
            side : match reverse_triangular(length as u64) {
                Ok(x) => x as usize,
                Err(msg) => panic!(msg),
            }
        }
    }

    fn get(&self, line: usize, col: usize) -> T {
        assert!(line < self.side && col < self.side);

        let offset = nth_triangular_number(line as u64) as usize;
        self.numbers[col + offset].clone()
    }

    fn set(&mut self, line: usize, col: usize, value: T) {
        assert!(line < self.side && col < self.side);

        let offset = nth_triangular_number(line as u64) as usize;
        self.numbers[col + offset] = value;
    }
}

pub fn solve_triangle_max_path<T: Add<T, Output = T> + Clone + Ord>(triangle_number: &[T]) -> T {
    let mut trinum : NumberTriangle<T> = NumberTriangle::new(triangle_number.iter().map(|x| x.clone()).collect());
    let mut new_value;
    for line in (1 .. trinum.side).rev() {
        for col in (0 .. line) {
            new_value = trinum.get(line - 1, col) + cmp::max(
                trinum.get(line, col),
                trinum.get(line, col + 1)
            );
            trinum.set(line - 1, col, new_value);
        }
    }
    trinum.get(0, 0)
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
    assert!(reverse_triangular(27).is_err());
}

#[test]
fn solve_triangle_max_path_test() {
    assert_eq!(23, solve_triangle_max_path(&[
        3,
        7, 4,
        2, 4, 6,
        8, 5, 9, 3,
    ]));
}
