use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (lhs, rhs) = BufReader::new(File::open("day01.txt").expect("what could be happen?"))
        .lines()
        .map(|e| e.expect("line should exist"))
        .fold((Vec::new(), HashMap::new()), |(mut lhs, mut rhs), line| {
            let mut splitted = line.split("   ");
            let (l, r) = (
                splitted.next().unwrap().parse::<i32>().unwrap(),
                splitted.next().unwrap().parse::<i32>().unwrap(),
            );

            lhs.push(l);
            let cnt = rhs.get(&r).unwrap_or(&0i32);
            rhs.insert(r, cnt + 1);
            (lhs, rhs)
        });

    println!(
        "{}",
        lhs.into_iter()
            .map(|l| l * rhs.get(&l).unwrap_or(&0i32))
            .sum::<i32>()
    );
}
