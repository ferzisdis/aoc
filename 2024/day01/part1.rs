use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (mut lhs, mut rhs) =
        BufReader::new(File::open("day01.txt").expect("what could be happen?"))
            .lines()
            .map(|e| e.expect("line should exist"))
            .fold((Vec::new(), Vec::new()), |(mut lhs, mut rhs), line| {
                let mut splitted = line.split("   ");
                lhs.push(splitted.next().unwrap().parse::<i32>().unwrap());
                rhs.push(splitted.next().unwrap().parse::<i32>().unwrap());
                (lhs, rhs)
            });

    lhs.sort();
    rhs.sort();

    println!(
        "{}",
        lhs.into_iter()
            .zip(rhs.into_iter())
            .map(|(l, r)| (r - l).abs())
            .sum::<i32>()
    );
}
