// [Problem 6]: (https://projecteuler.net/problem=6)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// The sum of the squares of the first ten natural numbers is,
//     12 + 22 + ... + 102 = 385
// The square of the sum of the first ten natural numbers is,
//    (1 + 2 + ... + 10)2 = 552 = 3025
// Hence the difference between the sum of the squares of the first ten natural
// numbers and the square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred
// natural numbers and the square of the sum.

fn diff_sum_square_square_sum(min: u64, max: u64) -> u64 {
    let sum = (min..max+1).fold(0, |a, b| a + b);
    let sum2 = (min..max+1).fold(0, |a, b| a + b*b);

    sum*sum - sum2
}

fn main() {
    println!("{:?}", diff_sum_square_square_sum(0, 100));
}

#[test]
fn example() {
    assert_eq!(2640, diff_sum_square_square_sum(0, 10));
}
