use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

fn read_int<T>(it: &mut T) -> i32
    where T : Iterator<Item = u8>
{
    let mut res = 0;
    for b in it {
        match b {
            b'0'..=b'9' => {
                res = res * 10 + i32::from(b - b'0')
            }
            _ => break
        }
    }

    return  res;
}

fn main() {
    println!("{}",  BufReader::new(File::open("day4.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|s| {
            let mut it = s.into_bytes().into_iter();
            let l1 = read_int(&mut it);
            let r1 = read_int(&mut it);
            let l2 = read_int(&mut it);
            let r2 = read_int(&mut it);

            if l1 <= l2 && r2 <= r1 || l2 <= l1 && r1 <= r2 {
                1
            } else {
                0
            }
        }).sum::<i32>());
}
