// [Problem 14]: (https://projecteuler.net/problem=14)
// - - - - - - - - - - - - - - - - - - - - - - - - - -
// The following iterative sequence is defined for the set of positive integers:
//     n → n/2 (n is even)
//     n → 3n + 1 (n is odd)
//
// Using the rule above and starting with 13, we generate the following
// sequence:
//      13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1)
// contains 10 terms. Although it has not been proved yet (Collatz Problem),
// it is thought that all starting numbers finish at 1.
//
// Which starting number, under one million, produces the longest chain?
// NOTE: Once the chain starts the terms are allowed to go above one million.
extern crate num;

use std::mem;
use num::Integer;

// Collatz Iterator
// ----------------

struct CollatzIterator {
    next: u64,
}

impl Iterator for CollatzIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.next == 0 {
            return None;
        }

        let next =
            if self.next == 1 {0}
            else if self.next.is_even() {self.next/2}
            else {3*self.next + 1};
        Some(mem::replace(&mut self.next, next))
    }
}

fn len_collatz(start: u64) -> usize {
    CollatzIterator{next: start}.count()
}


fn main() {
    println!("{:?}",(2..1000000).map(|i| (len_collatz(i), i)).max().unwrap());
}

#[test]
fn example() {
    assert_eq!(10 ,len_collatz(13));
}
