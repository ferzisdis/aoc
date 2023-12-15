use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let res = BufReader::new(File::open("day15.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .flat_map(|x| {
            x.split(',')
                .map(|x| x.bytes().fold(0, |acc, ch| {
                    ((acc + ch as i32) * 17) % 256
                })).collect::<Vec<_>>()
        }).sum::<i32>();

    println!("{}", res)
}
