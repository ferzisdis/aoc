use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{iter, u32};

fn main() {
    let mut lines = BufReader::new(File::open("day5.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.expect("do it"));

    let seeds = lines.next();
    let mut res = seeds.iter().flat_map(|x| {
        let mut splits = x.split(':');
        splits.next();
        splits.next().iter()
            .flat_map(|x| x.split(' ')
                .filter(|x| x.len() > 0)
                .filter_map(|x| x.clone().parse::<u32>().ok())).collect::<Vec<u32>>()
    }).collect::<Vec<u32>>();

    lines.next(); // skip empty line

    loop {
        if lines.next().is_none() {
            break
        } // skip name of maps

        let mut map = Vec::new();
        loop {
            match lines.next() {
                None => break,
                Some(str) if str.len() == 0 => break,
                Some(str) => {
                    map.push(str.split(' ').filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>())
                }
            }
        }
        map.sort_by(|lhs, rhs| lhs.get(1).cmp(&rhs.get(1)));

        res = res.into_iter().map(|x| {
            let loc = map.binary_search_by(|probe| probe.get(1).cmp(&Some(&x)));
            match loc {
                Ok(pos) => map[pos][0],
                Err(pos) => {
                    if pos > 0 && x - map[pos - 1][1] < map[pos - 1][2] {
                        map[pos - 1][0] + (x - map[pos - 1][1])
                    } else {
                        x
                    }
                }
            }
        }).collect::<Vec<u32>>()
    }

    println!("{}", res.iter().min().unwrap())
}
