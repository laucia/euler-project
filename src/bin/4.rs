// [Problem 4]: (https://projecteuler.net/problem=4)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
use std::iter;

fn is_palindrome(value: u64) -> bool {
    let string_value = value.to_string();
    let rev_string_value: String = string_value.chars().rev().collect();
    string_value == rev_string_value
}

fn find_max_palindrome(min: u64, max: u64) -> u64 {
    (min..max).flat_map(|x| (min .. x).zip(iter::repeat(x)))
              .map(|(x, y)| x*y)
              .filter(|&x| is_palindrome(x))
              .max().unwrap()
}

fn main() {
    println!("{:?}", find_max_palindrome(100, 1000));
}

#[test]
fn example() {
    assert_eq!(9009, find_max_palindrome(10, 100));
}
