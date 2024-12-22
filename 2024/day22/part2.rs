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
            let mut hs = HashMap::new();
            let _ = (0..2000).fold((num, 0u64), |(acc, idx), i| {
                let next = next_secret(acc);
                let diff = (next % 10) as i32 - (acc % 10) as i32;
                let next_idx = ((idx % 1000000) * 100 + (diff + 20) as u64) as u64;
                if i >= 3 && !hs.contains_key(&next_idx) {
                    hs.insert(next_idx, next % 10);
                }
                (next, next_idx)
            });
            hs
        })
        .fold(HashMap::new(), |mut acc, cur| {
            for (k, v) in cur {
                *acc.entry(k).or_insert(0) += v;
            }
            acc
        });

    println!("{}", res.values().max().unwrap());
}

fn next_secret(mut acc: u64) -> u64 {
    acc = ((acc * 64) ^ acc) % 16777216;
    acc = ((acc / 32) ^ acc) % 16777216;
    acc = ((acc * 2048) ^ acc) % 16777216;
    acc
}
