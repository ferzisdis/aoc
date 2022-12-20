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

fn process_block(block: Vec<i32>, st: &mut VecDeque<i32>, inputs: &mut impl Iterator<Item = u8>) {
    for _ in 0..(block.len() + 3) {
        st.push_back(0);
    }

    let mut block = block;
    for down in 0..=st.len() {
        let op = inputs.next().expect("cycled");
        block = shift(block, op, st.iter().rev().skip(down).map(|x| *x));

        if !can_shift_down(&block, st.iter().rev().skip(down + 1).map(|x| *x)) {
            for (b, back) in block.iter().zip(st.iter_mut().rev().skip(down)) {
                *back = *back | *b;
            }
            break
        }
    }

    loop {
        match st.back() {
            Some(b) if *b == 0 => { st.pop_back(); }
            _ => break
        }
    }
}

fn main() {
    let mut inputs = BufReader::new(File::open("day17.txt").expect("I know you are existed"))
        .bytes()
        .map(|x| x.expect("I can do it!"))
        .collect::<Vec<u8>>()
        .into_iter()
        .cycle();

    let mut blocks_it = (0..).map(|n| {
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
    let mut count_blocks = 0;
    for _ in 0..10000 {
        count_blocks += 1;
        match blocks_it.next() {
            Some(block) => {
                process_block(block, &mut st, &mut inputs);
            },
            None => break
        }
    }

    println!("blocks {:?}", st.len());

    let start_of = 2000;
    let mut length_of_repeat = 0;
    for idx in (start_of+1)..st.len() {
        if (start_of..idx).zip(idx..).all(|(lhs, rhs)| rhs < st.len() && st[lhs] == st[rhs]) {
            length_of_repeat = idx - start_of;
            println!("found repeat from {} to {} with len {}", start_of, idx, length_of_repeat);
            // break;
        }
    }

    let mut start_of_unfinished_block = 0;
    for i in 1.. {
        if start_of + length_of_repeat * (i + 1) > st.len() {
            start_of_unfinished_block = start_of + length_of_repeat * i;
            println!("unfinished block {}", start_of_unfinished_block);
            break;
        }
    }

    for _ in 0.. {
        count_blocks += 1;
        match blocks_it.next() {
            Some(block) => {
                process_block(block, &mut st, &mut inputs);
                if (start_of..start_of+length_of_repeat).zip(start_of_unfinished_block..start_of_unfinished_block+length_of_repeat)
                    .all(|(lhs, rhs)| rhs < st.len() && st[lhs] == st[rhs]) {
                    start_of_unfinished_block = start_of_unfinished_block+length_of_repeat;
                    println!("new block {} at {} figure", start_of_unfinished_block, count_blocks);
                    break;
                }
            },
            None => break
        }
    }

    let mut target_rocks = 1000000000000i64;
    let mut addition = 0i64;
    let blocks_checkpoint = count_blocks;
    for _ in 0.. {
        count_blocks += 1;
        match blocks_it.next() {
            Some(block) => {
                process_block(block, &mut st, &mut inputs);
                if (start_of..start_of+length_of_repeat).zip(start_of_unfinished_block..start_of_unfinished_block+length_of_repeat)
                    .all(|(lhs, rhs)| rhs < st.len() && st[lhs] == st[rhs]) {
                    println!("crated {} blocks by {} figures", length_of_repeat, count_blocks - blocks_checkpoint);
                    let repeations = (target_rocks - count_blocks as i64) / (count_blocks - blocks_checkpoint) as i64;
                    println!("reduce {} prepeats", repeations);
                    target_rocks = target_rocks - count_blocks as i64 - (count_blocks - blocks_checkpoint) as i64 * repeations;
                    addition = length_of_repeat as i64 * repeations;
                    println!("ost {} rocks, addition {}", target_rocks, addition);
                    break;
                }
            },
            None => break
        }
    }

    for _ in 0..target_rocks {
        count_blocks += 1;
        match blocks_it.next() {
            Some(block) => {
                process_block(block, &mut st, &mut inputs);
            },
            None => break
        }
    }

    println!("result {:?}", st.len() as i64 + addition)
}

