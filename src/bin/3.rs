// [Problem 3]: (https://projecteuler.net/problem=3)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
extern crate core;

fn is_prime(value: u64, cache: &mut Vec<u64>) -> bool{
    let top_limit = ((value-1) as f64).sqrt().ceil() as u64;
    for prime in cache {
        if value % prime == 0 {
            return false;
        }
    }
    let start = match cache.len() {
        0 => 2,
        n => cache[n-1],
    };

    for i in start..top_limit {
        if value % i == 0 {
            return false;
        }
    }
    cache.push(value);
    return true;
}

fn biggest_prime_naive(value: u64) -> u64 {
    let mut cache = Vec::new();
    let mut prime_divisors = Vec::new();

    let mut i = 0;
    for i in 2..value {
        if is_prime(i, &mut cache) {
            while value % i == 0 {
                value /= i;
            }
            println!("{:?}", value);
        }
    }
    0
}

fn main() {
    println!("{:?}", biggest_prime_naive(10000));
}
