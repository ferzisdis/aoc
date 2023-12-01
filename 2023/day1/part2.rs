use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    println!("{}",
             BufReader::new(File::open("day1.txt").expect("what could be happen?"))
                 .lines()
                 .map(|x| x.expect("do it"))
                 .fold(0, |mut acc, x| {
                     let regex = Regex::new("one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();

                     let first = regex.find_iter(x.as_str())
                         .map(|m| match m.as_str() {
                             "one" | "1" => 1,
                             "two" | "2" => 2,
                             "three" | "3" => 3,
                             "four" | "4" => 4,
                             "five" | "5" => 5,
                             "six" | "6" => 6,
                             "seven" | "7" => 7,
                             "eight" | "8" => 8,
                             "nine" | "9" => 9,
                             _ => 0,
                         }).next();

                     let regex_rev = Regex::new("enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|[0-9]").unwrap();
                     let last = regex_rev.find_iter(x.chars().rev().collect::<String>().as_str())
                         .map(|m| match m.as_str() {
                             "eno" | "1" => 1,
                             "owt" | "2" => 2,
                             "eerht" | "3" => 3,
                             "ruof" | "4" => 4,
                             "evif" | "5" => 5,
                             "xis" | "6" => 6,
                             "neves" | "7" => 7,
                             "thgie" | "8" => 8,
                             "enin" | "9" => 9,
                             _ => 0,
                         }).next();

                     acc += first.unwrap() * 10 + last.unwrap();
                     acc
                 })
                );
}
