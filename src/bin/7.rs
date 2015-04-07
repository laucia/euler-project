// [Problem 7]: (https://projecteuler.net/problem=7)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
// that the 6th prime is 13.
// What is the 10 001st prime number?
extern crate algorithm;

use algorithm::prime::primer;

fn nth_prime(n: usize) -> u64 {
    primer().nth(n-1).unwrap()
}

fn main() {
    println!("{:?}", nth_prime(10001));
}

#[test]
fn example() {
    assert_eq!(13, nth_prime(6));
}
