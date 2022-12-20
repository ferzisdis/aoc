use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
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

fn main() {
    let mut inputs = BufReader::new(File::open("day18.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|s| {
            let mut it = s.into_bytes().into_iter();
            (read_int(&mut it), read_int(&mut it), read_int(&mut it))
        });

    let mut hs_sides = HashMap::new();
    let mut hs_cubes = HashSet::new();

    for cube in inputs {
        hs_sides.remove(&cube);
        hs_cubes.insert(cube);

        for add in vec![(-1,0,0),(1,0,0),(0,-1,0),(0,1,0),(0,0,-1),(0,0,1)] {
            let (xc,yc,zc) = cube;
            let (xs, ys, zs) = add;
            let side = (xc+xs, yc+ys, zc+zs);
            if !hs_cubes.contains(&side) {
                hs_sides.insert(side, match hs_sides.get(&side) {
                    Some(v) => *v + 1,
                    None => 1,
                });
            }
        }
    }

    println!("res {}", hs_sides.values().sum::<i32>())
}

