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
    let mut is_neg = false;
    for b in it {
        match b {
            b'0'..=b'9' => {
                res = res * 10 + i32::from(b - b'0')
            }
            b'-' => { is_neg = true }
            _ => break
        }
    }
    return if is_neg { -res } else { res };
}

fn read_n(it: &mut impl Iterator<Item = u8>, n: usize) {
    for _ in 0..n {
        it.next();
    }
}

fn main() {
    let target_line = 2000000;

    let mut vec = BufReader::new(File::open("day15.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|s| {
            let mut it = s.into_bytes().into_iter();
            read_n(&mut it, "Sensor at x=".len());
            let sensor_x = read_int(&mut it);
            read_n(&mut it, " y=".len());
            let sensor_y = read_int(&mut it);
            read_n(&mut it, " closest beacon is at x=".len());
            let beacon_x = read_int(&mut it);
            read_n(&mut it, " y=".len());
            let beacon_y = read_int(&mut it);

            let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            let to_target = (target_line - sensor_y).abs();

            if to_target <= dist {
                let arm = dist - to_target;
                Some((sensor_x - arm, sensor_x + arm))
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.expect("filtered before"))
        .collect::<Vec<(i32, i32)>>();

    vec.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
    let mut res = 0;
    let mut border = vec.first().map(|(left, _)| *left).expect("many intervals should exist");

    for (left, right) in vec {
        res += (right - left.max(border) + 1).max(0);
        border = (right + 1).max(border);
    }

    res -= 1; // well-known one beacon in row
    println!("res = {}", res)
}
