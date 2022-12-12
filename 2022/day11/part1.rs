use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::{Peekable, TakeWhile};
use std::ops::IndexMut;

struct Monkey
{
    tests: i32,
    items: VecDeque<i32>,
    op: Box<dyn Fn(i32)->i32>,
    test: Box<dyn Fn(i32)->usize>,
}

struct Throw {
    monkey: usize,
    item: i32,
}

impl Monkey
{
    fn throw(&mut self) -> Option<Throw> {
        self.items.pop_front().and_then(|item| {
            let level = (self.op)(item) / 3;
            self.tests += 1;
            Some(Throw{monkey: (self.test)(level), item: level})
        })
    }

    fn catch(&mut self, item: i32) {
        self.items.push_back(item);
    }
}

fn main() {
    let mut monkeys = vec![
        Monkey { tests:0, items: VecDeque::from(vec![96, 60, 68, 91, 83, 57, 85]), op: Box::new(|x| x * 2), test: Box::new(|x| if x % 17 == 0 { 2 } else { 5 }) },
        Monkey { tests:0, items: VecDeque::from(vec![75, 78, 68, 81, 73, 99]), op: Box::new(|x| x + 3), test: Box::new(|x| if x % 13 == 0  { 7 } else { 4 }) },
        Monkey { tests:0, items: VecDeque::from(vec![69, 86, 67, 55, 96, 69, 94, 85]), op: Box::new(|x| x + 6), test: Box::new(|x| if x % 19 == 0  { 6 } else { 5 }) },
        Monkey { tests:0, items: VecDeque::from(vec![88, 75, 74, 98, 80]), op: Box::new(|x| x + 5), test: Box::new(|x| if x % 7 == 0 { 7 } else { 1 }) },
        Monkey { tests:0, items: VecDeque::from(vec![82]), op: Box::new(|x| x + 8), test: Box::new(|x| if x % 11 == 0 { 0 } else { 2 }) },
        Monkey { tests:0, items: VecDeque::from(vec![72, 92, 92]), op: Box::new(|x| x * 5), test: Box::new(|x| if x % 3 == 0 { 6 } else { 3 }) },
        Monkey { tests:0, items: VecDeque::from(vec![74, 61]), op: Box::new(|x| x * x), test: Box::new(|x| if x % 2 == 0 { 3 } else { 1 }) },
        Monkey { tests:0, items: VecDeque::from(vec![76, 86, 83, 55]), op: Box::new(|x| x + 4), test: Box::new(|x| if x % 5 == 0 { 4 } else { 0 }) },
    ];


    for _ in 0..20 {
        for m in 0..monkeys.len() {
            loop {
                match monkeys[m].throw() {
                    Some(throw) => {
                        monkeys[throw.monkey].catch(throw.item);
                    },
                    None => break
                }
            }
        }
    }

    monkeys.sort_by(|lhs, rhs| rhs.tests.cmp(&lhs.tests));
    let mut it = monkeys.into_iter();
    let first = it.next().expect("monkeys > 2").tests;
    let second = it.next().expect("monkeys > 2").tests;

    println!("{}", first * second);

}
