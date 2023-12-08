use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("day8.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"));

    let mut moves = lines.next().expect("no moves");
    lines.next(); // skip empty lines
    let mut maps = lines.fold(HashMap::new(), |mut agg, x| {
        let mut parts = x.split(" = ");
        let key = String::from(parts.next().expect("should be a key"));
        let values = parts.next()
            .expect("should be values")
            .split(", ")
            .map(|x| {
                if x.starts_with("(") {
                    x.chars().skip(1).collect::<String>()
                } else {
                    x.chars().take(x.len() - 1).collect::<String>()
                }
            }).collect::<Vec<String>>();

        agg.insert(key, values);
        agg
    });

    let mut res = 0;
    let mut curr = "AAA";
    for (i, m) in moves.bytes().cycle().enumerate() {
        match m {
            b'L' => {
                curr = maps.get(curr).expect("expected full map").get(0).expect("expected left value")
            },
            b'R' => {
                curr = maps.get(curr).expect("expected full map").get(1).expect("expected right value")
            }
            _ => panic!("unexpected byte {}", m)
        }

        if curr == "ZZZ" {
            res = i + 1;
            break;
        }
    }

    println!("{}", res);
}
