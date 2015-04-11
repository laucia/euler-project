// [Problem 1]: (https://projecteuler.net/problem=1)
// - - - - - - - - - - - - - - - - - - - - - - - - -
// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn compute(bound: u64) -> u64 {
    (1 .. bound).filter(|&n| n % 3 == 0 || n % 5 == 0)
                .fold(0, |a, v| a+v)
}

fn main() {
   println!("{:?}", compute(1000));
}


#[test]
fn example() {
    assert_eq!(23, compute(10));
}
