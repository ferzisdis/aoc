extern crate core;

use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;


fn shift(origin: Vec<i32>, op: u8, st: impl Iterator<Item = i32>) -> Vec<i32> {
    let mut res = Vec::new();
    res.reserve(origin.len());

    for (part, back) in origin.iter().zip(st) {
        let part = match (*part, op) {
            (_, b'<') if *part & 0b1000000 > 0 => return origin,
            (_, b'>') if *part & 0b0000001 > 0 => return origin,
            (part, b'>') => part >> 1,
            (part, b'<') => part << 1,
            _ => panic!("other")
        };
        if part & back > 0 {
            return origin
        }
        res.push(part);
    }

    res
}

fn can_shift_down(origin: &Vec<i32>, st: impl Iterator<Item = i32>) -> bool {
    let mut res = Vec::new();
    res.reserve(origin.len());

    for (part, back) in origin.iter().zip(st) {
        if part & back > 0 {
            return false
        }
        res.push(*part);
    }

    if res.len() < origin.len() {
        return false
    }
    true
}

fn main() {
    let mut inputs = BufReader::new(File::open("day17.txt").expect("I know you are existed"))
        .bytes()
        .map(|x| x.expect("I can do it!"))
        .collect::<Vec<u8>>()
        .into_iter()
        .cycle();

    let mut blocks_it = (0..2022).map(|n| {
        match n % 5 {
            0 => {
                vec![
                    0b0011110
                ]
            },
            1 => {
                vec![
                    0b0001000,
                    0b0011100,
                    0b0001000
                ]
            },
            2 => {
                vec![
                    0b0000100,
                    0b0000100,
                    0b0011100
                ]
            },
            3 => {
                vec![
                    0b0010000,
                    0b0010000,
                    0b0010000,
                    0b0010000
                ]
            },
            4 => {
                vec![
                    0b0011000,
                    0b0011000
                ]
            },
            _ => panic!("Impossible")
        }
    });

    let mut st = VecDeque::new();

    loop {
        match blocks_it.next() {
            Some(block) => {
                for _ in 0..(block.len() + 3) {
                    st.push_front(0);
                }

                let mut block = block;
                for down in 0..=st.len() {
                    let op = inputs.next().expect("cycled");
                    block = shift(block, op, st.iter().skip(down).map(|x| *x));

                    if !can_shift_down(&block, st.iter().skip(down + 1).map(|x| *x)) {
                        for (b, back) in block.iter().zip(st.iter_mut().skip(down)) {
                            *back = *back | *b;
                        }
                        break
                    }
                }

                loop {
                    match st.front() {
                        Some(b) if *b == 0 => { st.pop_front(); }
                        _ => break
                    }
                }
            },
            None => break
        }
    }

    println!("{:?}", st.len())
}

