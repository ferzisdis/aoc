use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let source = BufReader::new(File::open("day06.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split(' ')
                .filter(|part| !part.is_empty())
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let problems = (0..source[0].len())
        .map(|col| {
            source
                .iter()
                .map(|row| row[col].clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let res = problems
        .into_iter()
        .map(|mut problem| {
            problem.reverse();
            let mut it = problem.into_iter();
            let operation = it.next().unwrap();
            match operation.as_str() {
                "+" => it.map(|x| x.parse::<u64>().unwrap()).sum::<u64>(),
                "*" => it.map(|x| x.parse::<u64>().unwrap()).product::<u64>(),
                _ => panic!("Unknown operation"),
            }
        })
        .sum::<u64>();

    println!("{}", res);
}
