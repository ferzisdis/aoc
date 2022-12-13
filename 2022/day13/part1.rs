use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;

fn read_int(it: &mut Peekable<impl Iterator<Item=u8>>) -> i32
{
    let mut res = 0;
    loop {
        match it.next_if(|b| b != &b',' && b != &b']') {
            Some(b) => {
                res = res * 10 + i32::from(b - b'0')
            },
            None => break
        }
    }

    return res;
}

fn is_right_order(lhs: &mut Peekable<impl Iterator<Item=u8>>, rhs: &mut Peekable<impl Iterator<Item=u8>>) -> Option<bool> {
    if let (Some(lb), Some(rb)) = (lhs.peek(), rhs.peek())  {
        match (lb, rb) {
            (&b'[', &b'[') => {
                println!("inner branch [ [");
                lhs.next(); rhs.next();
                loop {
                    lhs.next_if_eq(&b','); rhs.next_if_eq(&b',');
                    if let (Some(lb), Some(rb)) = (lhs.peek(), rhs.peek())  {
                        match (lb, rb) {
                            (&b']', &b']') => {
                                lhs.next(); rhs.next();
                                println!("inner branch ] ]");
                                return None
                            },
                            (&b']', _) => return Some(true),
                            (_, &b']') => return Some(false),
                            (&b'[', _) | (_, &b'[') => {
                                match is_right_order(lhs, rhs) {
                                    Some(res) => return Some(res),
                                    None => continue
                                }
                            },
                            (&(b'0'..=b'9'), &(b'0'..=b'9')) => {
                                let (lint, rint) = (read_int(lhs), read_int(rhs));
                                if lint == rint {
                                    continue;
                                }
                                return Some(lint < rint)
                            },
                            _ => panic!("both {} {}", char::from(*lb), char::from(*rb))
                        }
                    }
                }
            },
            (&b'[', &(b'0'..=b'9')) => {
                println!("left branch [ *");
                lhs.next();
                match lhs.peek()  {
                    Some(&b']') => return Some(true),
                    Some(&b'[') => {
                        return match is_right_order(lhs, rhs) {
                            Some(res) => Some(res),
                            None => {
                                match lhs.next() {
                                    Some(b']') => {
                                        None
                                    },
                                    _ => Some(false)
                                }
                            }
                        };
                    },
                    Some(&(b'0'..=b'9')) => {
                        let (lint, rint) = (read_int(lhs), read_int(rhs));
                        if lint == rint {
                            return match lhs.next() {
                                Some(b']') => {
                                    None
                                },
                                _ => Some(false)
                            };
                        }
                        return Some(lint < rint)
                    },
                    _ => panic!("left")
                }
            },
            (&(b'0'..=b'9'), &b'[') => {
                println!("right branch [ *");
                rhs.next();
                match rhs.peek()  {
                    Some(&b']') => return Some(false),
                    Some(&b'[') => {
                        return match is_right_order(lhs, rhs) {
                            Some(res) => Some(res),
                            None => {
                                match rhs.next() {
                                    Some(b']') => {
                                        None
                                    },
                                    _ => Some(true)
                                }
                            }
                        }
                    },
                    Some(&(b'0'..=b'9')) => {
                        let (lint, rint) = (read_int(lhs), read_int(rhs));
                        if lint == rint {
                            return match rhs.next() {
                                Some(b']') => {
                                    None
                                },
                                _ => Some(true)
                            }
                        }
                        return Some(lint < rint)
                    },
                    _ => panic!("right")
                }
            },
            _ => panic!("first")
        }
    }

    None
}

fn main() {
    let mut it = BufReader::new(File::open("day13.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"));

    let mut res = 0;
    let mut idx = 0;

    loop {
        match it.next() {
            Some(first) => {
                let second = it.next().expect("rows divided by 3");
                it.next(); // skip empty line
                idx += 1;

                if is_right_order(
                    &mut first.into_bytes().into_iter().peekable(),
                    &mut second.into_bytes().into_iter().peekable())
                    .expect("expect result anyway")
                {
                    res = res + idx;
                    println!("correct!")
                } else {
                    println!("incorrect!")
                }
            },
            None => break
        }
    }

    println!("result = {}", res)
}
