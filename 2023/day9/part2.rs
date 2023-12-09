use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let res = BufReader::new(File::open("day9.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| x.split_ascii_whitespace()
            .filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<_>>())
        .map(|mut v| {
            for i in 0.. {
                let mut it = v.iter_mut().skip(i).rev();
                let mut prev = it.next().expect("at least one digit");
                let mut all_zeroes = true;
                loop {
                    if let Some(cur) = it.next() {
                        *prev = *prev - *cur;
                        if *prev != 0 {
                            all_zeroes = false
                        }
                        prev = cur
                    } else {
                        break
                    }
                }
                if all_zeroes {
                    return v.iter().take(i + 1).rev().fold(0, |acc, x| x - acc)
                }
            }
            panic!("unreachable!")
        }).sum::<i32>();

    println!("{}", res);
}
