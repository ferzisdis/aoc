use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Top, Right, Bottom, Left
}

fn main() {
    let map = BufReader::new(File::open("day16.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| x.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, Direction::Right));
    let mut visited = HashSet::new();

    while let Some((i, j, dir)) = queue.pop_back() {
        if let Some(b) = map.get(i as usize).map(|x| x.get(j as usize)).unwrap_or(None) {
            if !visited.contains(&(i, j, dir)) {
                visited.insert((i, j, dir));
            } else {
                continue
            }

            match (dir, *b) {
                (Direction::Right, b'.' | b'-')=> { queue.push_back((i, j + 1, Direction::Right)) },
                (Direction::Right, b'/') | (Direction::Left, b'\\') => { queue.push_back((i - 1, j, Direction::Top)) },
                (Direction::Right, b'\\') | (Direction::Left, b'/') => { queue.push_back((i + 1, j, Direction::Bottom)) },
                (Direction::Left, b'.' | b'-')=> { queue.push_back((i, j - 1, Direction::Left)) },
                (Direction::Right | Direction::Left, b'|') => {
                    queue.push_back((i - 1, j, Direction::Top));
                    queue.push_back((i + 1, j, Direction::Bottom))
                },

                (Direction::Top, b'.' | b'|')=> { queue.push_back((i - 1, j, Direction::Top)) },
                (Direction::Top, b'/') | (Direction::Bottom, b'\\') => { queue.push_back((i, j + 1, Direction::Right)) },
                (Direction::Top, b'\\') | (Direction::Bottom, b'/') => { queue.push_back((i, j - 1, Direction::Left)) },
                (Direction::Bottom, b'.' | b'|') => { queue.push_back((i + 1, j, Direction::Bottom)) },
                (Direction::Top | Direction::Bottom, b'-') => {
                    queue.push_back((i, j + 1, Direction::Right));
                    queue.push_back((i, j - 1, Direction::Left));
                },
                _ => panic!("unexpected combination {:?} {}", dir, char::from(*b))
            }
        }
    }

    println!("{}", visited.iter().map(|(i, j, _)| (i, j)).collect::<HashSet<_>>().len())
}
