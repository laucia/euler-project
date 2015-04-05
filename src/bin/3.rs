// [Problem 3]: (https://projecteuler.net/problem=3)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
extern crate num;

use num::integer::Integer;

fn is_prime(value: u64, cache: &mut Vec<u64>) -> bool{
    let top_limit = ((value-1) as f64).sqrt().ceil() as u64;
    for prime in cache.iter() {
        if value.is_multiple_of(&prime) {
            return false;
        }
    }
    let start = match cache.len() {
        0 => 2,
        n => cache[n-1],
    };

    for i in start..top_limit {
        if value.is_multiple_of(&i) {
            return false;
        }
    }
    cache.push(value);
    return true;
}

fn biggest_prime_naive(value: u64) -> u64 {
    let mut temp_value = value;
    let mut cache = Vec::new();
    let mut prime_divisors = Vec::new();

    let mut i = 2;
    while temp_value > 1 {
        if is_prime(i, &mut cache) {
            while temp_value.is_multiple_of(&i) {
                temp_value /= i;
                prime_divisors.push(i);
            }
        }
        i += 1;
    }
    println!("{:?}", prime_divisors);
    match prime_divisors.len() {
        0 => 0,
        n => prime_divisors[n-1],
    }
}

fn main() {
    println!("{:?}", biggest_prime_naive(600851475143));
}
