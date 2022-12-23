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

fn main() {
    let mut nums = BufReader::new(File::open("day20.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|s| {
            read_int(&mut s.into_bytes().into_iter())
        }).collect::<Vec<i32>>();

    println!("nums {:?}", nums.iter().sum::<i32>());

    let mut positions = (0..nums.len()).collect::<Vec<usize>>();
    for (idx, num) in nums.iter().enumerate() {
        match (*num) % (nums.len() as i32 - 1) {
            num if num > 0 => {
                let (cur_pos, next_pos) = (positions[idx], (positions[idx] + num as usize + 1).rem_euclid(nums.len()));
                if cur_pos == next_pos {
                    continue
                }
                if next_pos > cur_pos {
                    for pos in &mut positions {
                        if *pos > cur_pos && *pos < next_pos {
                            *pos -= 1
                        }
                    }
                    positions[idx] = next_pos - 1
                } else {
                    for pos in &mut positions {
                        if *pos >= next_pos && *pos < cur_pos {
                            *pos += 1
                        }
                    }
                    positions[idx] = next_pos
                }
            },
            num if num < 0 => {
                let (cur_pos, next_pos) = (positions[idx], (positions[idx] as i32 + num - 1).rem_euclid(nums.len() as i32) as usize);
                if cur_pos == next_pos {
                    continue
                }
                if next_pos > cur_pos {
                    for pos in &mut positions {
                        if *pos > cur_pos && *pos <= next_pos {
                            *pos -= 1
                        }
                    }
                    positions[idx] = next_pos
                } else {
                    for pos in &mut positions {
                        if *pos > next_pos && *pos < cur_pos {
                            *pos += 1
                        }
                    }
                    positions[idx] = next_pos + 1
                }
            },
            _ => continue
        }
    }

    let zero_pos = nums.iter().enumerate().find_map(|(idx, val)| {
        if *val == 0 {
            Some(positions[idx])
        } else {
            None
        }
    }).expect("zero should be in input data");

    let target = (0..positions.len())
        .map(|pos| { nums[positions.iter().enumerate()
            .find_map(|(idx, x)| if *x == pos { Some(idx) } else { None }).expect("")] }).collect::<Vec<i32>>();

    println!("control {}", target.iter().sum::<i32>());

    println!("res zero = {}| {}", zero_pos,
             target[(zero_pos+1000).rem_euclid(positions.len())] +
             target[(zero_pos+2000).rem_euclid(positions.len())] +
             target[(zero_pos+3000).rem_euclid(positions.len())]);
}
