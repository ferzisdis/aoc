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
    head: Coord,
    tail: Coord,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Coord {x: 0, y: 0},
            tail: Coord {x: 0, y: 0},
        }
    }

    fn shrink(&mut self) -> Coord {
        if (self.head.y - self.tail.y).abs() > 1 || (self.head.x - self.tail.x).abs() > 1 {
            if (self.head.y - self.tail.y).abs() > 0 {
                self.tail.y += (self.head.y - self.tail.y) / (self.head.y - self.tail.y).abs()
            }
            if (self.head.x - self.tail.x).abs() > 0 {
                self.tail.x += (self.head.x - self.tail.x) / (self.head.x - self.tail.x).abs()
            }
        }

        self.tail
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
                b'U' => rope.head.y -= 1,
                b'D' => rope.head.y += 1,
                b'L' => rope.head.x -= 1,
                b'R' => rope.head.x += 1,
                _ => panic!("unexpect other")
            }
            path.insert(rope.shrink());
            return rope;
        });

    println!("{}", path.len());
}
