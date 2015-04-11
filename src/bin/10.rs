// [Problem 10]: (https://projecteuler.net/problem=10)
// - - - - - - - - - - - - - - - - - - - - - - - - - -
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.
extern crate algorithm;

use algorithm::prime::primer;

fn solve(max: u64) -> u64 {
    primer().take_while(|&x| x < max)
            .fold(0, |a, v| a + v)
}

fn main() {
    println!("{:?}", solve(2000000));
}

#[test]
fn example() {
    assert_eq!(17, solve(10));
}
