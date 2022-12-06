use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::IndexMut;

fn main() {
    let mut hs = [0; 26];
    let mut cnt = 0;

    println!("{}", BufReader::new(File::open("day6.txt").expect("I know you are existed"))
        .bytes()
        .map(|x| x.expect("You can do it!"))
        .enumerate()
        .take_while(|(idx, b)| {
            let lastIdx = std::mem::replace(hs.index_mut(usize::from(*b - b'a')), *idx + 1);
            cnt = std::cmp::min(cnt + 1, *idx - lastIdx + 1);
            cnt <= 4
        })
        .count());
}
