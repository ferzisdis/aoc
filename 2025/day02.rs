use std::{
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
            for i in l / 10u64.pow(l.ilog10() + 1 / 2).. {
                let num = i * 10u64.pow(i.ilog10() + 1) + i;
                if num < l {
                    continue;
                }
                if num > r {
                    break;
                }
                acc += num;
            }
            acc
        });
    println!("{}", ans);
}
