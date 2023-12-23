use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

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
    }

    let res = forward_index.into_iter()
        .filter(|(key, vals)| vals.iter().all(|x| backward_index.get(x).unwrap().len() > 1))
        .count();

    println!("{}", res);
}
