use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

fn main() {
    println!("{}", BufReader::new(File::open("day2.txt").expect("I know you is existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|x| {
            match &x[..] {
                "A X" => 0 + 3,
                "A Y" => 3 + 1,
                "A Z" => 6 + 2,
                "B X" => 0 + 1,
                "B Y" => 3 + 2,
                "B Z" => 6 + 3,
                "C X" => 0 + 2,
                "C Y" => 3 + 3,
                "C Z" => 6 + 1,
                _ => panic!("who checked input?")
            }
        }).sum::<i32>());
}
