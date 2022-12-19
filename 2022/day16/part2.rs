use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;

fn read_int(it: &mut impl Iterator<Item = u8>) -> i32
{
    let mut res: i32 = 0;
    let mut is_neg = false;
    for b in it {
        match b {
            b'0'..=b'9' => {
                res = res * 10 + i32::from(b - b'0')
            }
            b'-' => { is_neg = true }
            _ => break
        }
    }
    return if is_neg { -res } else { res };
}

fn read_word(it: &mut impl Iterator<Item = u8>)
{
    for b in it {
        match b {
            b' ' => break,
            _ => continue
        }
    }
}

fn read_n(it: &mut impl Iterator<Item = u8>, n: usize) -> Option<String> {
    let mut s = String::new();
    s.reserve(n);

    for _ in 0..n {
        if let Some(b) = it.next() {
            s.push(char::from(b));
        } else {
            return None;
        }
    }

    Some(s)
}

fn main() {
    let inputs = BufReader::new(File::open("day16.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|s| {
            let mut it = s.into_bytes().into_iter();
            read_n(&mut it, "Valve ".len());
            let from = read_n(&mut it, 2);
            read_n(&mut it, " has flow rate=".len());
            let rate = read_int(&mut it);
            read_word(&mut it); // ' '
            read_word(&mut it); // 'tunnel(s) '
            read_word(&mut it); // 'lead(s) '
            read_word(&mut it); // 'to '
            read_word(&mut it); // 'valve(s) '
            let mut to = Vec::new();
            loop {
                if let Some(s) = read_n(&mut it, 2) {
                    to.push(s);
                } else {
                    break
                }
                read_n(&mut it, ", ".len());
            }

            (from.expect("known format"), rate, to)
        });

    let mut indexes = HashMap::new();
    let mut valves_rate = Vec::new();
    let mut valves_data = Vec::new();

    for (valve, rate, to) in inputs {

        let idx_valve = match indexes.get(&valve) {
            Some(i) => *i,
            None => {
                let idx_valve = indexes.len();
                indexes.insert(valve, idx_valve);
                valves_data.push((999, Vec::new()));
                idx_valve
            }
        };

        let idx_valve_with_rate = match rate {
            i if i > 0 => {
                valves_rate.push(rate as usize);
                valves_rate.len() - 1
            },
            _ => 999
        };

        let mut neighbors = Vec::new();
        for t in to {
            neighbors.push(match indexes.get(&t) {
                Some(i) => *i,
                None => {
                    let idx_valve = indexes.len();
                    indexes.insert(t, idx_valve);
                    valves_data.push((999, Vec::new()));
                    idx_valve
                }
            });
        }

        valves_data[idx_valve] = (idx_valve_with_rate, neighbors);
    }

    println!("idxs: {:?}", indexes);

    let mut dp_prev: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut dp_prev_prev: Vec<Vec<Vec<usize>>> = Vec::new();

    for minute in 1..=26 {
        println!("minute {}", minute);
        let mut dp_cur = Vec::new();
        let t = valves_data.iter().map(|_| 0).collect::<Vec<usize>>();
        dp_cur.resize(1 << valves_rate.len(), valves_data.iter().map(|_| t.clone()).collect::<Vec<Vec<usize>>>());

        for state in 0..(1 << valves_rate.len()) {
            for (position_my, (idx_rate_my, paths_my)) in valves_data.iter().enumerate() {
                for (position_el, (idx_rate_el, paths_el)) in valves_data.iter().enumerate() {
                    if minute == 1 {
                        continue
                    }

                    if minute == 2 {
                        if *idx_rate_my != 999 && (state & 1 << *idx_rate_my) == 0 {
                            dp_cur[state][position_my][position_el] += (minute - 1) * valves_rate[*idx_rate_my];
                        }
                        if *idx_rate_my != *idx_rate_el && *idx_rate_el != 999 && (state & 1 << *idx_rate_el) == 0 {
                            dp_cur[state][position_my][position_el] += (minute - 1) * valves_rate[*idx_rate_el];
                        }
                        continue
                    }

                    let mut max_score = if *idx_rate_my != *idx_rate_el &&
                        *idx_rate_my != 999 && (state & 1 << *idx_rate_my) == 0 &&
                        *idx_rate_el != 999 && (state & 1 << *idx_rate_el) == 0 {
                        dp_prev[state | (1 << *idx_rate_my) | (1 << *idx_rate_el)][position_my][position_el] +
                        (minute - 1) * valves_rate[*idx_rate_my] + (minute - 1) * valves_rate[*idx_rate_el]
                    } else { 0 };

                    max_score = max_score.max(if *idx_rate_my != 999 && (state & 1 << *idx_rate_my) == 0 {
                        let mut max_score = 0;
                        for path_el in paths_el {
                            max_score = max_score.max(dp_prev[state | (1 << *idx_rate_my)][position_my][*path_el] + (minute - 1) * valves_rate[*idx_rate_my]);
                        }
                        max_score
                    } else {
                        0
                    });

                    max_score = max_score.max(if *idx_rate_my != *idx_rate_el && *idx_rate_el != 999 && (state & 1 << *idx_rate_el) == 0 {
                        let mut max_score = 0;
                        for path_my in paths_my {
                            max_score = max_score.max(dp_prev[state | (1 << *idx_rate_el)][*path_my][position_el] + (minute - 1) * valves_rate[*idx_rate_el]);
                        }
                        max_score
                    } else {
                        0
                    });

                    for path_my in paths_my {
                        for path_el in paths_el {
                            max_score = max_score.max(dp_prev[state][*path_my][*path_el]);
                        }
                    }
                    dp_cur[state][position_my][position_el] = max_score;
                }



            }
        }
        println!("current {:?}", dp_cur[0]);
        dp_prev_prev = dp_prev;
        dp_prev = dp_cur;
    }

    println!("{:?}", dp_prev[0][indexes["AA"]][indexes["AA"]])
}

