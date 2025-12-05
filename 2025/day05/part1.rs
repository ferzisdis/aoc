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

    let mut res = 0;
    for line in it {
        let id = line.parse::<u64>().unwrap();

        if ranges.iter().any(|&(start, end)| start <= id && id <= end) {
            res += 1;
        }
    }

    println!("{}", res);
}
