use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let ans = BufReader::new(File::open("day01.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .fold((50, 0), |acc, x| {
            let (dir, val) = x.split_at(1);
            let current = match dir {
                "R" => (acc.0 + val.parse::<i32>().unwrap()),
                "L" => (acc.0 - val.parse::<i32>().unwrap()),
                _ => unreachable!("Invalid direction"),
            };

            let rem = current.rem_euclid(100);
            let div = current.div_euclid(100);
            println!("{} {} ({}, {})", acc.0, x, div, rem);

            (rem, acc.1 + div.abs())
        });
    println!("{}", ans.1);
}
