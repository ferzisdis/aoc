use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}",
             BufReader::new(File::open("day1.txt").expect("what could be happen?"))
                 .lines()
                 .map(|x| x.expect("do it"))
                 .fold(0, |mut acc, x| {
                     let mut digits = x.chars()
                         .filter_map(|ch| ch.to_digit(10));

                     let first = digits.next().expect("should be at least one digit");
                     let last = digits.next_back().unwrap_or(first);
                     acc += first * 10 + last;
                     acc
                 })
                );
}
