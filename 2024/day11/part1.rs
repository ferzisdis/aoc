use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut hs = HashMap::new();
    let res = BufReader::new(File::open("day11.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .flat_map(|x| {
            x.split(" ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| solve(&mut hs, x, 25))
        .sum::<u64>();

    println!("{}", res);
}

fn solve(hs: &mut HashMap<(u128, usize), u64>, num: u128, blinks: usize) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if num == 0 {
        return solve(hs, 1, blinks - 1);
    }
    if hs.contains_key(&(num, blinks)) {
        return hs.get(&(num, blinks)).unwrap().clone();
    }
    let res = match num.ilog10() + 1 {
        power if power % 2 == 1 => solve(hs, num * 2024, blinks - 1),
        power => {
            solve(hs, num / 10u64.pow(power / 2) as u128, blinks - 1)
                + solve(hs, num % 10u64.pow(power / 2) as u128, blinks - 1)
        }
    };

    hs.insert((num, blinks), res);
    res
}
