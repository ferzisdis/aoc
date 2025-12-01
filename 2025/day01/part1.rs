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
                "R" => (acc.0 + val.parse::<i32>().unwrap()).rem_euclid(100),
                "L" => (acc.0 - val.parse::<i32>().unwrap()).rem_euclid(100),
                _ => unreachable!("Invalid direction"),
            };

            if current == 0 {
                (current, acc.1 + 1)
            } else {
                (current, acc.1)
            }
        });
    println!("{}", ans.1);
}
