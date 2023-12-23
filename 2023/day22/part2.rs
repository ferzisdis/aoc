use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Eq, PartialEq)]
struct BrickPosition {
    level: usize,
    idx: usize,
}

impl Ord for BrickPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        other.level.cmp(&self.level)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for BrickPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize)
}

fn main() {
    let mut bricks = BufReader::new(File::open("day22.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            let (from, to) = x.split_once('~').unwrap();
            let mut it = from.split(',').filter_map(|x| x.parse::<usize>().ok())
                .zip(to.split(',').filter_map(|x| x.parse::<usize>().ok()))
                .map(|(f, t)| (f.min(t), t.max(f)));

            Brick {
                x: it.next().unwrap(),
                y: it.next().unwrap(),
                z: it.next().unwrap(),
            }
        }).collect::<Vec<_>>();

    let (max_x, max_y, max_z) = (
        bricks.iter().map(|b| b.x.1).max().unwrap(),
        bricks.iter().map(|b| b.y.1).max().unwrap(),
        bricks.iter().map(|b| b.z.1).max().unwrap());

    let mut map = Vec::new();
    let mut map_y = Vec::new();
    map_y.resize(max_y + 1, (0usize, -1i32));
    map.resize(max_x + 1, map_y);

    bricks.sort_by(|lhs, rhs| lhs.z.0.partial_cmp(&rhs.z.0).unwrap());

    let mut forward_index = HashMap::new();
    let mut backward_index = HashMap::new();
    for z in (0..bricks.len()) {
        forward_index.insert(z as i32, HashSet::new());
        backward_index.insert(z, HashSet::new());
    }

    let mut levels = Vec::new();
    levels.resize(bricks.len(), 0);

    for (idx, brick) in bricks.iter().enumerate() {
        let max_z = (brick.x.0..=brick.x.1).flat_map(|x| (brick.y.0..=brick.y.1).map(move |y| (x, y)))
            .map(|(x, y)| map.get(x).unwrap().get(y).unwrap().0).max().unwrap();

        for (x, y) in (brick.x.0..=brick.x.1).flat_map(|x| (brick.y.0..=brick.y.1).map(move |y| (x, y))) {
            let (last_z, last_idx) = map.get_mut(x).unwrap().get_mut(y).unwrap();

            if *last_z == max_z && *last_idx != -1 {
                forward_index.get_mut(&last_idx).unwrap().insert(idx);
                backward_index.get_mut(&idx).unwrap().insert(last_idx.clone());
            }

            *last_idx = idx as i32;
            *last_z = max_z + brick.z.1 - brick.z.0 + 1
        }

        *levels.get_mut(idx).unwrap() = max_z + 1;
    }

    let mut res = 0;
    for (idx, _) in levels.iter().enumerate() {
        let mut heap = BinaryHeap::new();
        let mut falls = HashSet::new();
        falls.insert(idx);
        for forw_idx in forward_index.get(&(idx as i32)).unwrap() {
            heap.push(BrickPosition { level: *levels.get(*forw_idx).unwrap(), idx: *forw_idx});
        }

        while let Some(BrickPosition { level, idx }) = heap.pop() {
            if falls.contains(&idx) {
                continue
            }

            if !backward_index.get(&idx).unwrap().iter().all(|back_idx| falls.contains(&(*back_idx as usize))) {
                continue
            }

            falls.insert(idx);
            for forw_idx in forward_index.get(&(idx as i32)).unwrap() {
                heap.push(BrickPosition { level: *levels.get(*forw_idx).unwrap(), idx:  *forw_idx });
            }
        }

        res += falls.len() - 1;
    }

    println!("{}", res);
}
