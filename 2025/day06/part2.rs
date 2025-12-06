use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let source = BufReader::new(File::open("day06.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut num = String::new();
    let mut nums = Vec::<String>::new();
    let mut res = 0;
    for col in (0..source.iter().max_by_key(|x| x.len()).unwrap().len()).rev() {
        if !num.is_empty() {
            println!("Num: {}", num);
            nums.push(num);
            num = String::new();
        }

        for row in &source {
            match row.as_bytes().get(col) {
                Some(&b' ') | None => {}
                Some(&b'+') => {
                    println!("()+ Num: {}", num);
                    nums.push(num);
                    num = String::new();
                    res += nums.iter().map(|x| x.parse::<u64>().unwrap()).sum::<u64>();
                    nums.clear();
                }
                Some(&b'*') => {
                    println!("(*) Num: {}", num);
                    nums.push(num);
                    num = String::new();
                    res += nums
                        .iter()
                        .map(|x| x.parse::<u64>().unwrap())
                        .product::<u64>();
                    nums.clear();
                }
                Some(ch) => num.push(*ch as char),
            }
        }
    }

    println!("{}", res);
}
