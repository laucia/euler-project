extern crate num;

use self::num::integer::Integer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


const BASE_PRIMES: &'static [u64] = &[
      2,   3,   5,   7,  11,  13,  17,  19,  23,  29,  31,  37,  41,  43,  47,
     53,  59,  61,  67,  71,  73,  79,  83,  89,  97, 101, 103, 107, 109, 113,
];

// Cache
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct PrimeCache {
    cache: Vec<u64>,
}

impl PrimeCache {
    pub fn new() -> PrimeCache {
        PrimeCache::with_capacity(50000)
    }

    fn with_capacity(capacity: usize) -> PrimeCache {
        let mut cache = Vec::with_capacity(capacity + BASE_PRIMES.len());
        cache.extend(BASE_PRIMES.iter().cloned());
        PrimeCache {
            cache: cache
        }
    }

    fn max_prime(&self) -> u64 {
        *self.cache.last().unwrap()
    }

    fn is_prime(&self, n: u64) -> bool {
        self.cache.iter()
            .take_while(|& &prime| prime * prime <= n)
            .all(|&prime| !n.is_multiple_of(&prime))
    }

    fn nth_prime(&mut self, nth: usize) -> u64 {
        if self.cache.len() < nth {
            for n in (self.max_prime() + 2 ..) {
                if self.is_prime(n) {
                    self.cache.push(n);
                }
                if self.cache.len() > nth {
                    break;
                }
            }
        }
        return self.cache[nth-1]
    }
}

// Iterator
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct PrimeIterator {
    cache: Rc<RefCell<PrimeCache>>,
    next_nth: usize,
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let prime = self.cache.borrow_mut().nth_prime(self.next_nth);
        self.next_nth += 1;
        Some(prime)
    }
}

// Usefull functions
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn primer() -> PrimeIterator {
    PrimeIterator {
        cache: Rc::new(RefCell::new(PrimeCache::new())),
        next_nth: 1,
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

