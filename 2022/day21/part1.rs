extern crate core;

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

enum Operation {
    Const(i32),
    Eval(String, u8, String)
}

fn main() {
    let mut inputs = BufReader::new(File::open("day21.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|s| {
            let mut it = s.into_bytes().into_iter().peekable();

            let name = read_n(&mut it, 4);
            it.next(); it.next();
            let op = match it.peek().clone() {
                Some(b) if *b >= b'0' && *b <= b'9' => {
                    Operation::Const(read_int(&mut it))
                },
                _ => {
                    let lhs = read_n(&mut it, 4);
                    it.next();
                    let op = it.next().expect("must");
                    it.next();
                    let rhs = read_n(&mut it, 4);

                    Operation::Eval(lhs, op, rhs)
                }
            };

            (name, op)
        }).collect::<Vec<(String, Operation)>>();

    let mut values = HashMap::new();
    let mut evals = HashMap::new();

    for (name, op) in inputs {
        match op {
            Operation::Const(val) => {
                values.insert(name, val as i64);
            },
            Operation::Eval(_, _, _) => {
                evals.insert(name, op);
            }
        }
    }

    println!("root = {}", slove("root", &mut values, &evals));
}

fn slove(root: &str, values: &mut HashMap<String, i64>, evals: &HashMap<String, Operation>) -> i64 {
    if let Some(cached) = values.get(root) {
        *cached
    } else {
        let val = match evals.get(root) {
            Some(Operation::Eval(lhs, op, rhs)) if *op == b'+' => {
                slove(lhs, values, evals) + slove(rhs, values, evals)
            },
            Some(Operation::Eval(lhs, op, rhs)) if *op == b'-' => {
                slove(lhs, values, evals) - slove(rhs, values, evals)
            },
            Some(Operation::Eval(lhs, op, rhs)) if *op == b'*' => {
                slove(lhs, values, evals) * slove(rhs, values, evals)
            },
            Some(Operation::Eval(lhs, op, rhs)) if *op == b'/' => {
                slove(lhs, values, evals) / slove(rhs, values, evals)
            },
            _ => panic!("unexpected res")
        };
        values.insert(String::from(root), val);
        val
    }
}
