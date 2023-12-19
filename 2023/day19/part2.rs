use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("day19.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"));

    let mut workflows = HashMap::new();
    loop {
        if let Some(line) = lines.next() {
            if line.is_empty() {
                break
            }

            if let Some((name, r_part)) = line.split_once('{') {
                workflows.insert(name.to_string(), r_part.trim_end_matches('}').to_string());
            }
        }
    }

    let start = vec!["x", "m", "a", "s"].into_iter()
        .fold(HashMap::new(),|mut acc, x| {
            acc.insert(x.to_string(), (1, 4000));
            acc
        });

    let mut parts = VecDeque::new();
    parts.push_back(("in", start));
    let mut res = 0;

    while let Some((wf,mut part)) = parts.pop_front()  {
        if let Some(rules) = workflows.get(wf) {
            for rule in rules.split(',') {
                if let Some((r, target)) = rule.split_once(':') {
                    let mut chars = r.chars();
                    let category = String::from(chars.next().unwrap());
                    let condition = chars.next().unwrap();
                    let compare_to = chars.collect::<String>().parse::<i32>().unwrap();

                    let (min, max) = *part.get(category.as_str()).unwrap();
                    match condition {
                        '>' if min > compare_to => {
                            parts.push_back((target, part.clone()));
                            break
                        },
                        '>' if max > compare_to => {
                            let mut next = part.clone();
                            *next.get_mut(category.as_str()).unwrap() = (min, compare_to);
                            *part.get_mut(category.as_str()).unwrap() = (compare_to + 1, max);
                            parts.push_back((target, part.clone()));
                            part = next;
                        },
                        '<' if max < compare_to => {
                            parts.push_back((target, part.clone()));
                            break
                        },
                        '<' if min < compare_to => {
                            let mut next = part.clone();
                            *next.get_mut(category.as_str()).unwrap() = (compare_to, max);
                            *part.get_mut(category.as_str()).unwrap() = (min, compare_to - 1);
                            parts.push_back((target, part.clone()));
                            part = next;
                        },
                        _ => (),
                    };
                } else {
                    parts.push_back((rule, part.clone()));
                }
            }
        } else {
            res += match wf {
                "A" => part.into_values().map(|(min, max)| max - min + 1).fold(1u128, |acc, x| acc * x as u128),
                "R" => 0,
                _ => panic!("unexpected state {}", wf)
            }
        }
    }

    println!("{}", res)
}
