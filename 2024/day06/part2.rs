use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut field = BufReader::new(File::open("day06.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = field
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, byte)| byte == &&b'^')
                .map(|(col, _)| (row as i32, col as i32))
                .collect::<Vec<_>>()
        })
        .into_iter()
        .next()
        .unwrap();

    let mut cur = start;
    let mut direction = 0;
    let mut traps = HashSet::new();
    let mut path = HashSet::new();

    while get_cell(&field, cur).is_some() {
        let next = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .get(direction)
            .map(|add| (cur.0 + add.0, cur.1 + add.1))
            .unwrap();
        if get_cell(&field, next).map(|x| x == &b'#').unwrap_or(false) {
            direction = (direction + 1).rem_euclid(4)
        } else {
            if !path.contains(&next) && get_cell(&field, next) == Some(&b'.') {
                *get_cell_mut(&mut field, next).unwrap() = b'#';
                if is_trapped(&field, cur, direction) {
                    println!("inserted ({},{})", next.0, next.1);
                    traps.insert(next);
                }
                *get_cell_mut(&mut field, next).unwrap() = b'.';
            }
            path.insert(cur);
            cur = next
        }
    }

    println!("{}", traps.len());
}

fn is_trapped(field: &Vec<Vec<u8>>, start: (i32, i32), mut dir: usize) -> bool {
    let mut visited = HashSet::new();
    let mut cur = start;

    while get_cell(&field, cur).is_some() {
        if !visited.insert((cur, dir)) {
            return true;
        }
        let next = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .get(dir)
            .map(|add| (cur.0 + add.0, cur.1 + add.1))
            .unwrap();
        if get_cell(&field, next).map(|x| x == &b'#').unwrap_or(false) {
            dir = (dir + 1).rem_euclid(4)
        } else {
            cur = next
        }
    }
    false
}

fn get_cell(field: &Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field.get(x as usize).and_then(|line| line.get(y as usize))
}

fn get_cell_mut(field: &mut Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&mut u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field
        .get_mut(x as usize)
        .and_then(|line| line.get_mut(y as usize))
}
