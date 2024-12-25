use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day25.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap());

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    while let Some(first_row) = it.next() {
        let mut res = Vec::from_iter(std::iter::repeat(0).take(first_row.len()));
        for _ in 0..5 {
            for (i, val) in it.next().unwrap().into_bytes().into_iter().enumerate() {
                res[i] += if val == b'#' { 1 } else { 0 }
            }
        }
        _ = it.next(); // last row
        _ = it.next(); // empty row
        if first_row.contains("#") {
            locks.push(res);
        } else {
            keys.push(res);
        }
    }

    println!("locks {}, keys {}", locks.len(), keys.len());

    let mut fits = 0;
    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(l, k)| *l + *k <= 5) {
                fits += 1;
            }
        }
    }
    println!("{}", fits);
}
