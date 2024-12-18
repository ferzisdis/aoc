use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let broken_pixels = BufReader::new(File::open("day18.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split_once(",")
                .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
                .unwrap()
        })
        .take(1024)
        .collect::<HashSet<_>>();
    println!("{}", get_min_path(broken_pixels, (70, 70)));
    println!("Finish!");
}

fn get_min_path(broken_pixels: HashSet<(i32, i32)>, end: (i32, i32)) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, (0, 0))));
    let mut visited = HashSet::new();

    while let Some(Reverse((score, cur))) = heap.pop() {
        if cur == end {
            return score;
        }
        if cur.0 < 0 || cur.1 < 0 || cur.0 > end.0 || cur.1 > end.1 {
            continue;
        }
        if broken_pixels.contains(&cur) {
            continue;
        }
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        for add in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next = (cur.0 + add.0, cur.1 + add.1);
            heap.push(Reverse((score + 1, next)));
        }
    }

    unreachable!()
}
