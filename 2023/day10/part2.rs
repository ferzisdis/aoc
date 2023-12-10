use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::ControlFlow;

#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    N, S, E, W
}

fn try_next(map: &Vec<Vec<u8>>, (dir, i, j): (Direction, i32, i32)) -> Option<(Direction, i32, i32)> {
    if i < 0 || j < 0 {
        return None
    }
    return match map.get(i as usize) {
        None => None,
        Some(row) => match row.get(j as usize) {
            None => None,
            Some(sigh) => {
                let new_dir = match (dir, sigh) {
                    (Direction::N, b'|') => Some(Direction::N),
                    (Direction::N, b'F') => Some(Direction::E),
                    (Direction::N, b'7') => Some(Direction::W),
                    (Direction::E, b'-') => Some(Direction::E),
                    (Direction::E, b'J') => Some(Direction::N),
                    (Direction::E, b'7') => Some(Direction::S),
                    (Direction::S, b'|') => Some(Direction::S),
                    (Direction::S, b'L') => Some(Direction::E),
                    (Direction::S, b'J') => Some(Direction::W),
                    (Direction::W, b'-') => Some(Direction::W),
                    (Direction::W, b'F') => Some(Direction::S),
                    (Direction::W, b'L') => Some(Direction::N),
                    _ => None
                };
                new_dir.map(|dir| match dir {
                    Direction::N => (dir, i - 1, j),
                    Direction::S => (dir, i + 1, j),
                    Direction::E => (dir, i, j + 1),
                    Direction::W => (dir, i, j - 1),
                })
            }
        }
    };
}

fn main() {
    let map = BufReader::new(File::open("day10.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| x.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = map.iter().enumerate()
        .try_fold(Option::None, |_: Option<(i32, i32)>, (i, row)|{
            let fold = row.iter().enumerate().try_fold(Option::None, |_: Option<i32>, (j, val)| {
                if *val == b'S' {
                    ControlFlow::Break(Option::Some(j as i32))
                } else {
                    ControlFlow::Continue(Option::None)
                }
            });
            if let ControlFlow::Break(Some(j)) = fold {
                ControlFlow::Break(Some((i as i32, j as i32)))
            } else {
                ControlFlow::Continue(None)
            }
        });

    let mut visited = HashSet::new();
    let mut left_hand = VecDeque::new();

    if let ControlFlow::Break(Some((start_i, start_j))) = start {
        visited.insert((start_i, start_j));

        let mut start_pipes = vec![
            (Direction::N, start_i - 1, start_j),
            (Direction::E, start_i, start_j + 1),
            (Direction::S, start_i + 1, start_j),
            (Direction::W, start_i, start_j - 1),
        ].into_iter().filter_map(|x| {
            try_next(&map, x.clone()).map(|_| x)
        });

        let mut lhs = start_pipes.next().expect("no connected pipes");

        for idx in 1.. {
            let (dest, i, j) = lhs;
            let sign = *map.get(i as usize)
                .map(|x| x.get(j as usize).unwrap_or(&b'.'))
                .unwrap_or(&b'.');
            if sign == b'S' {
                break
            }

            visited.insert((i, j));
            match (&dest, sign) {
                (Direction::N, b'|') => { left_hand.push_back((i, j - 1)) },
                (Direction::N, b'F') => { left_hand.push_back((i, j - 1)); left_hand.push_back((i - 1, j - 1)); left_hand.push_back((i - 1, j)); },
                (Direction::N, b'7') => { },
                (Direction::E, b'-') => { left_hand.push_back((i, j - 1)) },
                (Direction::E, b'J') => { },
                (Direction::E, b'7') => { left_hand.push_back((i - 1, j)); left_hand.push_back((i - 1, j + 1)); left_hand.push_back((i, j + 1)); },
                (Direction::S, b'|') => { left_hand.push_back((i, j + 1)); }
                (Direction::S, b'L') => { },
                (Direction::S, b'J') => { left_hand.push_back((i, j + 1)); left_hand.push_back((i + 1, j + 1)); left_hand.push_back((i + 1, j)); }
                (Direction::W, b'-') => { left_hand.push_back((i + 1, j)); }
                (Direction::W, b'F') => { },
                (Direction::W, b'L') => { left_hand.push_back((i + 1, j)); left_hand.push_back((i + 1, j - 1)); left_hand.push_back((i, j - 1)); },
                _ => ()
            }

            lhs = try_next(&map, (dest, i, j)).expect(format!("found a hole in the pipe {}", idx).as_str());
        }
    }

    let border_len = visited.len();
    println!("left hand cnt: {}, border cnt: {}", left_hand.len(), border_len);

    while !left_hand.is_empty() {
        let cur = left_hand.pop_front().unwrap();
        if cur.0 < 0 || cur.1 < 0
            || map.get(cur.0 as usize).map_or_else(|| true, |x| x.get(cur.1 as usize).is_none()) {
            continue
        }
        if visited.contains(&cur) {
            continue
        }
        visited.insert(cur);

        for add in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
            left_hand.push_back((cur.0 + add.0, cur.1 + add.1));
        }
    }

    for (i, row) in map.iter().enumerate() {
        println!("{}", String::from_utf8(row.iter().enumerate().map(|(j, b)| {
            if visited.contains(&(i as i32, j as i32)) {
                b'0'
            } else {
                b'1'
            }
        }).collect::<Vec<_>>()).unwrap())
    }

    let is_outer = visited.contains(&(0, 0));
    if !is_outer {
        println!("{}", visited.len() - border_len);
    } else {
        println!("{}", map.len() * map.iter().next().unwrap().len() - visited.len())
    }
}
