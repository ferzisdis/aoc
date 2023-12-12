use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::ops::ControlFlow;

fn main() {
    println!("{}", BufReader::new(File::open("day12.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .filter_map(|x| {
            x.split_once(" ").map(|(left, right)| {
                (left.bytes().collect::<VecDeque<_>>(),
                 right.split(",").filter_map(|x| x.parse::<u64>().ok()).collect::<VecDeque<_>>())
            })
        }).map(|(mut it, mut left)| {
            let clone_it = it.clone();
            let clone_left = left.clone();
            for i in 0..4 {
                it.push_back(b'?');
                for val in clone_it.iter() {
                    it.push_back(*val);
                }
                for val in clone_left.iter() {
                    left.push_back(*val);
                }
            }

            println!("{}", String::from_utf8(it.clone().into_iter().collect::<Vec<_>>()).unwrap().as_str());

            solve(&mut it, None, &mut left, &mut HashMap::new()) as u64
        }).sum::<u64>());
}

fn solve(it: &mut VecDeque<u8>, cur: Option<u64>, left: &mut VecDeque<u64>, dp: &mut HashMap<String, u64>) -> u64 {
    let key = format!("{}|{}|{}",
                        String::from_utf8(it.clone().into_iter().collect::<Vec<_>>()).unwrap(),
                        cur.map(|x| x.to_string()).unwrap_or(String::new()),
                        left.iter().fold(String::new(), |mut acc, x| { acc.push_str(x.to_string().as_str()); acc }));
    if let Some(dp_val) = dp.get(&key) {
        return *dp_val
    }

    let res = if let Some(item) = it.pop_front() {
        let res = match item {
            b'.' => {
                match cur {
                    Some(0) | None => solve(it, None, left, dp),
                    _ => 0,
                }
            }
            b'#' => {
                match cur {
                    None => {
                        if let Some(l) = left.pop_front() {
                            let res = solve(it, Some(l - 1), left, dp);
                            left.push_front(l);
                            res
                        } else {
                            0
                        }
                    }
                    Some(0) => 0,
                    Some(val) => {
                        solve(it, Some(val - 1), left, dp)
                    }
                }
            }
            b'?' => {
                it.push_front(b'#');
                let first = solve(it, cur, left, dp);
                it.pop_front();
                it.push_front(b'.');
                let second = solve(it, cur, left, dp);
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
    };

    dp.insert(key, res);
    res
}
