use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::ControlFlow;

fn main() {
    let mut lines = BufReader::new(File::open("day11.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"));

    let mut row_presence = Vec::new();
    let mut col_presence = Vec::new();
    let mut galaxies = Vec::new();
    for (i, line) in lines.enumerate() {
        if col_presence.len() == 0 {
            col_presence.resize(line.len(), false);
        }
        row_presence.push(false);

        for (j, sign) in line.bytes().enumerate() {
            match sign {
                b'#' => {
                    *row_presence.get_mut(i).unwrap() = true;
                    *col_presence.get_mut(j).unwrap() = true;
                    galaxies.push((i, j));
                },
                _ => {}
            }
        }
    }

    let row_lens: Vec<i32> = row_presence.into_iter()
        .fold(Vec::new(), |mut acc, x| {
            let prev = acc.last().map(|x| *x).unwrap_or(0);
            acc.push(prev + if x { 1 } else { 1000000 });
            acc
        });

    let col_lens: Vec<i32> = col_presence.into_iter()
        .fold(Vec::new(), |mut acc, x| {
            let prev = acc.last().map(|x| *x).unwrap_or(0);
            acc.push(prev + if x { 1 } else { 1000000 });
            acc
        });

    println!("{}", galaxies.iter().enumerate().flat_map(|(i, lhs)| {
        galaxies.iter().skip(i + 1).map(|rhs| (*lhs, *rhs))
    }).map(|((lhs_i, lhs_j), (rhs_i, rhs_j))| {
        ((*row_lens.get(lhs_i).unwrap() - *row_lens.get(rhs_i).unwrap()).abs()
         + (*col_lens.get(lhs_j).unwrap() - *col_lens.get(rhs_j).unwrap()).abs()) as i64
    }).sum::<i64>())
}
