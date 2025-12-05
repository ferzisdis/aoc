use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day05.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap());

    let mut ranges = Vec::new();

    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        ranges.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
    }

    ranges.sort_by_key(|(start, _)| *start);

    let mut res = 0;
    let mut prev_end = 0;

    for (start, end) in ranges {
        if end < prev_end {
            continue;
        }

        res += end - start.max(prev_end) + 1;
        prev_end = end.max(prev_end) + 1;
    }

    println!("{}", res);
}
