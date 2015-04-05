extern crate num;

use self::num::integer::Integer;
use std::collections::HashMap;

pub struct Primer {
    cache: Vec<u64>,
    last_returned: u64,
}

impl Primer {
    fn is_prime(&mut self, value: u64) -> bool {
        for prime in self.cache.iter() {
            if value.is_multiple_of(&prime) {
                return value/prime == 1;
            }
        }
        let start = match self.cache.len() {
            0 => 2,
            n => self.cache[n-1],
        };

        let top_limit = (value as f64).sqrt().ceil() as u64;
        for i in start..top_limit {
            if value.is_multiple_of(&i) {
                return false;
            }
        }
        self.cache.push(value);
        return true;
    }
}

// Implement 'Iterator' for 'Primer'
impl Iterator for Primer {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let mut i = self.last_returned;
        let last_cached = *self.cache.last().unwrap();

        // Check cache
        if i < last_cached {
            for prime in self.cache.iter() {
                if *prime > i {
                    self.last_returned = *prime;
                    return Some(*prime);
                }
            }
        }

        // Create a new prime number
        i += 1;
        while !self.is_prime(i) {
            i += 1;
        }
        self.last_returned == i;
        Some(i)
    }
}

pub fn primer() -> Primer {
    Primer {
        cache: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47],
        last_returned: 0,
    }
}

pub fn prime_factorization(value: u64) -> Vec<u64> {
    let mut prime_divisors = Vec::new();
    let mut temp_value = value;

    for prime in primer() {
        while temp_value.is_multiple_of(&prime) {
            temp_value /= prime;
            prime_divisors.push(prime);
        }
        if temp_value == 1 {
            break;
        }
    }

    prime_divisors
}

pub fn prime_divisor_counts(value: u64) -> HashMap<u64, u64> {
    let mut divisor_counts : HashMap<u64, u64> = HashMap::new();
    let prime_divisors = prime_factorization(value);

    for divisor in prime_divisors {
        if !divisor_counts.contains_key(&divisor){
            divisor_counts.insert(divisor, 0);
        }
        match divisor_counts.get_mut(&divisor) {
            Some(x) => *x += 1,
            None => (),
        };
    }

    divisor_counts
}

