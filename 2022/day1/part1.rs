use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

fn main() {
    println!("{}", BufReader::new(File::open("day1.txt").expect("I know you is existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .fold(vec![0], |mut acc, x| {
            if x == "" {
                acc.push(0);
            } else {
                *acc.last_mut().expect("It is possible?") += x.parse::<i32>().expect("Seriously?");
            }
            acc
        }).iter().max().expect("Last try!"));
}
