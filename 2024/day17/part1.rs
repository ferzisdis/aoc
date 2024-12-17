use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day17.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap());

    let mut reg_a = it
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, r)| r.parse::<u32>().unwrap())
        .unwrap();
    let mut reg_b = it
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, r)| r.parse::<u32>().unwrap())
        .unwrap();
    let mut reg_c = it
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, r)| r.parse::<u32>().unwrap())
        .unwrap();

    it.next();
    let program = it
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, r)| {
            r.split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap();

    let mut pos = 0;

    while let Some(instr) = program.get(pos) {
        let operand = program.get(pos + 1).unwrap().clone();
        match instr {
            0 => reg_a = reg_a / (2u32.pow(get_val(operand, reg_a, reg_b, reg_c))),
            1 => reg_b = reg_b ^ operand,
            2 => reg_b = get_val(operand, reg_a, reg_b, reg_c).rem_euclid(8),
            3 => {
                if reg_a != 0 {
                    pos = operand as usize;
                    continue;
                }
            }
            4 => reg_b = reg_b ^ reg_c,
            5 => print!("{},", get_val(operand, reg_a, reg_b, reg_c).rem_euclid(8)),
            6 => reg_b = reg_a / (2u32.pow(get_val(operand, reg_a, reg_b, reg_c))),
            7 => reg_c = reg_a / (2u32.pow(get_val(operand, reg_a, reg_b, reg_c))),
            _ => unreachable!("Invalid instruction: {}", instr),
        }
        pos += 2;
    }
    println!();
    println!("Finish!")
}

fn get_val(operand: u32, reg_a: u32, reg_b: u32, reg_c: u32) -> u32 {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unreachable!("Invalid operand: {}", operand),
    }
}
