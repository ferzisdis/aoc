use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{iter, u32};

fn main() {
    let mut acc = VecDeque::new();
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
                let wins = iter.next().iter().filter_map(|x| iter.next().map(|y| {
                    x.intersection(&y).count()
                })).sum::<usize>();

                let total_cards = acc.pop_front().unwrap_or(0) + 1;
                for i in 1..=wins {
                    if i - 1 < acc.len() {
                        *acc.get_mut(i - 1).unwrap() += total_cards
                    } else {
                        acc.push_back(total_cards)
                    }
                }

                total_cards
            }).unwrap_or(0)
        }).sum::<i32>();

    println!("{}", result)
}
