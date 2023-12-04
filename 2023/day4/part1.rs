use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

fn main() {
    let result = BufReader::new(File::open("day4.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            let mut parts = x.split(':');
            parts.next(); // skip Card XX:

            parts.next().map(|x| {
                let mut iter = x.split('|').into_iter().map(|x| {
                    x.split(' ').filter(|x| x.len() > 0).collect::<HashSet<&str>>()
                });
                iter.next().iter().filter_map(|x| iter.next().map(|y| {
                    x.intersection(&y).fold(0, |acc, _| {
                        if acc == 0 {
                            1
                        } else {
                            acc * 2
                        }
                    })
                })).sum::<i32>()
            }).unwrap_or(0)
        }).sum::<i32>();

    println!("{}", result)
}
