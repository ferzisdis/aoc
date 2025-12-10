use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    ops::ControlFlow,
    u32, vec,
};

fn next_permutation(st: &mut VecDeque<usize>, max_len: usize) -> bool {
    if st.len() == 0 {
        st.push_back(0);
        return true;
    }

    let back = st.pop_back().unwrap();
    if back + 1 < max_len {
        st.push_back(back + 1);
        return true;
    }

    next_permutation(st, max_len - 1);
    assert!(*st.back().unwrap() < max_len + 1);
    st.push_back(st.back().unwrap() + 1);
    st.len() < max_len
}

fn next_increment(
    v: &mut Vec<u32>,
    vars: &Vec<usize>,
    buttons: &Vec<u32>,
    joltages: &Vec<u32>,
) -> bool {
    let mut l = vars.len();
    for i in (0..vars.len()).rev() {
        if v[vars[i]] > 0 {
            v[vars[i]] -= 1;
            break;
        }
        v[vars[i]] = 0;
        l = i;
    }

    for j in l..vars.len() {
        let max_val = joltages
            .iter()
            .enumerate()
            .filter(|(jolt_idx, _)| buttons[vars[j]] & (1 << *jolt_idx) != 0)
            .map(|(jolt_idx, jolt_val)| {
                let others_sum = vars
                    .iter()
                    .filter(|other| **other != vars[j] && buttons[**other] & (1 << jolt_idx) != 0)
                    .map(|other| v[*other])
                    .sum::<u32>();

                if others_sum < *jolt_val {
                    *jolt_val - others_sum
                } else {
                    0
                }
            })
            .min()
            .unwrap();
        v[vars[j]] = max_val;
    }

    l > 0
}

