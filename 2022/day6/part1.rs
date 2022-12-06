use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::TakeWhile;
use std::ops::IndexMut;

trait TakeUntilExt<P>
    where
        Self: Sized,
{
    fn take_until(self, predicate: P) -> TakeUntil<Self, P>;
}

impl<I, P> TakeUntilExt<P> for I
    where
        I: Sized + Iterator,
        P: FnMut(&I::Item) -> bool,
{
    fn take_until(self, predicate: P) -> TakeUntil<Self, P> {
        TakeUntil {
            iter: self,
            flag: false,
            predicate,
        }
    }
}

struct TakeUntil<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> Iterator for TakeUntil<I, P>
    where
        I: Iterator,
        P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            self.iter.next().and_then(|x| {
                if (self.predicate)(&x) {
                    self.flag = true;
                }
                Some(x)
            })
        }
    }
}

fn main() {
    let mut hs = [0; 26];
    let mut cnt = 0;

    println!("{}", BufReader::new(File::open("day6.txt").expect("I know you are existed"))
        .bytes()
        .map(|x| x.expect("You can do it!"))
        .enumerate()
        .take_until(|(idx, b)| {
            let lastIdx = std::mem::replace(hs.index_mut(usize::from(*b - b'a')), *idx + 1);
            cnt = std::cmp::min(cnt + 1, *idx - lastIdx + 1);
            cnt == 4
        })
        .count());

}
