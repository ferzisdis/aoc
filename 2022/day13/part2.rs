use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
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

fn is_right_order(lhs: &mut Peekable<impl Iterator<Item=u8>>, rhs: &mut Peekable<impl Iterator<Item=u8>>) -> Ordering {
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
                                return Equal
                            },
                            (&b']', _) => return Less,
                            (_, &b']') => return Greater,
                            (&b'[', _) | (_, &b'[') => {
                                match is_right_order(lhs, rhs) {
                                    Less => return Less,
                                    Greater => return Greater,
                                    Equal => continue
                                }
                            },
                            (&(b'0'..=b'9'), &(b'0'..=b'9')) => {
                                let (lint, rint) = (read_int(lhs), read_int(rhs));
                                if lint == rint {
                                    continue;
                                }
                                return lint.cmp(&rint);
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
                    Some(&b']') => return Less,
                    Some(&b'[') => {
                        return match is_right_order(lhs, rhs) {
                            Less => return Less,
                            Greater => return Greater,
                            Equal => {
                                match lhs.next() {
                                    Some(b']') => {
                                        Equal
                                    },
                                    _ => Greater
                                }
                            }
                        };
                    },
                    Some(&(b'0'..=b'9')) => {
                        let (lint, rint) = (read_int(lhs), read_int(rhs));
                        if lint == rint {
                            return match lhs.next() {
                                Some(b']') => {
                                    Equal
                                },
                                _ => Greater
                            };
                        }
                        return lint.cmp(&rint);
                    },
                    _ => panic!("left")
                }
            },
            (&(b'0'..=b'9'), &b'[') => {
                println!("right branch [ *");
                rhs.next();
                match rhs.peek()  {
                    Some(&b']') => return Greater,
                    Some(&b'[') => {
                        return match is_right_order(lhs, rhs) {
                            Less => return Less,
                            Greater => return Greater,
                            Equal => {
                                match rhs.next() {
                                    Some(b']') => {
                                        Equal
                                    },
                                    _ => Less
                                }
                            }
                        }
                    },
                    Some(&(b'0'..=b'9')) => {
                        let (lint, rint) = (read_int(lhs), read_int(rhs));
                        if lint == rint {
                            return match rhs.next() {
                                Some(b']') => {
                                    Equal
                                },
                                _ => Less
                            }
                        }
                        return return lint.cmp(&rint);
                    },
                    _ => panic!("right")
                }
            },
            _ => panic!("first")
        }
    }

    Equal
}

fn main() {
    let mut vec = BufReader::new(File::open("day13.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .filter(|s| s.len() > 0)
        .collect::<Vec<String>>();

    vec.push(String::from("[[2]]"));
    vec.push(String::from("[[6]]"));
    vec.sort_by(|lhs, rhs| {
        is_right_order(
            &mut lhs.bytes().into_iter().peekable(),
            &mut rhs.bytes().into_iter().peekable())
    });

    let mut res = 1;
    for (idx, str) in vec.into_iter().enumerate() {
       if str == "[[2]]" || str == "[[6]]" {
           res *= (idx + 1)
       }
    }
    println!("res = {}", res)
}
