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
    heap.push(Reverse((0, start, None)));
    let mut visited = HashSet::new();
    let mut results = Vec::new();

    while let Some(Reverse((score, cur, cheat))) = heap.pop() {
        if visited.contains(&(cur, cheat)) || visited.contains(&(cur, Option::None)) {
            continue;
        }
        visited.insert((cur, cheat));
        match get_cell(field, cur) {
            None => (),
            Some(&b'#') => match cheat {
                None => {
                    for add in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let next = (cur.0 + add.0, cur.1 + add.1);
                        heap.push(Reverse((score + 1, next, Some((cur, next)))));
                    }
                }
                Some(_) => (),
            },
            Some(&b'E') => match cheat {
                None => {
                    let mut cnt = 0;
                    for r in results {
                        println!("faster on {} sec", score - r);
                        if score - r >= limit {
                            cnt += 1;
                        }
                    }
                    return cnt;
                }
                Some(_) => {
                    println!("finish at {} heap size {}", score, heap.len());
                    results.push(score);
                }
            },
            Some(&b'S' | &b'.') => {
                for add in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let next = (cur.0 + add.0, cur.1 + add.1);
                    heap.push(Reverse((score + 1, next, cheat)));
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
