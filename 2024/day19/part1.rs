use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day19.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap());

    let binding = it.next().unwrap();
    let (parts, max_size) =
        binding
            .split(", ")
            .fold((HashSet::new(), 0), |(mut hs, max_size), val| {
                hs.insert(val);
                (hs, max_size.max(val.len()))
            });
    it.next();

    let res = it
        .map(|line| is_valid(&parts, max_size, line.as_str()))
        .filter(|x| *x)
        .count();
    println!("{}", res);
    println!("Finish!");
}

fn is_valid(parts: &HashSet<&str>, max_size: usize, line: &str) -> bool {
    if line.len() == 0 {
        return true;
    }
    for size in 0..=max_size {
        if line.len() < size {
            break;
        }
        let (left, right) = (&line[0..size], &line[size..]);
        if !parts.contains(left) {
            continue;
        }
        if is_valid(parts, max_size, right) {
            return true;
        }
    }
    false
}
