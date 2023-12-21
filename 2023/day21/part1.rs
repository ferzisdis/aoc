use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("day21.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"));

    let mut start = (0, 0);
    let mut plots = HashSet::new();
    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (i as i32, j as i32);
                plots.insert((i, j));
            }
            if ch == '.' {
                plots.insert((i, j));
            }
        }
    }

    let mut moves: HashSet<(i32, i32)> = HashSet::new();
    moves.insert(start);
    for _ in 0..64 {
        moves = moves.into_iter().flat_map(|(move_i, move_j)| {
                vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .into_iter().map(|(add_i, add_j)| (move_i + add_i, move_j + add_j))
                    .collect::<Vec<_>>()
            })
            .filter_map(|(i, j)| plots.get(&(i as usize, j as usize)).map(|_| (i, j)))
            .collect::<HashSet<_>>();
    }

    println!("{}", moves.len())
}
