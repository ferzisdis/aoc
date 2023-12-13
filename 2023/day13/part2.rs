use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::ops::ControlFlow;

fn main() {
    let mut lines = BufReader::new(File::open("day13.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            x.bytes().collect::<Vec<_>>()
        });

    let mut res = 0;
    let mut by_rows = Vec::new();
    let mut by_cols = Vec::new();

    loop {
        let line = lines.next();

        if line.as_ref().map(|l| l.len()).unwrap_or(0) == 0 {
            if let Some(val) = find_mirror(&by_cols) {
                res += val
            } else {
                by_cols.reverse();
                if let Some(val) = find_mirror(&by_cols) {
                    res += by_cols.len() - val
                }
            }
            if let Some(val) = find_mirror(&by_rows) {
                res += 100 * val;
            } else {
                by_rows.reverse();
                if let Some(val) = find_mirror(&by_rows) {
                    res += 100 * (by_rows.len() - val);
                }
            }

            by_cols.clear();
            by_rows.clear();
            if let None = line {
                break
            } else {
                continue
            }
        }

        if let Some(line) = line {
            if by_cols.len() == 0 {
                by_cols.resize(line.len(), Vec::new());
            }
            for (col, val) in by_cols.iter_mut().zip(line.iter()) {
                col.push(*val);
            }
            by_rows.push(line)
        }
    }

    println!("{}", res)
}

fn find_mirror(map: &Vec<Vec<u8>>) -> Option<usize> {
    let first_line = map.first().unwrap();
    for (idx, line) in map.iter().enumerate() {
        if idx == 0 || idx % 2 != 1 {
            continue
        }
        if first_line.iter().zip(line).map(|(lhs, rhs)| if lhs == rhs { 0 } else { 1 }).sum::<i32>() < 2 {
            let weight = (idx + 1) / 2;
            let changes =
                map.iter().take(weight).zip(map.iter().skip(weight)
                    .take(weight).rev()).map(|(lhs, rhs)| lhs.iter()
                        .zip(rhs).map(|(l, r)| if l == r { 0 } else { 1 }).sum::<i32>())
                    .sum::<i32>();
            if changes == 1 {
                return Some(weight)
            }
        }
    }

    None
}
