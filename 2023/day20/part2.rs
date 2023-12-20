use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Signal::{High, Low};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Signal {
    Low, High
}

enum Module {
    FlipFlop(bool), Conjunction(HashMap<String, Signal>), Broadcaster, Output
}

fn main() {
    let mut lines = BufReader::new(File::open("day20.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"));

    let mut map = HashMap::new();
    for line in lines {
        if let Some((left, right)) = line.split_once(" -> ") {
            let (module, name) = if left.starts_with('%') {
                (Module::FlipFlop(false), left.trim_start_matches('%'))
            } else if left.starts_with('&') {
                (Module::Conjunction(HashMap::new()), left.trim_start_matches('&'))
            } else {
                (Module::Broadcaster, left)
            };

            let mut connections = right.split(", ")
                .map(|x| x.to_string()).collect::<Vec<_>>();
            map.insert(name.to_string(), (module, connections));
        }
    }
    map.insert("output".to_string(), (Module::Output, Vec::new()));
    map.insert("rx".to_string(), (Module::Output, Vec::new()));

    let mut updates = map.iter().flat_map(|(k, (_, connections))| {
        connections.iter().filter_map(|conncection| {
            map.get(conncection.as_str()).and_then(|(m, _)| match m {
                Module::Conjunction(_) => Some(conncection.to_string()),
                _ => None
            }).map(|x| (x, k.to_string()))
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    for (k, v) in updates {
        if let Some((Module::Conjunction(hs),_)) = map.get_mut(k.as_str()) {
            hs.insert(v, Signal::Low);
        }
    }

    let mut counters = HashMap::new();

    for i in 1..100000000 {
        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), Signal::Low, "broadcaster".to_string()));

        while let Some((from, signal, to)) = queue.pop_front() {
            // println!("{} {:?} {}", from, signal, to);

            // *counters.get_mut(&signal).unwrap() += 1;

            if !map.contains_key(to.as_str()) {
                panic!("unknown key [{}]", to)
            }
            let (module, connections) = map.get_mut(to.as_str()).unwrap();
            let next_signal = match module {
                Module::FlipFlop(state) => {
                    if signal == Signal::High {
                        continue; // ignore
                    }
                    if *state {
                        *state = false;
                        Signal::Low
                    } else {
                        *state = true;
                        Signal::High
                    }
                },
                Module::Conjunction(hs) => {
                    if to == "tj" {
                        if counters.len() == 0 {
                            for (k, _) in hs.iter() {
                                counters.insert(k.to_string(), 0);
                            }
                        }
                        if signal == High {
                            *counters.get_mut(from.as_str()).unwrap() = i as u64;
                        }
                    }

                    *hs.get_mut(from.as_str()).unwrap() = signal;
                    if hs.values().all(|x| *x == Signal::High) {
                        Signal::Low
                    } else {
                        Signal::High
                    }
                },
                Module::Output => continue,
                Module::Broadcaster => signal,
            };

            for connection in connections.iter() {
                queue.push_back((to.to_string(), next_signal, connection.to_string()));
            }
        }

        if counters.values().all(|x| *x > 0) {
            break
        }

        // println!("========");
    }

    println!("{}", counters.values().fold(1u64, |acc, x| lcm(acc, *x)))
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
