// [Problem 3]: (https://projecteuler.net/problem=3)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
use std::num;

fn is_prime(value: u64, cache: &mut[u64]){
    let top_limit = num::sqrt(value-1).ceil()
    for prime in cache {
        if i % prime == 0 {
            return false;
        }
    }
}

fn biggest_prime_naive(value: u64) -> u64 {
    let mut vector = Vec::new()
    let mut is_prime;
    for i in 2...top_limit {
        is_prime = false;
        a = for prime in vector {
            if i % prime == 0 {
                break;
            }
        }
        if a.peek().is_none() {
            vector.append(i)

        }
    }
}
