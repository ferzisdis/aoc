use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let result = BufReader::new(File::open("day02.txt").expect("what could be happen?"))
        .lines()
        .map(|e| e.expect("line should exist"))
        .filter_map(|e| if is_safe(e) { Some(()) } else { None })
        .count();

    println!("{}", result);
}

fn is_safe(report: String) -> bool {
    let levels = report
        .split(" ")
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    levels
        .iter()
        .zip(levels.iter().skip(1))
        .all(|(l, r)| l < r && r - l <= 3)
        || levels
            .iter()
            .zip(levels.iter().skip(1))
            .all(|(l, r)| l > r && l - r <= 3)
}