fn main() {
    let res = BufReader::new(File::open("day10.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let mut it = x.split(' ');
            let signal = it.next().unwrap();
            let mut buttons = Vec::new();
            let mut joltage = Option::None;
            while let Some(part) = it.next() {
                if part.starts_with('(') {
                    buttons.push(part);
                } else {
                    joltage = Some(part);
                    break;
                }
            }

            (
                signal[1..signal.len() - 1]
                    .bytes()
                    .rev()
                    .fold(0u32, |acc, b| {
                        if b == b'#' {
                            acc << 1 | 1
                        } else {
                            acc << 1 | 0
                        }
                    }),
                buttons
                    .into_iter()
                    .map(|x| {
                        x[1..x.len() - 1]
                            .split(',')
                            .map(|x| x.parse::<u32>().unwrap())
                            .fold(0u32, |acc, x| acc | (1 << x))
                    })
                    .collect::<Vec<_>>(),
                joltage.unwrap()[1..joltage.unwrap().len() - 1]
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(_, buttons, joltage)| {
            let mut st = VecDeque::new();
            let mut unknowns = (0..joltage.len())
                .map(|j| {
                    buttons.iter().enumerate().fold(0u32, |acc, (i, b)| {
                        if b & (1 << j) != 0 {
                            acc | (1 << i)
                        } else {
                            acc
                        }
                    })
                })
                .collect::<Vec<_>>();

            println!("try found {:?}", joltage);
            println!("start permutations");
            while next_permutation(&mut st, buttons.len()) {
                let mut initial_mask = st.iter().fold(0u32, |acc, &x| acc | 1 << x);
                let mut unknowns = unknowns.clone();
                let mut was_removed = true;

                while was_removed {
                    was_removed = false;
                    for i in 0..buttons.len() {
                        let mask = initial_mask | (1 << i);
                        for (_, extracted) in unknowns
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, val)| **val != 0 && **val & mask == **val)
                        {
                            was_removed = true;
                            *extracted = 0;
                        }

                        if was_removed {
                            initial_mask = mask;
                            break;
                        }
                    }
                }
                if unknowns.iter().all(|val| *val == 0) {
                    break;
                }
            }
            println!("finish permutations: {:?}", st);
            if st.len() > buttons.len() {
                panic!("Too many buttons {}", st.len());
            }

            let mut solve_orders = Vec::new();

            let mut initial_mask = st.iter().fold(0u32, |acc, &x| acc | 1 << x);
            while !unknowns.iter().all(|val| *val == 0) {
                // println!("initial mask {}", initial_mask);
                for i in 0..buttons.len() {
                    let mask = initial_mask | (1 << i);
                    let mut was_removed = false;
                    for (num, extracted) in unknowns
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, val)| **val != 0 && **val & mask == **val)
                    {
                        was_removed = true;
                        solve_orders.push((
                            num,
                            i,
                            (0..buttons.len())
                                .filter(|&idx| idx != i && *extracted & (1 << idx) != 0)
                                .collect::<Vec<_>>(),
                        ));
                        *extracted = 0;
                    }
                    if was_removed {
                        println!("solve orders: {:?}, unknowns {:?}", solve_orders, unknowns);
                        initial_mask = mask;
                        break;
                    }
                }
            }

            let mut buttons_order = st.into_iter().collect::<Vec<_>>();
            let scores = buttons_order
                .iter()
                .map(|button_id| {
                    let mut res = vec![0; buttons.len()];
                    let (_, base_key, _) = solve(&joltage, &solve_orders, &res);
                    res[*button_id] = 1;
                    let (_, key, _) = solve(&joltage, &solve_orders, &res);
                    println!("for button {}: {}, {}", button_id, base_key, key);
                    (*button_id, key - base_key)
                })
                .collect::<Vec<_>>();

            println!("solve_orders: {:?}, st: {:?}", solve_orders, buttons_order);

            let mut min_nums = u32::MAX;
            let mut min_score = i32::MAX;
            let mut res = vec![0; buttons.len()];

            next_increment(&mut res, &buttons_order, &buttons, &joltage);
            res[buttons_order[buttons_order.len() - 1]] += 1;
            println!("initial res {:?}", res);
            while next_increment(&mut res, &buttons_order, &buttons, &joltage) {
                let score = scores
                    .iter()
                    .map(|(button_id, score)| res[*button_id] as i32 * *score)
                    .sum::<i32>();
                if score > min_score {
                    continue;
                }

                let (is_valid, total, total_res) = solve(&joltage, &solve_orders, &res);
                if is_valid && (total as u32) < min_nums {
                    println!("intermediate result: {} ({:?})", total, total_res);
                    let mut for_assert = vec![0; joltage.len()];
                    for (i, val) in total_res.iter().enumerate() {
                        for j in 0..joltage.len() {
                            if buttons[i] & (1 << j) != 0 {
                                for_assert[j] += *val as u32;
                            }
                        }
                    }
                    // assert_eq!(joltage, for_assert);
                    if joltage != for_assert {
                        println!("skip invalid");
                        continue;
                    }
                    min_nums = total as u32;
                    min_score = score;
                }
            }

            println!("found result: {}", min_nums);
            min_nums
        })
        .sum::<u32>();

    println!("{}", res)
}

fn solve(
    joltage: &Vec<u32>,
    solve_orders: &Vec<(usize, usize, Vec<usize>)>,
    res: &Vec<u32>,
) -> (bool, i32, Vec<i32>) {
    let mut valid = true;
    let mut res = res.iter().map(|x| *x as i32).collect::<Vec<_>>();
    for (jolt_idx, button_idx, desp) in solve_orders.iter() {
        let sum = desp.iter().map(|&x| res[x]).sum::<i32>();
        let count = joltage[*jolt_idx] as i32 - sum;
        if res[*button_idx] != 0 || res[*button_idx] != count {
            // valid = false;
        }
        res[*button_idx] = count;
        // println!("res {:?}", res);
    }
    // println!("res {:?}", res);
    (
        res.iter().all(|x| *x >= 0) && valid,
        res.iter().sum::<i32>(),
        res,
    )
}
