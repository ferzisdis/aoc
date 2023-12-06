use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("day6.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.expect("do it"));

    let extract_nums = |x: &String| {
        let mut splits = x.split(':');
        splits.next();
        splits.next().iter()
            .flat_map(|x| x.split(' ')
                .filter(|x| x.len() > 0))
            .collect::<String>().parse::<u64>()
    };

    let times = lines.next();
    let dists = lines.next();
    let res = times.iter().flat_map(extract_nums)
        .zip(dists.iter().flat_map(extract_nums))
        .map(|(time, dist)| {
            (1..=time).filter(|ms| (time - ms) * ms > dist).count()
        }).fold(1, |acc, val| acc * val);

    println!("{}", res)
}
