use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::ControlFlow;

#[derive(Eq, PartialEq)]
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

    let mut res = 0;
    if let ControlFlow::Break(Some((start_i, start_j))) = start {
        let mut start_pipes = vec![
            try_next(&map, (Direction::N, start_i - 1, start_j)),
            try_next(&map, (Direction::E, start_i, start_j + 1)),
            try_next(&map, (Direction::S, start_i + 1, start_j)),
            try_next(&map, (Direction::W, start_i, start_j - 1)),
        ].into_iter().filter_map(|x| x);

        let mut lhs = start_pipes.next().expect("no connected pipes");
        let mut rhs = start_pipes.next().expect("expected at least two pipe");

        for idx in 2.. {
            if lhs.1 == rhs.1 && lhs.2 == rhs.2 {
                res = idx;
                break
            }

            lhs = try_next(&map, lhs).expect(format!("found a hole in the pipe {}", idx).as_str());
            rhs = try_next(&map, rhs).expect(format!("found a hole in the pipe {}", idx).as_str());
        }
    }

    println!("{}", res);
}
