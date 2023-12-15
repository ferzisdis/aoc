#![feature(linked_list_cursors)]
use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let res = BufReader::new(File::open("day15.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .next()
        .map(|x| {
            let mut hs: Vec<LinkedList<(&str, usize)>> = Vec::new();
            hs.resize(256, LinkedList::new());
            x.split(',')
                .fold(hs, |mut acc, x| {
                    if let Some((k, v)) = x.split_once('=') {
                        let parsed_val = v.parse::<usize>().unwrap();
                        let mut cur = acc.get_mut(hash(k)).unwrap()
                            .cursor_front_mut();
                        loop{
                            if let Some((cur_k, cur_v)) = cur.current() {
                                if &k == cur_k {
                                    *cur_v = parsed_val;
                                    break
                                }
                            } else {
                                cur.push_back((k, parsed_val));
                                break
                            }
                            cur.move_next();
                        }
                    } else if let Some((k, _)) = x.split_once('-') {
                        let mut cur = acc.get_mut(hash(k)).unwrap()
                            .cursor_front_mut();
                        while let Some((cur_k, _)) = cur.current() {
                            if &k == cur_k {
                                cur.remove_current();
                                break;
                            }
                            cur.move_next()
                        }
                    }
                    acc
                }).into_iter().enumerate().map(|(i, list)| {
                    list.into_iter().enumerate().map(|(j, (_, val))| (i + 1) * (j + 1) * val)
                        .sum::<usize>()
                }).sum::<usize>()
        }).unwrap_or(0);

    println!("{}", res)
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |acc, ch| {
        ((acc + ch as i32) * 17) % 256
    }) as usize
}
