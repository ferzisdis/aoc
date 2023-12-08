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


    let mut cycles = maps.keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| x.as_str())
        .flat_map(|x| {
            let mut targets = Vec::new();
            let mut visited = HashSet::new();
            let mut curr = x;
            let mut ended = false;
            for (i, (pos, m)) in moves.bytes().enumerate().cycle().enumerate() {
                if visited.contains(&(pos, curr)) {
                    // targets.push(i + 1 - targets.iter().next_back().map_or(0, |x| *x));
                    ended = true
                } else {
                    visited.insert((pos, curr));
                }

                curr = match m {
                    b'L' => {
                        maps.get(curr).expect("expected full map").get(0).expect("expected left value")
                    },
                    b'R' => {
                        maps.get(curr).expect("expected full map").get(1).expect("expected right value")
                    },
                    _ => panic!("unexpected byte {}", m)
                };

                if curr.ends_with("Z") {
                    targets.push(i + 1 - targets.iter().next_back().map_or(0, |x| *x));
                    break;

                    if ended {
                        break
                    }
                }
            }
            targets
        }).collect::<Vec<_>>();

    let mut res: Vec<usize> = Vec::new();
    for cycle in cycles {
        let mut cur = cycle;
        let mut simples = Vec::new();
        for d in 2.. {
            if d > cur {
                break
            }
            while cur % d == 0 {
                cur /= d;
                simples.push(d);
            }
        }
        if cur > 1 {
            simples.push(cur)
        }

        let mut new_res = Vec::new();
        let mut it = res.into_iter().peekable();

        for s in simples {
            loop {
                if let Some(it_v) = it.peek(){
                    if *it_v == s {
                        new_res.push(*it_v);
                        it.next();
                        break
                    } else if *it_v < s {
                        new_res.push(*it_v);
                        it.next();
                        continue
                    } else {
                        new_res.push(s);
                        break
                    }
                } else {
                    new_res.push(s);
                    break
                }
            }
        }
        for it_v in it {
            new_res.push(it_v)
        }
        res = new_res
    }
    println!("{}", res.into_iter().fold(1u64, |acc, x| acc * x as u64));
}
