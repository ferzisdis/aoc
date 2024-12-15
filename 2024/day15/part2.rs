use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    str::from_utf8,
};

fn main() {
    let mut it = BufReader::new(File::open("day15.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap());

    let mut field = Vec::new();
    while let Some(line) = it.next() {
        if line.len() == 0 {
            break;
        }
        field.push(
            line.into_bytes()
                .into_iter()
                .flat_map(|x| match x {
                    b'#' => [b'#', b'#'],
                    b'.' => [b'.', b'.'],
                    b'O' => [b'[', b']'],
                    b'@' => [b'@', b'.'],
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut start = field
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, val)| val == &&b'@')
                .map(move |(y, _)| (x as i32, y as i32))
        })
        .next()
        .unwrap();

    for m in it.flat_map(|line| line.into_bytes()) {
        let v = match m {
            b'^' => (-1, 0),
            b'>' => (0, 1),
            b'v' => (1, 0),
            b'<' => (0, -1),
            _ => unreachable!(),
        };

        start = match if v.0 == 0 {
            try_move_horisontal(&mut field, start, v)
        } else {
            try_move_vertical(&mut field, start, v)
        } {
            Some(next) => next,
            None => start,
        };
    }

    let res = field
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, val)| val == &&b'[')
                .map(move |(y, _)| 100 * x + y)
        })
        .sum::<usize>();

    for line in field {
        println!("{}", from_utf8(&line).unwrap())
    }
    println!("{}", res)
}

fn try_move_vertical(
    field: &mut Vec<Vec<u8>>,
    start: (i32, i32),
    v: (i32, i32),
) -> Option<(i32, i32)> {
    let mut st = VecDeque::new();
    st.push_back(start);
    let mut visited = HashSet::new();
    let mut updates = Vec::new();
    while let Some(cur) = st.pop_front() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        let next = (cur.0 + v.0, cur.1 + v.1);
        match get_cell(field, next).unwrap().clone() {
            b'#' => return None,
            b'.' => {
                updates.push((cur, next));
            }
            b'[' => {
                updates.push((cur, next));
                st.push_back((next.0, next.1 + 1));
                st.push_back(next);
            }
            b']' => {
                updates.push((cur, next));
                st.push_back((next.0, next.1 - 1));
                st.push_back(next);
            }
            _ => unreachable!(),
        }
    }

    updates.reverse();
    for (cur, next) in updates {
        let prev = get_cell(field, next).unwrap().clone();
        *get_cell_mut(field, next).unwrap() = get_cell(field, cur).unwrap().clone();
        *get_cell_mut(field, cur).unwrap() = prev;
    }

    return Some((start.0 + v.0, start.1 + v.1));
}

fn try_move_horisontal(
    field: &mut Vec<Vec<u8>>,
    cur: (i32, i32),
    v: (i32, i32),
) -> Option<(i32, i32)> {
    let next = (cur.0 + v.0, cur.1 + v.1);
    match get_cell(field, next).unwrap().clone() {
        b'.' => {
            *get_cell_mut(field, next).unwrap() = get_cell(field, cur).unwrap().clone();
            *get_cell_mut(field, cur).unwrap() = b'.';
            Some(next)
        }
        b'[' | b']' => match try_move_horisontal(field, next, v) {
            Some(_) => {
                *get_cell_mut(field, next).unwrap() = get_cell(field, cur).unwrap().clone();
                *get_cell_mut(field, cur).unwrap() = b'.';
                Some(next)
            }
            None => None,
        },
        b'#' => None,
        unknown => unreachable!("unknown '{}'", unknown),
    }
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
