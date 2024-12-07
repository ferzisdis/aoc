use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let res = BufReader::new(File::open("day07.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split_once(": ")
                .map(|(l, r)| {
                    (
                        l.parse::<u64>().unwrap(),
                        r.split(" ")
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap()
        })
        .filter_map(|(expect, nums)| {
            let mut it = nums.into_iter();
            if is_valid(expect, it.next().unwrap(), it) {
                Some(expect)
            } else {
                None
            }
        })
        .sum::<u64>();

    println!("{}", res);
}

fn is_valid<It>(expect: u64, cur: u64, mut next: It) -> bool
where
    It: Iterator<Item = u64> + Clone,
{
    if cur > expect {
        return false;
    }
    match next.next() {
        Some(num) => is_valid(expect, cur * num, next.clone()) || is_valid(expect, cur + num, next),
        None => expect == cur,
    }
}

fn get_cell(field: &Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field.get(x as usize).and_then(|line| line.get(y as usize))
}

fn get_cell_mut(field: &mut Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&mut u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field
        .get_mut(x as usize)
        .and_then(|line| line.get_mut(y as usize))
}
