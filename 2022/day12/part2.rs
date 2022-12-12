use std::{env, fs, usize};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::{Peekable, TakeWhile};
use std::ops::IndexMut;

fn step_to(data: &Vec<Vec<u8>>, cur: u8, idx_row_to : usize, idx_column_to: usize) -> Option<(usize, usize)> {
    data.get(idx_row_to).and_then(|v| v.get(idx_column_to).and_then(|next| {
        let next = match *next {
            b'S' => b'a',
            b'E' => b'z',
            _ => *next
        };

        let cur = match cur {
            b'S' => b'a',
            b'E' => b'z',
            _ => cur
        };

        if cur >= next && cur - next <= 1 || next > cur {
            Some((idx_row_to, idx_column_to))
        } else {
            None
        }
    }))
}

fn main() {
    let data = BufReader::new(File::open("day12.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("You can do it!"))
        .map(|s| s.into_bytes().into_iter().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut queue = VecDeque::new();
    let mut hs = HashMap::new();
    for (idx_row, row) in data.iter().enumerate() {
        for (idx_col, b) in row.iter().enumerate() {
            if *b == b'E' {
                queue.push_back((idx_row, idx_col));
                hs.insert((idx_row, idx_col), 0);
                break
            }
        }
    }

    let mut min : Option<i32> = None;

    loop {
        if let Some((idx_row, idx_col)) = queue.pop_front() {
            let cur = data[idx_row][idx_col];
            let cur_len = *hs.get(&(idx_row, idx_col)).expect("100% exist");
            if cur == b'a' {
                min = min.map_or(Some(cur_len), |x| Some(x.min(cur_len)));
            }
            if let Some(next) = step_to(&data, cur, idx_row + 1, idx_col)
                .and_then(|x| if hs.contains_key(&x) { None } else { Some(x) })
            {
                queue.push_back(next);
                hs.insert(next, cur_len + 1);
            }
            if idx_row > 0 {
                if let Some(next) = step_to(&data, cur, idx_row - 1, idx_col)
                    .and_then(|x| if hs.contains_key(&x) { None } else { Some(x)  })
                {
                    queue.push_back(next);
                    hs.insert(next, cur_len + 1);
                }
            }
            if let Some(next) = step_to(&data, cur, idx_row, idx_col + 1)
                .and_then(|x| if hs.contains_key(&x) { None } else { Some(x)  })
            {
                queue.push_back(next);
                hs.insert(next, cur_len + 1);
            }
            if idx_col > 0 {
                if let Some(next) = step_to(&data, cur, idx_row, idx_col - 1)
                    .and_then(|x| if hs.contains_key(&x) { None } else { Some(x)  })
                {
                    queue.push_back(next);
                    hs.insert(next, cur_len + 1);
                }
            }
        } else {
            break
        }
    }

    println!("{}", min.expect("hoped to found something"));
}
