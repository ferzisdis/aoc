use core::time;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    thread,
};

fn main() {
    let field = (101, 103);
    let robots = BufReader::new(File::open("day14.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split_once(" ")
                .map(|(pos, vel)| {
                    (
                        pos.split_once("=")
                            .unwrap()
                            .1
                            .split_once(",")
                            .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                            .unwrap(),
                        vel.split_once("=")
                            .unwrap()
                            .1
                            .split_once(",")
                            .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                            .unwrap(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    for second in 0..=20000 {
        // thread::sleep(time::Duration::from_secs(1));

        let positions = robots
            .iter()
            .map(|(pos, vel)| {
                (
                    (pos.0 + second * vel.0).rem_euclid(field.0),
                    (pos.1 + second * vel.1).rem_euclid(field.1),
                )
            })
            .collect::<HashSet<_>>();

        if !all_connected(&positions) {
            continue;
        }

        println!("second {}", second);
        for y in 0..=field.1 {
            let line = (0..=field.0)
                .map(|x| {
                    if positions.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>();

            println!("{}", line);
        }
    }
}

fn all_connected(positions: &HashSet<(i64, i64)>) -> bool {
    let mut visited = HashSet::new();
    let mut st = Vec::new();

    for pos in positions.iter() {
        st.clear();
        st.push(pos.clone());
        let visited_before = visited.len();
        while let Some(cur) = st.pop() {
            if !positions.contains(&cur) {
                continue;
            }
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur);
            for add in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, -1),
            ] {
                st.push((cur.0 + add.0, cur.1 + add.1));
            }
        }

        // println!("visited {}=={}", visited.len(), positions.len());
        if visited.len() - visited_before > 100 {
            return true;
        }
    }
    false
}
