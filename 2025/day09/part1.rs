use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use union_find::{QuickFindUf, UnionBySize, UnionFind};

struct Point {
    x: i64,
    y: i64,
}

fn area(lhs: &Point, rhs: &Point) -> i64 {
    (rhs.x.abs_diff(lhs.x) + 1) as i64 * (rhs.y.abs_diff(lhs.y) + 1) as i64
}

fn main() {
    let points = BufReader::new(File::open("day09.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let mut it = x.split(',');
            Point {
                x: it.next().unwrap().parse::<i64>().unwrap(),
                y: it.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let pairs = (0..points.len())
        .flat_map(|l| ((l + 1)..points.len()).map(move |r| (l.clone(), r.clone())))
        .collect::<Vec<_>>();

    let res = pairs
        .into_iter()
        .map(|(l, r)| area(&points[l], &points[r]))
        .max()
        .unwrap();

    println!("{}", res);
}
