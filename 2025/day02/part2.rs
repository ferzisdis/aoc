use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let ans = BufReader::new(File::open("day02.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .flat_map(|x| x.split(',').map(|t| t.to_string()).collect::<Vec<String>>())
        .fold(0u64, |mut acc, x| {
            let (l, r) = x.split_once('-').unwrap();
            let (l, r) = (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap());
            let mut hs = HashSet::new();

            for repeats in 2.. {
                if r.ilog10() + 1 < repeats {
                    break;
                }
                let digits = l.ilog10() + 1;
                let mut pattern = l / 10u64.pow(digits - digits / repeats);
                if pattern == 0 {
                    pattern = 1;
                }
                for i in pattern.. {
                    let num = (1..repeats).fold(i, |acc, _| acc * 10u64.pow(i.ilog10() + 1) + i);
                    if num < l || hs.contains(&num) {
                        continue;
                    }
                    if num > r {
                        break;
                    }
                    println!("found {}", num);
                    acc += num;
                    hs.insert(num);
                }
            }

            acc
        });
    println!("{}", ans);
}
