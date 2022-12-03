use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

fn main() {
    println!("{}", BufReader::new(File::open("day2.txt").expect("I know you is existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|x| {
            match &x[..] {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => panic!("who checked input?")
            }
        }).sum::<i32>());
}
