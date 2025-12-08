use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use union_find::{QuickFindUf, UnionBySize, UnionFind};

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn distance(lhs: &Point, rhs: &Point) -> i64 {
    (rhs.x - lhs.x) * (rhs.x - lhs.x)
        + (rhs.y - lhs.y) * (rhs.y - lhs.y)
        + (rhs.z - lhs.z) * (rhs.z - lhs.z)
}

fn main() {
    let points = BufReader::new(File::open("day08.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let mut it = x.split(',');
            Point {
                x: it.next().unwrap().parse::<i64>().unwrap(),
                y: it.next().unwrap().parse::<i64>().unwrap(),
                z: it.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut uf = QuickFindUf::<UnionBySize>::new(points.len());
    let mut pairs = (0..points.len())
        .flat_map(|l| ((l + 1)..points.len()).map(move |r| (l.clone(), r.clone())))
        .collect::<Vec<_>>();

    pairs.sort_by_key(|(l, r)| distance(&points[*l], &points[*r]));
    for (l, r) in pairs.into_iter().take(1000) {
        println!("union({}, {})", l, r);
        uf.union(l, r);
    }

    let mut res = (0..points.len())
        .into_iter()
        .fold(HashMap::new(), |mut acc, i| {
            acc.entry(uf.find(i)).and_modify(|x| *x += 1).or_insert(1);
            acc
        })
        .into_values()
        .collect::<Vec<_>>();
    res.sort_by(|lhs, rhs| rhs.cmp(lhs));

    println!("{}", res.into_iter().take(3).product::<i64>());
}
