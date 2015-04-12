// [Problem 15]: (https://projecteuler.net/problem=15)
// - - - - - - - - - - - - - - - - - - - - - - - - - -
// Starting in the top left corner of a 2×2 grid, and only being able to move
// to the right and down, there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?
extern crate num;

use num::bigint::BigUint;
use num::traits::{FromPrimitive, One};

fn factorial(n: u64) -> BigUint {
    (2..n+1).fold(BigUint::one(), |a, v| a*BigUint::from_u64(v).unwrap())
}

fn solve(w: u64, h: u64) -> BigUint {
    factorial(w+h)/(factorial(w)*factorial(h))
}

fn main() {
    println!("{:?}", solve(20, 20).to_string());
}

#[test]
fn example() {
    assert_eq!("6", solve(2, 2).to_string());
}

