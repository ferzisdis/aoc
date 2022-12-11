use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::{Peekable, TakeWhile};
use std::ops::IndexMut;
use crate::Cmd::{Addx, Noop};

enum Cmd {
    Addx(i32), Noop
}

fn read_to_eol(it: &mut impl Iterator<Item = u8>) {
    loop {
        match it.next() {
            Some(b'\n') | None => break,
            _ => continue
        }
    }
}

fn read_cmd(it: &mut impl Iterator<Item = u8>) -> Option<Cmd> {
    it.next().and_then(|b| {
        match b {
            b'a' => {
                assert_eq!(Some(b'd'), it.next());
                assert_eq!(Some(b'd'), it.next());
                assert_eq!(Some(b'x'), it.next());
                assert_eq!(Some(b' '), it.next());

                let val = read_int(it);
                read_to_eol(it);
                Some(Addx(val))
            }
            b'n' => {
                read_to_eol(it);
                Some(Noop)
            },
            _ => panic!("unexpected format!")
        }
    })
}

fn read_int(it: &mut impl Iterator<Item = u8>) -> i32
{
    let mut res: i32 = 0;
    let mut minus = false;
    for b in it {
        match b {
            b'0'..=b'9' => {
                res = res * 10 + i32::from(b - b'0')
            }
            b'-' => {
                minus = true
            }
            _ => break
        }
    }

    if minus { -res } else { res }
}

struct Processor {
    cycle: i32,
    x: i32,
    score: i32,
}

impl Processor {
    fn new() -> Processor {
        Processor { cycle: 0, x: 1, score: 0 }
    }

    fn read_x(&mut self) -> i32 {
        self.process_cycle();
        return self.x
    }

    fn write_x(&mut self, new_x: i32) {
        self.process_cycle();
        self.x = new_x;
    }

    fn noop(&mut self) {
        self.process_cycle();
    }

    fn process_cycle(&mut self) {
        self.cycle += 1;
        let score = self.x * match self.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => self.cycle,
            _ => 0
        };
        self.score += score;
    }
}

fn main() {
    let mut it = BufReader::new(File::open("day10.txt").expect("I know you are existed"))
        .bytes()
        .map(|x| x.expect("You can do it!"))
        .peekable();

    let mut processor = Processor::new();

    loop {
        match read_cmd(&mut it) {
            Some(cmd) => {
                match cmd {
                    Addx(val) => {
                        let x = processor.read_x();
                        processor.write_x(x + val);
                    },
                    Noop => {
                        processor.noop();
                    }
                }
            },
            None => break
        }
    }
    println!("{}", processor.score);

}
