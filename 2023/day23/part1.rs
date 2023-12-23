use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;


fn main() {
    let mut map = BufReader::new(File::open("day23.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            x.bytes().collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    println!("{}", dfs(&mut map, (0, 1)).unwrap() - 1);
}


fn dfs(map: &mut Vec<Vec<u8>>, (pos_i, pos_j): (i32, i32)) -> Option<usize> {

    match map.get(pos_i as usize).and_then(|row| row.get(pos_j as usize)).map(|x| *x) {
        None | Some(b'#') | Some(b'O') => None,
        Some(b) => {
            let moves = match b {
                b'.' => vec![(1,0), (-1,0), (0,1), (0,-1)],
                b'>' => vec![(0, 1)],
                b'v' => vec![(1, 0)],
                b'<' => vec![(0, -1)],
                b'^' => vec![(-1, 0)],
                _ => panic!("unexpected move")
            };
            if pos_i as usize == map.len() - 1 {
                return Some(1);
            }

            let mut res: Option<usize> = None;
            *map.get_mut(pos_i as usize).unwrap().get_mut(pos_j as usize).unwrap() = b'O';
            for (add_i, add_j) in moves {
                if let Some(m) = dfs(map, (pos_i + add_i, pos_j + add_j)) {
                    res = res.map(|x| x.max(m + 1)).or(Some(m + 1))
                }
            }
            *map.get_mut(pos_i as usize).unwrap().get_mut(pos_j as usize).unwrap() = b;
            res
        }
    }
}
