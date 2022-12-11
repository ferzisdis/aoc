use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::{Peekable, TakeWhile};
use std::ops::IndexMut;

fn main() {
    let data = BufReader::new(File::open("day8.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("You can do it!"))
        .map(|s| s.into_bytes().into_iter().map(|b| i32::from(b - b'0')).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut visible = HashSet::new();

    {
        let mut tmp = data.iter().next().expect("at least one row should exist").iter().map(|_| -1).collect::<Vec<i32>>();

        for (idx_row, row) in data.iter().enumerate() {
            let mut col_tmp = -1;
            for (idx_col, l) in row.iter().enumerate() {
                if tmp[idx_col] < *l {
                    visible.insert(idx_row * tmp.len() + idx_col);
                    let _ = std::mem::replace(tmp.index_mut(idx_col), *l);
                }
                if col_tmp < *l {
                    visible.insert(idx_row * tmp.len() + idx_col);
                    col_tmp = *l;
                }
            }
        }
    }
    {
        let mut tmp = data.iter().next().expect("at least one row should exist").iter().map(|_| -1).collect::<Vec<i32>>();
        for (idx_row, row) in data.iter().enumerate().rev() {
            let mut col_tmp = -1;
            for (idx_col, l) in row.iter().enumerate().rev() {
                if tmp[idx_col] < *l {
                    visible.insert(idx_row * tmp.len() + idx_col);
                    let _ = std::mem::replace(tmp.index_mut(idx_col), *l);
                }
                if col_tmp < *l {
                    visible.insert(idx_row * tmp.len() + idx_col);
                    col_tmp = *l;
                }
            }
        }
    }

    println!("{}", visible.len());

}
