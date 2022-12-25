use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;

fn from_snafu(b: u8) -> i32 {
    match b {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("unknown snafu digit {}", char::from(b))
    }
}

fn to_snafu(d: i32) -> u8 {
    match d {
        2 => b'2',
        1 => b'1',
        0 => b'0',
        -1 => b'-',
        -2 => b'=',
        _ => panic!("unknown snafu digit {}", d)
    }
}

fn main() {
    let mut inputs = BufReader::new(File::open("day25.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|s| {
            let mut num = s.into_bytes().into_iter().rev().map(|d| from_snafu(d)).collect::<Vec<i32>>();
            num
        });

    let mut res = vec![0; 25];

    for num in inputs {
        let mut add = 0;
        for (l, r) in res.iter_mut().zip(num.into_iter().chain((0..).map(|_| 0))) {
            let digit = *l + r + add;
            add = 0;

            match digit {
                d if d > 2 => { *l = d - 5; add += 1; }
                d if d < -2 => { *l = d + 5; add -= 1; }
                d => { *l = d; }
            }
            assert!(*l >= -2 && *l <= 2);
        }
    }

    println!("res = {}", String::from_utf8(res.into_iter().rev().map(|d| to_snafu(d)).collect::<Vec<u8>>()).expect("hmm"));
}
