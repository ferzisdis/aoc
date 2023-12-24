use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::id;

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64
}

fn main() {
    let points = BufReader::new(File::open("day24.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            let mut it = x.split('@').flat_map(|x| x.split(',')).map(|x| x.trim()).filter_map(|x| x.parse::<f64>().ok());
            Point {
                x: it.next().unwrap(),
                y: it.next().unwrap(),
                z: it.next().unwrap(),
                dx: it.next().unwrap(),
                dy: it.next().unwrap(),
                dz: it.next().unwrap()
            }
        }).collect::<Vec<_>>();

    let res = points.iter().enumerate()
        .flat_map(|(idx, lhs)| points.iter().skip(idx + 1).map(|rhs| (*lhs, *rhs)))
        .map(|(lhs, rhs)| if is_intersect(lhs, rhs, 200000000000000f64, 400000000000000f64) { 1 } else { 0 })
        .sum::<i32>();

    println!("{}", res);
}

fn is_intersect(lhs: Point, rhs: Point, least: f64, most: f64) -> bool {
    let (a1, b1) = (lhs.dy / lhs.dx, lhs.y - (lhs.dy / lhs.dx) * lhs.x);
    let (a2, b2) = (rhs.dy / rhs.dx, rhs.y - (rhs.dy / rhs.dx) * rhs.x);

    if a1 - a2 == 0f64 {
        return false
    }

    let (x, y) = ((b2 - b1) / (a1 - a2), a1 * (b2 - b1) / (a1 - a2) + b1);
    if x < least || x > most || y < least || y > most {
        return false
    }

    // println!("crossed at ({}, {})", x, y);

    (x - lhs.x) / lhs.dx > 0f64 && (y - lhs.y) / lhs.dy > 0f64
        && (x - rhs.x) / rhs.dx > 0f64 && (y - rhs.y) / rhs.dy > 0f64
}
