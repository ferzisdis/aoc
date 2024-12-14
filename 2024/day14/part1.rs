use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let field = (101, 103); // (101, 103);
    let seconds = 100;
    let res = BufReader::new(File::open("day14.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split_once(" ")
                .map(|(pos, vel)| {
                    (
                        pos.split_once("=")
                            .unwrap()
                            .1
                            .split_once(",")
                            .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                            .unwrap(),
                        vel.split_once("=")
                            .unwrap()
                            .1
                            .split_once(",")
                            .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                            .unwrap(),
                    )
                })
                .unwrap()
        })
        .map(|(pos, vel)| {
            (
                (pos.0 + seconds * vel.0).rem_euclid(field.0),
                (pos.1 + seconds * vel.1).rem_euclid(field.1),
            )
        })
        .fold(HashMap::new(), |mut acc, pos| {
            println!("pos ({},{})", pos.0, pos.1);
            let half = (field.0 / 2 + 1, field.1 / 2 + 1);
            if pos.0 + 1 != half.0 && pos.1 + 1 != half.1 {
                let quoter = ((pos.0 + 1) / half.0, (pos.1 + 1) / half.1);
                println!("quoter ({},{})", quoter.0, quoter.1);
                *acc.entry(quoter).or_insert(0) += 1;
            }
            acc
        })
        .into_values()
        .fold(1i64, |acc, val| {
            println!("val {}", val);
            acc * val
        });

    println!("{}", res);
}
