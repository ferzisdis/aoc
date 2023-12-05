use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{iter, u32};

struct Seed {
    beg: u64,
    end: u64,
}

fn main() {
    let mut lines = BufReader::new(File::open("day5.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.expect("do it"));

    let seeds = lines.next();
    let mut res = seeds.iter().flat_map(|x| {
        let mut splits = x.split(':');
        splits.next();
        let values = splits.next().iter()
            .flat_map(|x| x.split(' ')
                .filter(|x| x.len() > 0)
                .filter_map(|x| x.clone().parse::<u64>().ok())).collect::<Vec<u64>>();
        values.chunks(2)
            .map(|chunk| Seed { beg: chunk[0], end: chunk[0] + chunk[1] - 1 })
            .collect::<Vec<Seed>>()
    }).collect::<Vec<Seed>>();

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
                    map.push(str.split(' ').filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>())
                }
            }
        }
        map.sort_by(|lhs, rhs| lhs.get(1).cmp(&rhs.get(1)));

        res = res.into_iter()
            .flat_map(|seed| {
                let mut intervals = map.iter().flat_map(|x| {
                    let mut v = Vec::new();
                    v.push(x[1]);
                    v.push(x[1] + x[2] - 1);
                    v
                }).filter(|x| *x > seed.beg && *x < seed.end).collect::<VecDeque<u64>>();
                intervals.push_front(seed.beg);
                intervals.push_back(seed.end);

                let mut splitted_seeds = Vec::new();
                for (i, val) in intervals.iter().enumerate() {
                    if let Some(next) = intervals.get(i + 1) {
                        if *val + 1 < *next {
                            splitted_seeds.push(Seed { beg: *val, end: *next - 1})
                        }
                    }
                }
                splitted_seeds
            })
            .map(|x| {
                let loc = map.binary_search_by(|probe| probe.get(1).cmp(&Some(&x.beg)));
                match loc {
                    Ok(pos) => Seed { beg: map[pos][0], end:  x.end - x.beg + map[pos][0] },
                    Err(pos) => {
                        if pos > 0 && x.beg - map[pos - 1][1] < map[pos - 1][2] {
                            Seed { beg: map[pos - 1][0] + (x.beg - map[pos - 1][1]), end:  map[pos - 1][0] + (x.end - map[pos - 1][1])}
                        } else {
                            x
                        }
                    }
                }
        }).collect::<Vec<Seed>>()
    }

    println!("{}", res.iter().map(|x| x.beg).min().unwrap())
}
