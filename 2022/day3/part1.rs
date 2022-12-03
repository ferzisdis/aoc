use std::{env, fs};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

fn main() {
    println!("{}", BufReader::new(File::open("day3.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|x| {
            let halfSize = x.len() / 2;
            let mut left = HashSet::with_capacity(halfSize);
            let mut right = HashSet::with_capacity(halfSize);
            let mut it = x.bytes();
            for _ in 0..halfSize {
                left.insert(it.next().expect("You are joking?"));
            }
            for char in it {
                right.insert(char);
            }

            return left.intersection(&right)
                .map(|b| {
                    match *b {
                        b'a'..=b'z' => *b - b'a' + 1,
                        b'A'..=b'Z' => *b - b'A' + 27,
                        _ => panic!("Who have checked input?")
                    }
                })
                .map(|b| i32::from(b)).sum::<i32>();
        }).sum::<i32>());
}
