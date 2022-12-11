use std::{env, fs, usize};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::{Peekable, TakeWhile};
use std::ops::IndexMut;

fn read_usize(it: &mut impl Iterator<Item = u8>) -> usize
{
    let mut res: usize = 0;
    for b in it {
        match b {
            b'0'..=b'9' => {
                res = res * 10 + usize::from(b - b'0')
            }
            _ => break
        }
    }
    return res;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

struct Rope {
    knots: Vec<Coord>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            knots: (0..10).map(|_| Coord {x: 0, y: 0}).collect::<Vec<Coord>>()
        }
    }

    fn shrink(&mut self) -> Coord {
        let mut it = self.knots.iter_mut();
        let mut knot = *it.next().expect("in vector 10 knots");

        loop {
            if let Some(next_knot) = it.next() {
                if (knot.y - next_knot.y).abs() > 1 || (knot.x - next_knot.x).abs() > 1 {
                    if (knot.y - next_knot.y).abs() > 0 {
                        next_knot.y += (knot.y - next_knot.y) / (knot.y - next_knot.y).abs()
                    }
                    if (knot.x - next_knot.x).abs() > 0 {
                        next_knot.x += (knot.x - next_knot.x) / (knot.x - next_knot.x).abs()
                    }
                    knot = *next_knot
                } else {
                    break
                }
            } else {
                break;
            }
        }

        self.knots[9]
    }

    fn head(&mut self) -> &mut Coord {
        return &mut self.knots[0]
    }
}

fn main() {
    let mut path = HashSet::new();
    let data = BufReader::new(File::open("day9.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("You can do it!"))
        .flat_map(|s| {
            let mut it = s.into_bytes().into_iter();
            let direction = it.next().expect("verified");
            it.next(); // space
            let cnt = read_usize(&mut it);
            (0..cnt).map(move |_| direction)
        })
        .fold(Rope::new(), |mut rope, b| {
            match b {
                b'U' => rope.head().y -= 1,
                b'D' => rope.head().y += 1,
                b'L' => rope.head().x -= 1,
                b'R' => rope.head().x += 1,
                _ => panic!("unexpect other")
            }
            path.insert(rope.shrink());
            return rope;
        });

    println!("{}", path.len());
}
