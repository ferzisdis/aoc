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

    let mut parts = Vec::new();
    for line in lines {
        let part = line.trim_start_matches('{').trim_end_matches('}').split(',')
            .fold(HashMap::new(), |mut acc, x| {
                if let Some((name, val)) = x.split_once('=') {
                    acc.insert(name.to_string(), val.parse::<i32>().ok().unwrap());
                }
                acc
            });
        parts.push(part);
    }

    println!("{}", parts.into_iter().map(|part| {
        let mut wf = "in";
        loop {
            if let Some(rules) = workflows.get(wf) {
                for rule in rules.split(',') {
                    if let Some((r, target)) = rule.split_once(':') {
                        let mut chars = r.chars();
                        let category = String::from(chars.next().unwrap());
                        let condition = chars.next().unwrap();
                        let compare_to = chars.collect::<String>().parse::<i32>().unwrap();

                        let res = match condition {
                            '>' if *part.get(category.as_str()).unwrap() > compare_to => true,
                            '<' if *part.get(category.as_str()).unwrap() < compare_to => true,
                            _ => false,
                        };
                        if res {
                            wf = target;
                            break
                        }
                    } else {
                        wf = rule;
                    }
                }
            } else {
                break
            }
        }

        match wf {
            "A" => part.into_values().sum::<_>(),
            "R" => 0,
            _ => panic!("unexpected state")
        }
    }).sum::<i32>());
}
