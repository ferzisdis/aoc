use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

fn main() {
    let mut it = BufReader::new(File::open("day3.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"));

    let mut sum = 0;

    loop {
        if let Some(s) = it.next() {
            let firstElf: HashSet<u8, RandomState> = HashSet::from_iter(s.into_bytes());
            let secondElf: HashSet<u8, RandomState> = HashSet::from_iter(it.next().expect("I'm expert in RUST").into_bytes());
            let thirdElf: HashSet<u8, RandomState> = HashSet::from_iter(it.next().expect("I'm so good").into_bytes());

            sum += firstElf.intersection(&secondElf)
                .filter(|b| thirdElf.contains(b))
                .map(|b| {
                    match *b {
                        b'a'..=b'z' => *b - b'a' + 1,
                        b'A'..=b'Z' => *b - b'A' + 27,
                        _ => panic!("Who have checked input?")
                    }
                })
                .map(|b| i32::from(b)).sum::<i32>();
        } else {
            break;
        }
    }

    println!("{}", sum);
}
