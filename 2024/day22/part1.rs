use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let res = BufReader::new(File::open("day22.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let num = x.parse::<u64>().unwrap();

            (0..2000).fold(num, |mut acc, _| {
                acc = ((acc * 64) ^ acc) % 16777216;
                acc = ((acc / 32) ^ acc) % 16777216;
                acc = ((acc * 2048) ^ acc) % 16777216;
                acc
            })
        })
        .sum::<u64>();

    println!("{}", res);
}
