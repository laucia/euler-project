// [Problem 17]: (https://projecteuler.net/problem=17)
// - - - - - - - - - - - - - - - - - - - - - - - - - -
// If the numbers 1 to 5 are written out in words: one, two, three, four, five,
// then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out
// in words, how many letters would be used?
//
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
// forty-two) contains 23 letters and 115 (one hundred and fifteen) contains
// 20 letters. The use of "and" when writing out numbers is in compliance with
// British usage.

// Word Generators
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn _word_10(n: u32) -> String {
    assert!(n < 10);

    return match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => panic!(),
    };
}

fn _word_20(n: u32) -> String {
    assert!(n < 20);
    if n < 10 {
        return _word_10(n);
    }

    return match n {
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _  => panic!(),
    };
}

fn _word_100(n: u32) -> String {
    assert!(n < 100);
    if n < 20 {
        return _word_20(n);
    }

    let prefix = match n / 10 {
        0 | 1 => panic!(),
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => panic!()
    };
    if n % 10 != 0 {
        format!("{}-{}", prefix, _word_10(n % 10))
    } else {
        prefix
    }
}

fn _word_1000(n: u32) -> String {
    assert!(n < 1000);
    if n < 100 {
        return _word_100(n);
    }

    let prefix = format!("{} hundred", _word_10(n / 100));
    if n % 100 != 0 {
        format!("{} and {}", prefix, _word_100(n % 100))
    } else {
        prefix
    }
}

fn to_word(n: u32) -> String {
    assert!(n <= 1000);
    if n < 1000 {
        return _word_1000(n);
    }
    return "one thousand".to_string();
}

// Solve
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn solve(max: u32) -> usize {
    (1 .. max + 1)
        .map(to_word)
        .map(|w| {
            w.chars()
                .filter(|&c| c != '-' && c != ' ')
                .count()
        }).fold(0, |a, v| a + v)
}

fn main() {
    println!("{:?}",solve(1000));
}

#[test]
fn example() {
    assert_eq!(19, solve(5));
}

