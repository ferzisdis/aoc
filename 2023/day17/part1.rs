use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Top, Right, Bottom, Left
}

fn main() {
    let map = BufReader::new(File::open("day17.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| x.chars().filter_map(|ch| ch.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (width, height) = (map.first().map(|x| x.len()).unwrap(), map.len());

    let mut queue = VecDeque::new();
    queue.push_back((0i32, 0i32, Direction::Right, 0, 0));
    let mut visited = HashMap::new();

    while let Some((i, j, dir, step, mut cost)) = queue.pop_front() {
        if let Some(digit) = map.get(i as usize).map(|x| x.get(j as usize)).unwrap_or(None) {
            if step > 3 {
                continue
            }
            if step != 0 {
                cost += digit
            }

            if visited.get(&(i, j, dir, step)).map(|v| cost < *v).unwrap_or(true) {
                visited.insert((i, j, dir, step), cost);
            } else {
                continue
            }

            match dir {
                Direction::Right | Direction::Left => {
                    queue.push_back((i - 1, j, Direction::Top, 1, cost));
                    queue.push_back((i + 1, j, Direction::Bottom, 1, cost));
                },
                Direction::Top | Direction::Bottom => {
                    queue.push_back((i, j - 1, Direction::Left, 1, cost));
                    queue.push_back((i, j + 1, Direction::Right, 1, cost));
                }
            }

            match dir {
                Direction::Top => queue.push_back((i - 1, j, Direction::Top, step + 1, cost)),
                Direction::Right => queue.push_back((i, j + 1, Direction::Right, step + 1, cost)),
                Direction::Bottom => queue.push_back((i + 1, j, Direction::Bottom, step + 1, cost)),
                Direction::Left => queue.push_back((i, j - 1, Direction::Left, step + 1, cost))
            }
        }
    }

    println!("{}", vec![Direction::Top, Direction::Bottom, Direction::Left, Direction::Right]
        .into_iter()
        .flat_map(|dir| {
            vec![1, 2, 3].into_iter()
                .filter_map(|step| visited.get(&((height - 1) as i32, (width - 1) as i32, dir, step)))
                .map(|x| *x).collect::<Vec<_>>()
        })
        .min().unwrap_or(0))
}
