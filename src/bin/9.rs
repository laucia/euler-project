// [Problem 9]: (https://projecteuler.net/problem=9)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//     a2 + b2 = c2
// For example, 32 + 42 = 9 + 16 = 25 = 52.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
use std::{cmp, iter};

fn solve(sum: u64) -> u64 {
    (2..sum-1).flat_map(|c| {
                   let max_a = cmp::min(
                        (sum - c) / 2, // a < b so with a + b + c = sum
                        ((c * c / 2) as f64).sqrt() as u64 // same logic with a2 + b2 = c2
                    );
                   (1..max_a).zip(iter::repeat(c))
               })
              .map(|(a, c)| (a, sum - c - a, c))
              .find(|&(a, b, c)| a * a + b * b == c * c)
              .map(|(a, b, c)| a * b * c)
              .unwrap()
}

fn main() {
    println!("{:?}", solve(1000));
}

#[test]
fn example() {
    assert_eq!(31875000, solve(1000));
}
