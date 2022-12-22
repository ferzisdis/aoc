use std::{env, fs};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::Peekable;
use crate::Robot::{GeodeRobot, ObsidianRobot, OreRobot};

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

#[derive(Debug)]
enum Robot {
    OreRobot(i32),
    ClayRobot(i32),
    ObsidianRobot(i32, i32),
    GeodeRobot(i32, i32),
    NoBuild
}

fn main() {
    let mut inputs = BufReader::new(File::open("day19.txt").expect("I know you are existed"))
        .lines()
        .map(|x| x.expect("I can do it!"))
        .map(|s| {
            let mut it = s.into_bytes().into_iter();
            let mut robots = Vec::new();
            read_n(&mut it, "Each ore robot costs ".len());
            robots.push(Robot::OreRobot(read_int(&mut it)));
            read_n(&mut it, "ore. Each clay robot costs ".len());
            robots.push(Robot::ClayRobot(read_int(&mut it)));
            read_n(&mut it, "ore. Each obsidian robot costs ".len());
            let obs = read_int(&mut it);
            read_n(&mut it, "ore and ".len());
            robots.push(Robot::ObsidianRobot(obs, read_int(&mut it)));
            read_n(&mut it, "clay. Each geode robot costs ".len());
            let geo = read_int(&mut it);
            read_n(&mut it, "ore and ".len());
            robots.push(Robot::GeodeRobot(geo, read_int(&mut it)));

            robots.push(Robot::NoBuild);
            robots
        });


    let mut res = 0;
    for (id, robots) in inputs.enumerate() {
        println!("{:?}", robots);
        let val = slove(&mut HashMap::new(), &robots, 1, (0, 1), (0, 0), (0, 0), (0, 0));
        println!("calculated id {} with res {}", id + 1, val);
        res += (id + 1) as i32 * val;
    }

    println!("res {}", res);
}

fn slove(dp: &mut HashMap<i32, HashMap<((i32, i32),(i32, i32),(i32, i32),(i32, i32)), i32>>,
         robots: &Vec<Robot>,
         minute: i32,
         (ore_cnt, ores_rbt): (i32, i32),
         (clay_cnt, clay_rbt): (i32, i32),
         (obs_cnt, obs_rbt): (i32, i32),
         (geod_cnt, geod_rbt): (i32, i32),
) -> i32 {
    if minute == 24 {
        return geod_cnt + geod_rbt
    }
    if !dp.contains_key(&minute) {
        dp.insert(minute, HashMap::new());
    }

    let key = ((ore_cnt, ores_rbt), (clay_cnt, clay_rbt), (obs_cnt, obs_rbt), (geod_cnt, geod_rbt));
    match dp[&minute].get(&key).clone() {
        Some(val) => {
            *val
        }
        None => {
            let mut res = 0;
            for robot in robots.iter().rev() {
                if let  Robot::GeodeRobot(ore_cost, obsidian_cost) = robot {
                    if *ore_cost <= ore_cnt && *obsidian_cost <= obs_cnt {
                        res = res.max(slove(dp, robots,minute + 1,
                                            (ore_cnt - *ore_cost + ores_rbt, ores_rbt),
                                            (clay_cnt + clay_rbt, clay_rbt),
                                            (obs_cnt - *obsidian_cost + obs_rbt, obs_rbt),
                                            (geod_cnt + geod_rbt, geod_rbt + 1)
                        ));
                        if *ore_cost <= ore_cnt / 3 && *obsidian_cost <= obs_cnt / 2  {
                            break
                        }
                    }
                }

                if let Robot::ObsidianRobot(ore_cost, clay_cost) = robot {
                    if *ore_cost <= ore_cnt && *clay_cost <= clay_cnt {
                        res = res.max(slove(dp, robots,minute + 1,
                                            (ore_cnt - *ore_cost + ores_rbt, ores_rbt),
                                            (clay_cnt - *clay_cost + clay_rbt, clay_rbt),
                                            (obs_cnt + obs_rbt, obs_rbt + 1),
                                            (geod_cnt + geod_rbt, geod_rbt)
                        ));

                        if *ore_cost <= ore_cnt / 2 && *clay_cost <= clay_cnt / 2 {
                            break
                        }
                    }
                }

                if let Robot::ClayRobot(ore_cost) = robot {
                    if *ore_cost <= ore_cnt {
                        res = res.max(slove(dp, robots,minute + 1,
                                                  (ore_cnt - *ore_cost + ores_rbt, ores_rbt),
                                                  (clay_cnt + clay_rbt, clay_rbt + 1),
                                                  (obs_cnt + obs_rbt, obs_rbt),
                                                  (geod_cnt + geod_rbt, geod_rbt)
                        ))
                    }
                }
                if let Robot::OreRobot(ore_cost) = robot {
                    if *ore_cost <= ore_cnt {
                        res = res.max(slove(dp, robots, minute + 1,
                              (ore_cnt - *ore_cost + ores_rbt, ores_rbt + 1),
                              (clay_cnt + clay_rbt, clay_rbt),
                              (obs_cnt + obs_rbt, obs_rbt),
                              (geod_cnt + geod_rbt, geod_rbt)
                        ))
                    }
                }

                if let Robot::NoBuild  = robot {
                    res = res.max(slove(dp, robots,minute + 1,
                                        (ore_cnt + ores_rbt, ores_rbt),
                                        (clay_cnt + clay_rbt, clay_rbt),
                                        (obs_cnt + obs_rbt, obs_rbt),
                                        (geod_cnt + geod_rbt, geod_rbt)
                    ))
                }
            }
            let mut v = dp.get_mut(&minute).expect("");
            v.insert(key, res);
            res
        }
    }
}

