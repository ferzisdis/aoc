use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    f32::consts::PI,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day17.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap());

    let _ = it
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, r)| r.parse::<u64>().unwrap())
        .unwrap();
    let mut reg_b = it
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, r)| r.parse::<u64>().unwrap())
        .unwrap();
    let mut reg_c = it
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, r)| r.parse::<u64>().unwrap())
        .unwrap();

    it.next();
    let program = it
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, r)| {
            r.split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap();

    let mut hs = HashMap::new();
    for part_reg_a in 0..=2048 {
        let (_, out, mask) = calc_til_jump(
            &program,
            &mut part_reg_a.clone(),
            &mut reg_b.clone(),
            &mut reg_c.clone(),
        );
        println!(
            "{} => {} ({:#b}), out = {}",
            part_reg_a,
            part_reg_a & mask,
            mask,
            out
        );
        if part_reg_a == 4131 {
            // return;
        }
        hs.entry((out, calc_width(part_reg_a)))
            .or_insert(Vec::new())
            .push((part_reg_a, mask));
    }

    let mut a_variants = Vec::new();
    if !find_a_variants(&program, &hs, 0, 0, 0, &mut a_variants) {
        println!("ERROR!")
    }

    a_variants.reverse();
    let mut res = a_variants
        .into_iter()
        .fold(0u64, |acc, val| (acc << 3) + val as u64);

    println!("{} {} {}", res, reg_b, reg_c);

    loop {
        let (pos, out, _) = calc_til_jump(&program, &mut res, &mut reg_b, &mut reg_c);
        print!("{},", out);
        if pos != 0 {
            break;
        }
    }
    println!();
    println!("Finish!");
}

fn find_a_variants(
    program: &Vec<u64>,
    hs: &HashMap<(u64, u64), Vec<(u64, u64)>>,
    idx: usize,
    mask: u64,
    prev: u64,
    result: &mut Vec<u64>,
) -> bool {
    if idx == program.len() {
        return true;
    }
    let width = (program.len() - idx).min(4);
    for (candidate, candidate_mask) in &hs[&(program[idx], width as u64)] {
        if mask & prev == mask & candidate {
            result.push(candidate % 8);
            if find_a_variants(
                program,
                hs,
                idx + 1,
                (mask | candidate_mask) >> 3,
                (prev | (candidate_mask & candidate)) >> 3,
                result,
            ) {
                println!("val {} res {:#b}", &program[idx], candidate);
                return true;
            }
            result.pop();
        }
    }
    false
}

fn calc_til_jump(
    program: &Vec<u64>,
    reg_a: &mut u64,
    reg_b: &mut u64,
    reg_c: &mut u64,
) -> (usize, u64, u64) {
    let width_mask = ((1 << (calc_width(*reg_a) * 3)) - 1);
    let mut mask = 0u64;
    let mut pos = 0;
    let mut out = 0;
    while let Some(instr) = program.get(pos) {
        let operand = program.get(pos + 1).unwrap().clone();
        match instr {
            0 => *reg_a = *reg_a / (2u64.pow(get_val(operand, *reg_a, *reg_b, *reg_c) as u32)),
            1 => *reg_b = *reg_b ^ operand,
            2 => *reg_b = get_val(operand, *reg_a, *reg_b, *reg_c).rem_euclid(8),
            3 => {
                mask = mask & width_mask;
                if *reg_a != 0 {
                    pos = operand as usize;
                    return (pos, out, mask);
                }
            }
            4 => *reg_b = *reg_b ^ *reg_c,
            5 => out = get_val(operand, *reg_a, *reg_b, *reg_c).rem_euclid(8),
            6 => *reg_b = *reg_a / (2u64.pow(get_val(operand, *reg_a, *reg_b, *reg_c) as u32)),
            7 => {
                let power = get_val(operand, *reg_a, *reg_b, *reg_c);
                mask = (((1 << 3) - 1) << power) | ((1 << 3) - 1);
                *reg_c = *reg_a / (2u64.pow(power as u32));
            }
            _ => unreachable!("Invalid instruction: {}", instr),
        }
        pos += 2;
    }
    (pos, out, mask)
}

fn calc_width(val: u64) -> u64 {
    if val == 0 {
        return 1;
    }
    ((val.ilog2() / 3) + 1) as u64
}

fn get_val(operand: u64, reg_a: u64, reg_b: u64, reg_c: u64) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unreachable!("Invalid operand: {}", operand),
    }
}
