use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::ops::ControlFlow;
use std::rc::Rc;
use std::sync::mpsc::channel;

fn main() {
    let mut lines = BufReader::new(File::open("day14.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            x.chars().map(|x| Rc::new(RefCell::new(x))).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let mut by_cols = Vec::new();
    let mut by_rows = Vec::new();
    for line in lines.iter_mut() {
        if by_cols.len() == 0 {
            by_cols.resize(line.len(), Vec::new());
        }
        let mut row = Vec::new();
        for (col, val) in by_cols.iter_mut().zip(line) {
            col.push(Rc::clone(&val));
            row.push(Rc::clone(&val));
        }
        by_rows.push(row);
    }

    let by_cols_rev = by_cols.iter().map(|x| x.iter().rev().map(|x| Rc::clone(x)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let by_rows_rev = by_rows.iter().map(|x| x.iter().rev().map(|x| Rc::clone(x)).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut control = Vec::new();
    let mut control_i = 0;

    let mut cnt = 1000000000;
    let mut i = 0;

    while i < cnt {
        for v in vec![&by_cols, &by_rows, &by_cols_rev, &by_rows_rev] {
            v.iter()
                .map(|col| {
                    let len = col.len();
                    col.iter().enumerate().fold(0, |row, (idx, val)| {
                        let val = *val.borrow();
                        match val {
                            '.' => {
                                row
                            }
                            'O' => {
                                if row != idx {
                                    col.get(row).unwrap().replace('O');
                                    col.get(idx).unwrap().replace('.');
                                }
                                row + 1
                            }
                            '#' => {
                                idx + 1
                            }
                            _ => panic!("unexpected token")
                        }
                    })
                }).sum::<usize>();
        }

        let cur = by_rows.iter().flat_map(|x| x.iter().map(|x| *x.borrow())).collect::<Vec<_>>();
        if i == 1000 {
            control = cur;
            control_i = i;
        } else {
            if control == cur {
                let period = i - control_i;
                println!("period={}", period);
                i += period * ((cnt - i - 1) / period);
            }
        }
        i += 1
    }

    println!("{}", by_cols_rev.iter().map(|x| {
        x.iter().enumerate().map(|(idx, x)| if *x.borrow() == 'O' { idx + 1 } else { 0 }).sum::<usize>()
    }).sum::<usize>())
}
