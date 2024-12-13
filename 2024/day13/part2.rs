use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day13.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap());

    let mut res = 0;

    loop {
        let button_a = it
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.split_once("+").unwrap().1.parse::<i128>().unwrap(),
                    y.split_once("+").unwrap().1.parse::<i128>().unwrap(),
                )
            })
            .unwrap();
        let button_b = it
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.split_once("+").unwrap().1.parse::<i128>().unwrap(),
                    y.split_once("+").unwrap().1.parse::<i128>().unwrap(),
                )
            })
            .unwrap();

        let prize = it
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.split_once("=").unwrap().1.parse::<i128>().unwrap() + 10000000000000i128,
                    y.split_once("=").unwrap().1.parse::<i128>().unwrap() + 10000000000000i128,
                )
            })
            .unwrap();

        res += calc_tokens(button_a, button_b, prize);
        if it.next().is_none() {
            break;
        }
    }

    println!("{}", res);
}

fn calc_tokens(a: (i128, i128), b: (i128, i128), p: (i128, i128)) -> i128 {
    // a_x * a_n + b_x * b_n = p_x
    // a_y * a_n + b_y * b_n = p_y
    // a_n = (p_x - b_x * b_n) / a_x
    // p_x * a_y / a_x - b_x * a_y * b_n / a_x + b_y * b_n = p_y
    // p_x * a_y - b_x * a_y * b_n + b_y * b_n * a_x = p_y * a_x
    // b_n = (p_y * a_x - p_x * a_y) / (b_y * a_x - b_x * a_y)
    if (p.1 * a.0 - p.0 * a.1) % (b.1 * a.0 - b.0 * a.1) != 0 {
        return 0;
    }
    let b_n = (p.1 * a.0 - p.0 * a.1) / (b.1 * a.0 - b.0 * a.1);
    if (p.0 - b.0 * b_n) % a.0 != 0 {
        return 0;
    }
    let a_n = (p.0 - b.0 * b_n) / a.0;
    return a_n * 3 + b_n;
}
