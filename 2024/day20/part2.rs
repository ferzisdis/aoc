use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let field = BufReader::new(File::open("day20.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.into_bytes())
        .collect::<Vec<_>>();

    let start = field
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, val)| val == &&b'S')
                .map(move |(y, _)| (x as i32, y as i32))
        })
        .next()
        .unwrap();

    let cnt = get_fasters(&field, start, 100);
    println!("{}", cnt);
}

fn get_fasters(field: &Vec<Vec<u8>>, start: (i32, i32), limit: i32) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    let mut visited = HashSet::new();
    let mut path: Vec<(i32, (i32, i32))> = Vec::new();
    let mut res = 0;

    while let Some(Reverse((score, cur))) = heap.pop() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);
        match get_cell(field, cur) {
            None => (),
            Some(&b'#') => (),
            Some(b) if b == &b'S' || b == &b'.' || b == &b'E' => {
                for (prev_score, prev_pos) in &path {
                    let dist = (cur.0 - prev_pos.0).abs() + (cur.1 - prev_pos.1).abs();
                    if dist > 20 {
                        continue;
                    }
                    let profit = (score - prev_score) - dist;
                    if profit >= limit {
                        println!("fount path with profit {}", profit);
                        res += 1;
                    }
                }
                path.push((score, cur));
                if b == &b'E' {
                    return res;
                }
                for add in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let next = (cur.0 + add.0, cur.1 + add.1);
                    heap.push(Reverse((score + 1, next)));
                }
            }
            _ => unreachable!(),
        }
    }

    unreachable!()
}

fn get_cell(field: &Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field.get(x as usize).and_then(|line| line.get(y as usize))
}
