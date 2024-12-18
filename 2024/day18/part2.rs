use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use union_find::{QuickUnionUf, UnionBySize, UnionFind};

fn main() {
    let broken_pixels = BufReader::new(File::open("day18.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split_once(",")
                .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let res = get_first_deadlock(broken_pixels, (70, 70));
    println!("{},{}", res.0, res.1);
    println!("Finish!");
}

fn get_first_deadlock(mut broken_pixels: Vec<(i32, i32)>, end: (i32, i32)) -> (i32, i32) {
    let (width, height) = (end.0 + 1, end.1 + 1);
    let mut uf = QuickUnionUf::<UnionBySize>::new(width as usize * height as usize);
    broken_pixels.reverse();

    let mut restored = HashSet::new();
    for x in 0..=end.0 {
        for y in 0..=end.1 {
            let cur = (x, y);
            if broken_pixels.contains(&cur) {
                continue;
            }
            restored.insert((x, y));
            for add in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next = (cur.0 + add.0, cur.1 + add.1);
                if next.0 < 0 || next.1 < 0 || next.0 > end.0 || next.1 > end.1 {
                    continue;
                }
                if restored.contains(&next) {
                    uf.union(
                        (cur.0 * width + cur.1) as usize,
                        (next.0 * width + next.1) as usize,
                    );
                }
            }
        }
    }
    for cur in broken_pixels {
        restored.insert(cur);
        for add in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next = (cur.0 + add.0, cur.1 + add.1);
            if next.0 < 0 || next.1 < 0 || next.0 > end.0 || next.1 > end.1 {
                continue;
            }
            if restored.contains(&next) {
                uf.union(
                    (cur.0 * width + cur.1) as usize,
                    (next.0 * width + next.1) as usize,
                );
            }
        }
        if uf.find(0) == uf.find((end.0 * width + end.1) as usize) {
            return cur;
        }
    }

    unreachable!()
}
