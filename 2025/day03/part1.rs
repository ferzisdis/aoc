use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let ans = BufReader::new(File::open("day03.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.into_bytes())
        .map(|x| {
            let res = (x.len() - 2..x.len())
                .fold((0, 0), |(left, res), mut right| {
                    for i in (left..=right).rev() {
                        if x[i] >= x[right] {
                            right = i;
                        }
                    }
                    (right + 1, res * 10 + (x[right] - '0' as u8) as u64)
                })
                .1;
            println!("found {}", res);
            res
        })
        .sum::<u64>();

    println!("{}", ans);
}
