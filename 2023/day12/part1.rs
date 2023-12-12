use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::ControlFlow;

fn main() {
    println!("{}", BufReader::new(File::open("day12.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .filter_map(|x| {
            x.split_once(" ").map(|(left, right)| {
                (left.bytes().collect::<VecDeque<_>>(), right.split(",").filter_map(|x| x.parse::<u32>().ok()).collect::<VecDeque<_>>())
            })
        }).map(|(mut it, mut left)| {
            solve(&mut it, None, &mut left)
        }).sum::<u32>());
}

fn solve(it: &mut VecDeque<u8>, cur: Option<u32>, left: &mut VecDeque<u32>) -> u32 {
    if let Some(item) = it.pop_front() {
        let res = match item {
            b'.' => {
                match cur {
                    Some(0) | None => solve(it, None, left),
                    _ => 0,
                }
            }
            b'#' => {
                match cur {
                    None => {
                        if let Some(l) = left.pop_front() {
                            let res = solve(it, Some(l - 1), left);
                            left.push_front(l);
                            res
                        } else {
                            0
                        }
                    }
                    Some(0) => 0,
                    Some(val) => {
                        solve(it, Some(val - 1), left)
                    }
                }
            }
            b'?' => {
                it.push_front(b'#');
                let first = solve(it, cur, left);
                it.pop_front();
                it.push_front(b'.');
                let second = solve(it, cur, left);
                it.pop_front();
                first + second
            }
            _ => panic!("unexpected token")
        };
        it.push_front(item);
        res
    } else {
        match (cur, left.is_empty()) {
            (None, true) | (Some(0), true) => 1,
            _ => 0,
        }
    }
}
