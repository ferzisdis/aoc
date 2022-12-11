use std::{env, fs, usize};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::{Peekable, TakeWhile};
use std::ops::IndexMut;

fn append_tree(tree_dec: &mut VecDeque<i32>, tree_len: usize) -> i32 {
    let mut res = -1;
    for i in (0..=9).rev() {
        if i == tree_len {
            res = std::mem::replace(tree_dec.index_mut(tree_len), 1)
        } else if i < tree_len {
            tree_dec[i] = 1
        } else {
            tree_dec[i] += 1
        }
    }

    return res;
}

fn main() {
    let data = BufReader::new(File::open("day8.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("You can do it!"))
        .map(|s| s.into_bytes().into_iter().map(|b| usize::from(b - b'0')).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let mut scores = HashMap::new();

    {
        let mut tmp = data.iter().next().expect("at least one row should exist").iter().map(|_| VecDeque::from([0; 10])).collect::<Vec<VecDeque<i32>>>();

        for (idx_row, row) in data.iter().enumerate() {
            let mut col_tmp = VecDeque::from([0; 10]);
            for (idx_col, l) in row.iter().enumerate() {
                scores.insert(idx_row * tmp.len() + idx_col,
                              append_tree(&mut tmp[idx_col], *l) * append_tree(&mut col_tmp, *l));
            }
        }
    }
    {
        let mut tmp = data.iter().next().expect("at least one row should exist").iter().map(|_| VecDeque::from([0; 10])).collect::<Vec<VecDeque<i32>>>();

        for (idx_row, row) in data.iter().enumerate().rev() {
            let mut col_tmp = VecDeque::from([0; 10]);
            for (idx_col, l) in row.iter().enumerate().rev() {
                *scores.get_mut(&(idx_row * tmp.len() + idx_col))
                    .expect("inserted from previous") *= append_tree(&mut tmp[idx_col], *l) * append_tree(&mut col_tmp, *l);
            }
        }
    }

    println!("{}", scores.values().max().expect("at least one value should exist"));
}
