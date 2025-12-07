use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let res = BufReader::new(File::open("day07.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .fold(HashMap::new(), |mut set, line| {
            for (i, ch) in line.bytes().enumerate() {
                match ch {
                    b'S' => {
                        set.insert(i, 1u64);
                    }
                    b'^' => {
                        if let Some(prev) = set.remove(&i) {
                            set.entry(i + 1)
                                .and_modify(|val| *val += prev)
                                .or_insert(prev);
                            set.entry(i - 1)
                                .and_modify(|val| *val += prev)
                                .or_insert(prev);
                        }
                    }
                    _ => {}
                }
            }
            set
        })
        .into_values()
        .sum::<u64>();

    println!("{}", res);
}
