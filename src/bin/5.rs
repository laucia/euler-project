// [Problem 5]: (https://projecteuler.net/problem=5)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// 2520 is the smallest number that can be divided by each of the numbers from
// 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the
// numbers from 1 to 20?
extern crate algorithm;

use algorithm::prime::prime_divisor_counts;
use std::collections::HashMap;
use std::cmp;


fn merge_prime_divisors(prime_divisors: &Vec<HashMap<u64, u64> >) -> HashMap<u64, u64>{
    let mut merged_counts : HashMap<u64, u64> = HashMap::new();
    for prime_count in prime_divisors.iter() {
        for (prime, count) in prime_count.iter(){
            if !merged_counts.contains_key(&prime){
                merged_counts.insert(*prime, 0);
            }
            match merged_counts.get_mut(prime) {
                Some(x) => *x = cmp::max(*x, *count),
                None => (),
            };
        }
    }
    merged_counts
}

fn min_product_solver(max_range: u64) -> u64 {
    let mut prime_divisors = Vec::new();
    for i in 2..max_range+1 {
        prime_divisors.push(prime_divisor_counts(i))
    }
    let merged_divisors = merge_prime_divisors(&prime_divisors);
    let mut result = 1;
    for (prime, count) in merged_divisors.iter(){
        result *= prime.pow(*count as u32);
    }
    result
}

fn main() {
    println!("{:?}", min_product_solver(20));
}

#[test]
fn example() {
    assert_eq!(2520, min_product_solver(10));
}
