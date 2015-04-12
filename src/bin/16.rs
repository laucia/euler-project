// [Problem 16]: (https://projecteuler.net/problem=16)
// - - - - - - - - - - - - - - - - - - - - - - - - - -
// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?
extern crate num;

use num::bigint::BigUint;
use num::traits::FromPrimitive;

fn solve(base: u64, pow: usize) -> u64 {
    let base = BigUint::from_u64(base).unwrap();
    num::pow(base, pow)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as u64)
        .fold(0,|a, v| a + v)
}

fn main() {
    println!("{:?}",solve(2, 1000));
}

#[test]
fn example() {
    assert_eq!(26, solve(2, 15));
}
