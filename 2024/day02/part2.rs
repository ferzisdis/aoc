use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let result = BufReader::new(File::open("day02.txt").expect("what could be happen?"))
        .lines()
        .map(|e| e.expect("line should exist"))
        .filter_map(|e| if is_safe(e) { Some(()) } else { None })
        .count();

    println!("{}", result);
}

fn is_safe(report: String) -> bool {
    let mut levels = report
        .split(" ")
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    if is_valid_order(&levels) {
        return true;
    }
    levels.reverse();
    is_valid_order(&levels)
}

fn is_valid_order(levels: &Vec<u32>) -> bool {
    let mut it = levels.iter();
    let mut prevs = VecDeque::new();
    prevs.push_back(it.next().unwrap());
    let mut mistake = false;

    while let Some(cur) = it.next() {
        if is_pair_valid(prevs.back().unwrap(), cur) {
            if prevs.len() == 2 {
                if !is_pair_valid(prevs.front().unwrap(), prevs.back().unwrap()) {
                    if mistake {
                        return false;
                    }
                    mistake = true;
                }
                prevs.pop_front();
            }
            prevs.push_back(cur);
            continue;
        }
        if prevs.len() == 1 {
            prevs.push_back(cur);
            continue;
        }
        if is_pair_valid(prevs.front().unwrap(), cur) {
            prevs.pop_front();
            prevs.push_back(cur);
            continue;
        }
        if mistake {
            return false;
        }
        mistake = true;
    }
    if prevs.len() == 2 && !is_pair_valid(prevs.front().unwrap(), prevs.back().unwrap()) {
        if mistake {
            return false;
        }
    }
    true
}

fn is_pair_valid(l: &u32, r: &u32) -> bool {
    l < r && r - l <= 3
}
