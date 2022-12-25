use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;

fn main() {
    let mut inputs = BufReader::new(File::open("day24.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"));


    let mut blizzards = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, s) in inputs.enumerate() {
        for (x, b) in s.into_bytes().into_iter().enumerate() {
            match b {
                b'>' | b'<' | b'^' | b'v' => blizzards.push((x as i32, y as i32, b)),
                _ => ()
            }
            max_x = max_x.max(x as i32);
        }
        max_y = max_y.max(y as i32);
    }

    let start = (1, 0);
    let finish = (max_x - 1, max_y);

    let one = slove(start, finish, (max_x, max_y), &mut blizzards);
    let two = slove(finish, start, (max_x, max_y), &mut blizzards);
    let three = slove(start, finish, (max_x, max_y), &mut blizzards);
    println!("{}+{}+{}={}", one, two, three, one + two + three);
}

fn slove(start: (i32, i32), finish: (i32, i32), (max_x, max_y): (i32, i32), blizzards: &mut Vec<(i32, i32, u8)>) -> i32 {
    let mut cur = HashSet::new();

    cur.insert(start);

    for minute in 1.. {
        let mut blizzards_hs = HashSet::new();
        for (x, y, b) in blizzards.iter_mut() {
            match b {
                b'>' => { *x += 1; if *x == max_x { *x = 1} },
                b'<' => { *x -= 1; if *x == 0 { *x = max_x - 1} },
                b'^' => { *y -= 1; if *y == 0 { *y = max_y - 1 } },
                b'v' => { *y += 1; if *y == max_y { *y = 1 } },
                _ => panic!("unknown direction")
            }
            blizzards_hs.insert((*x, *y));
        }

        let mut next = HashSet::new();
        for (x, y) in cur {
            for (add_x, add_y) in vec![(0, -1), (1, 0), (0, 1), (-1, 0), (0, 0)] {
                let x = x + add_x;
                let y = y + add_y;

                if x == 0 || y == 0 || x == max_x || y == max_y {
                    if finish == (x, y) {
                        println!("finish at {}", minute);
                        return minute;
                    }
                    if start != (x, y) {
                        continue
                    }
                }

                if !blizzards_hs.contains(&(x, y)) {
                    next.insert((x, y));
                }
            }
        }
        cur = next;
    }

    panic!("unreacheble");
}
