use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;

fn read_int(it: &mut Peekable<impl Iterator<Item = u8>>) -> i32
{
    let mut res: i32 = 0;
    let mut is_neg = false;
    loop {
        match it.peek().clone() {
            Some(b'0'..=b'9') => {
                res = res * 10 + i32::from(it.next().expect("should exist") - b'0');
            }
            Some(b'-') => { is_neg = true }
            _ => break
        }
    }
    return if is_neg { -res } else { res };
}

fn read_n(it: &mut impl Iterator<Item = u8>, n: usize) -> String {
    let mut s = String::new();
    s.reserve(n);

    for _ in 0..n {
        if let Some(b) = it.next() {
            s.push(char::from(b));
        } else {
            panic!("Not enough length: {}", s)
        }
    }

    s
}

fn mir(x: i32) -> i32 {
    x + 50 - (2 * (x % 50) + 1)
}

enum Ground {
    Empty,
    Free,
    Wall
}

fn go((x, y): (i32, i32), mut dir: i32, map: &Vec<Vec<Ground>>) -> Option<(i32, i32, i32)> {
    let (add_x, add_y) = match dir {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => panic!("invalid direction")
    };

    let mut x = x;
    let mut y = y;
    loop {
        x = x + add_x;
        y = y + add_y;

        let variants = match dir {
            0 => {
                vec![
                    (y, x, 0),
                    (y - (y % 50) - 1, x + (y % 50), 3),
                    (mir(y) + 100, x - 50 - 1, 2),
                    (mir(y) - 100, x + 50 - 1, 2),
                ]
            },
            1 => {
                vec![
                    (y, x, 1),
                    (y + (x % 50), x - (x % 50) - 1, 2),
                    (y - 200, x + 100, 1),
                ]
            },
            2 => {
                vec![
                    (y, x, 2),
                    (y - (y % 50) + 50, x - 50 + (y % 50) + 1, 1),
                    (y - (y % 50) - 150, x + 50 + (y % 50) + 1, 1),
                    (mir(y) + 100, x - 50 + 1, 0),
                    (mir(y) - 100, x + 50 + 1, 0),
                ]
            },
            3 => {
                vec![
                    (y, x, 3),
                    (y + (x % 50) - 50 + 1, x - (x % 50) + 50, 0),
                    (y + (x % 50) + 150 + 1, x - (x % 50) - 50, 0),
                    (y + 200, x - 100, 3),
                ]
            },
            _ => panic!("invalid direction")
        };

        for (y, x, dir) in variants {
            if x < 0 || y < 0 {
                continue
            }
            match map.get(y as usize).and_then(|row| row.get(x as usize)) {
                Some(Ground::Free) => return Some((x, y, dir)),
                Some(Ground::Wall) => return None,
                _ => {
                    continue
                }
            }
        }

        panic!("impossible {:?}", (x, y, dir))
    }
}

fn main() {
    let mut inputs = BufReader::new(File::open("day22.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"));

    let mut map = Vec::new();
    loop {
        let s = inputs.next().expect("expect non-empty line");
        if s == "" {
            break
        }
        let mut it = s.into_bytes().into_iter();
        let mut vec = Vec::new();
        loop {
            match it.next() {
                Some(b' ') => vec.push(Ground::Empty),
                Some(b'.') => vec.push(Ground::Free),
                Some(b'#') => vec.push(Ground::Wall),
                _ => break
            }
        }
        map.push(vec);
    }

    assert_eq!(go((52, 0), 3, &map), Some((0, 152, 0)));
    assert_eq!(go((102, 0), 3, &map), Some((2, 199, 3)));
    assert_eq!(go((149, 2), 0, &map), Some((99, 147, 2)));
    assert_eq!(go((103, 49), 1, &map), Some((99, 53, 2)));
    assert_eq!(go((99, 53), 0, &map), Some((103, 49, 3)));
    assert_eq!(go((99, 147), 0, &map), Some((149, 2, 2)));
    assert_eq!(go((54, 149), 1, &map), Some((49, 154, 2)));
    assert_eq!(go((49, 154), 0, &map), Some((54, 149, 3)));
    assert_eq!(go((2, 199), 1, &map), Some((102, 0, 1)));
    //
    assert_eq!(go((0, 152), 2, &map), Some((52, 0, 1)));
    assert_eq!(go((0, 103), 2, &map), Some((50, 46, 0)));
    assert_eq!(go((2, 100), 3, &map), Some((50, 52, 0)));
    assert_eq!(go((50, 52), 2, &map), Some((2, 100, 1)));
    assert_eq!(go((50, 46), 2, &map), Some((0, 103, 0)));

    let mut it = inputs.next().expect("not found path").into_bytes().into_iter().peekable();
    let mut direction = 0;
    let mut pos = (50, 0);
    loop {
        let steps = read_int(&mut it);
        for _ in 0..steps {
            match go(pos, direction, &map) {
                Some((new_x, new_y, new_dir)) => {
                    pos = (new_x, new_y);
                    direction = new_dir;
                },
                None => break
            }
        }

        match it.next() {
            Some(b'R') => { direction = (direction + 1).rem_euclid(4) },
            Some(b'L') => { direction = (direction - 1).rem_euclid(4) },
            _ => break
        }
        println!("after {} steps pos = {:?}, dir = {}", steps, pos, direction);
    }

    println!("pos = {:?}, dir = {}", pos, direction);
    let (x, y) = pos;
    println!("res = {}", (y + 1) * 1000 + (x + 1) * 4 + direction);
}
