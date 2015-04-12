// [Problem 18]: (https://projecteuler.net/problem=18)
// - - - - - - - - - - - - - - - - - - - - - - - - - -
// By starting at the top of the triangle below and moving to adjacent numbers
// on the row below, the maximum total from top to bottom is 23.
// 3
// 7 4
// 2 4 6
// 8 5 9 3
// That is, 3 + 7 + 4 + 9 = 23.
// Find the maximum total from top to bottom of the triangle in the given file.
extern crate algorithm;

use algorithm::triangular::solve_triangle_max_path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn solve(file: File) -> u32 {
    let triangle_line = BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim().split(" ").filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<u32>>()
        })
        //.collect::<Vec<u32>>();
        .collect::<Vec<_>>();
    let mut triangles : Vec<u32> = Vec::new();
    for triangle_list in triangle_line {
        for value in triangle_list {
            triangles.push(value);
        }
    }

    solve_triangle_max_path(&triangles)
}

fn main() {
    let file = File::open("data/p067_triangle.txt").ok().unwrap();
    println!("{:?}", solve(file));
}
