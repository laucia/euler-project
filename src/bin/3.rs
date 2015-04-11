// [Problem 3]: (https://projecteuler.net/problem=3)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
extern crate algorithm;

use algorithm::prime::prime_factorization;

fn biggest_prime_naive(value: u64) -> u64 {
    let prime_divisors = prime_factorization(value);
    match prime_divisors.len() {
        0 => 0,
        n => prime_divisors[n-1],
    }
}

fn main() {
    println!("{:?}", biggest_prime_naive(600851475143));
}

#[test]
fn it_works() {
    assert_eq!(29, biggest_prime_naive(13195));
}
