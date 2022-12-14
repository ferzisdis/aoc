use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;

fn read_int(it: &mut impl Iterator<Item = u8>) -> i32
{
    let mut res: i32 = 0;
    for b in it {
        match b {
            b'0'..=b'9' => {
                res = res * 10 + i32::from(b - b'0')
            }
            _ => break
        }
    }
    return res;
}

fn main() {
    let mut hs = HashSet::new();
    let it = BufReader::new(File::open("day14.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"));
    let mut bottom_line = 0;

    for s in it {
        let mut it = s.into_bytes().into_iter();
        let mut start = (read_int(&mut it), read_int(&mut it));
        loop {
            if let None = it.next() {
                break;
            }
            it.next(); it.next(); // skip "> "

            let (ex, ey) = (read_int(&mut it), read_int(&mut it));
            match start {
                (sx, sy) if sx == ex => {
                    for y in sy.min(ey)..=sy.max(ey) {
                        hs.insert((sx, y));
                    }
                    bottom_line = bottom_line.max(sy.max(ey));
                },
                (sx, sy) if sy == ey => {
                    for x in sx.min(ex)..=sx.max(ex) {
                        hs.insert((x, sy));
                    }
                    bottom_line = bottom_line.max(sy);
                },
                (sx, sy) => panic!("impossible ({},{}) => ({},{})", sx, sy, ex, ex)
            }
            start = (ex, ey);
        }
    }

    let mut deq = VecDeque::new();
    deq.push_back((500, 0));

    let mut res = 0;
    loop {
        match deq.back() {
            Some(last) => {
                let (x, y) = last;
                println!("process ({}, {})", *x, *y);
                let current = match last {
                    (_, y) if *y == bottom_line => break,
                    (x, y) if !hs.contains(&(*x, *y + 1)) => Some((*x, *y + 1)),
                    (x, y) if !hs.contains(&(*x - 1, *y + 1)) => Some((*x - 1, *y + 1)),
                    (x, y) if !hs.contains(&(*x + 1, *y + 1)) => Some((*x + 1, *y + 1)),
                    _ => None
                };
                match current {
                    Some(coord) => {
                        deq.push_back(coord)
                    },
                    None => {
                        deq.pop_back().and_then(|last| {
                            hs.insert(last);
                            res += 1;
                            None::<()>
                        });
                    }
                }
            },
            _ => panic!("possible, but not real")
        }
    }

    println!("res = {}", res)
}
