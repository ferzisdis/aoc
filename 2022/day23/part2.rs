use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;

fn main() {
    let mut inputs = BufReader::new(File::open("day23.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"));

    let mut elves = HashSet::new();
    for (y, s) in inputs.enumerate() {
        for (x, b) in s.into_bytes().into_iter().enumerate() {
            if b == b'#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let mut dir = VecDeque::from(vec![b'N', b'S', b'W', b'E']);
    for idx in 0.. {
        println!("stage {}", idx);

        let mut next = HashMap::new();
        for (x, y) in elves.iter() {
            let mut new_pos = (*x, *y);

            if elves.contains(&(*x - 1, *y - 1)) ||
               elves.contains(&(*x, *y - 1)) ||
               elves.contains(&(*x + 1, *y - 1)) ||
               elves.contains(&(*x + 1, *y)) ||
               elves.contains(&(*x + 1, *y + 1)) ||
               elves.contains(&(*x, *y + 1)) ||
               elves.contains(&(*x - 1, *y + 1)) ||
               elves.contains(&(*x - 1, *y)) {
                for d in dir.iter() {
                    let m = match d {
                        b'N' if !elves.contains(&(*x - 1, *y - 1)) &&
                            !elves.contains(&(*x, *y - 1)) &&
                            !elves.contains(&(*x + 1, *y - 1)) => Some((*x, *y - 1)),
                        b'S' if !elves.contains(&(*x - 1, *y + 1)) &&
                            !elves.contains(&(*x, *y + 1)) &&
                            !elves.contains(&(*x + 1, *y + 1)) => Some((*x, *y + 1)),
                        b'W' if !elves.contains(&(*x - 1, *y - 1)) &&
                            !elves.contains(&(*x - 1, *y)) &&
                            !elves.contains(&(*x - 1, *y + 1)) => Some((*x - 1, *y)),
                        b'E' if !elves.contains(&(*x + 1, *y - 1)) &&
                            !elves.contains(&(*x + 1, *y)) &&
                            !elves.contains(&(*x + 1, *y + 1)) => Some((*x + 1, *y)),
                        _ => None
                    };
                    if let Some(m) = m {
                        new_pos = m;
                        break
                    }
                }
            }

            if !next.contains_key(&new_pos) {
                next.insert(new_pos, Vec::new());
            };

            (*next.get_mut(&new_pos).expect("insert previously")).push((*x, *y));
        }

        elves.clear();
        let mut moved = false;
        for (k, v) in next {
            if v.len() == 1 {
                if k != v[0] {
                    moved = true;
                }
                elves.insert(k);
            } else {
                for prev in v {
                    elves.insert(prev);
                }
            }
        }

        if !moved {
            break;
        }

        let d = dir.pop_front().expect("known size");
        dir.push_back(d);
    }

    let mut res = 0;

    for y in elves.iter().map(|(_, y)| *y).min().expect("> 0")..=elves.iter().map(|(_, y)| *y).max().expect("> 0") {
        for x in elves.iter().map(|(x, _)| *x).min().expect("> 0")..=elves.iter().map(|(x, _)| *x).max().expect("> 0") {
            if !elves.contains(&(x, y)) {
                print!(".");
                res += 1;
            } else {
                print!("#");
            }
        }
        println!()
    }

    println!("res {}", res);
}
