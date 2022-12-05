use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::IndexMut;
use crate::Crate::{Concrete, Empty};

fn read_usize<T>(it: &mut T) -> usize
    where T : Iterator<Item = u8>
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

fn skip_word<T>(it: &mut T) -> ()
    where T : Iterator<Item = u8>
{
    for b in it {
        match b {
            b' ' => break,
            _ => ()
        }
    }
}

enum Crate {
    Concrete(u8), Empty
}

fn read_crate<T>(it: &mut T) -> Option<Crate>
    where T : Iterator<Item = u8>
{
    it.next().map(|b| {
        match b {
            b'[' => {
                let cr = Concrete(it.next().expect("expected format"));
                it.next(); // ]
                it.next(); // space
                cr
            },
            b' ' => {
                it.next(); // space
                it.next(); // space
                it.next(); // space
                Empty
            },
            _ => panic!("Unexpected byte {}", b)
        }
    })
}

fn main() {
    let mut stacks: Vec<VecDeque<u8>> = Vec::new();

    let mut it = BufReader::new(File::open("day5.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"));

    loop {
        if let Some(s) = it.next() {
            if s == "" {
                break
            }

            let mut sit = s.into_bytes().into_iter();
            let mut idx = 0;
            loop {
                if idx == stacks.len() {
                    stacks.push(VecDeque::new())
                }
                match read_crate(&mut sit) {
                    Some(cr) => {
                        if let Concrete(b) = cr {
                            stacks.index_mut(idx).push_back(b);
                        }
                    },
                    None => break
                }
                idx = idx + 1
            }
        } else {
            panic!("Why is it happened?")
        }
    }

    for s in it {
        let mut sit = s.into_bytes().into_iter();
        skip_word(&mut sit); // move
        let cnt = read_usize(&mut sit);
        skip_word(&mut sit); // from
        let from = read_usize(&mut sit) - 1;
        skip_word(&mut sit); // to
        let to = read_usize(&mut sit) - 1;

        let mut old = std::mem::replace(stacks.index_mut(to), VecDeque::new());
        for _ in 0..cnt {
            let b = stacks.index_mut(from).pop_front().expect("correct input");

            stacks.index_mut(to).push_back(b);
        }
        stacks.index_mut(to).append(&mut old);
    }

    let mut res = String::new();
    for mut d in stacks {
        if let Some(b) = d.pop_front() {
           res.push(char::from(b));
        }
    }

    println!("{}", res);
}
