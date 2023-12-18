use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let map = BufReader::new(File::open("day18.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            let mut it = x.split_ascii_whitespace();
            it.next();
            it.next();

            let mut it_str = it.next().unwrap().chars();
            it_str.next(); // skip (
            it_str.next(); // skip #
            it_str.next_back(); // skip )

            (String::from(match it_str.next_back().unwrap() {
                '0' => "R", '1' => "D", '2' => "L", '3' => "U", _ => panic!("unexpected token")
            }), it_str.fold(0, |acc, b| acc * 16 + b.to_digit(16).unwrap() as u64))
        }).collect::<Vec<_>>();

    let ((i_lens, i_zero), (j_lens, j_zero)) = make_lenses(&map);

    let mut visited = HashSet::new();
    let mut left_hand: VecDeque<(i32, i32)> = VecDeque::new();
    let mut last = (i_zero, j_zero);
    visited.insert(last);

    for (dir, mut cnt) in map {
        while cnt > 0 {
            let next = match dir.as_str() {
                "R" => {
                    left_hand.push_back(((last.0 - 1) as i32, (last.1 + 1) as i32));
                    cnt -= j_lens.get(last.1 + 1).unwrap();
                    (last.0, last.1 + 1)
                },
                "L" => {
                    left_hand.push_back(((last.0 + 1) as i32, (last.1 - 1) as i32));
                    cnt -= j_lens.get(last.1 - 1).unwrap();
                    (last.0, last.1 - 1)
                },
                "D" => {
                    left_hand.push_back(((last.0 + 1) as i32, (last.1 + 1) as i32));
                    cnt -= i_lens.get(last.0 + 1).unwrap();
                    (last.0 + 1, last.1)
                },
                "U" => {
                    left_hand.push_back(((last.0 - 1) as i32, (last.1 - 1) as i32));
                    cnt -= i_lens.get(last.0 - 1).unwrap();
                    (last.0 - 1, last.1)
                },
                _ => panic!("unexpected token {}", dir.as_str())
            };
            visited.insert(next);
            last = next;
        }
    }

    let (i_min, i_max, j_min, j_max) = (
        visited.iter().map(|(i, j)| *i).min().unwrap() - 1,
        visited.iter().map(|(i, j)| *i).max().unwrap() + 1,
        visited.iter().map(|(i, j)| *j).min().unwrap() - 1,
        visited.iter().map(|(i, j)| *j).max().unwrap() + 1,
    );

    let border_len = visited.iter().map(|(i, j)| {
        *i_lens.get(*i).unwrap() * *j_lens.get(*j).unwrap()
    }).sum::<u64>();

    for i in i_min..=i_max {
        println!("{}", String::from_utf8((j_min..=j_max).map(|j| {
            if visited.contains(&(i, j)) {
                b'#'
            } else {
                b'.'
            }
        }).collect::<Vec<_>>()).unwrap())
    }

    println!("left hand cnt: {}, border cnt: {}", left_hand.len(), border_len);

    while !left_hand.is_empty() {
        let cur = left_hand.pop_front().unwrap();
        if cur.0 < i_min as i32 || cur.1 < j_min as i32 || cur.0 > i_max as i32 || cur.1 > j_max as i32 {
            continue
        }
        if visited.contains(&(cur.0 as usize, cur.1 as usize)) {
            continue
        }
        visited.insert((cur.0 as usize, cur.1 as usize));

        for add in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
            left_hand.push_back((cur.0 + add.0, cur.1 + add.1));
        }
    }

    for i in i_min..=i_max {
        println!("{}", String::from_utf8((j_min..=j_max).map(|j| {
            if visited.contains(&(i, j)) {
                b'#'
            } else {
                b'.'
            }
        }).collect::<Vec<_>>()).unwrap())
    }

    let mut res = 0u64;
    let is_outer = visited.contains(&(i_min, j_min));
    for i in i_min..=i_max {
        for j in j_min..=j_max {
            res += if (is_outer && !visited.contains(&(i, j))) || (!is_outer && visited.contains(&(i, j))) {
                *i_lens.get(i).unwrap() * *j_lens.get(j).unwrap()
            } else {
                0
            }
        }
    }
    if is_outer {
        res += border_len
    }

    println!("{}", res);
}

fn make_lenses(map: &Vec<(String, u64)>) -> ((Vec<u64>, usize), (Vec<u64>, usize)) {
    let hor_points = map.iter().fold((HashSet::new(), 0i32), |(mut hs, mut pos), (dir, cnt)| {
        hs.insert(pos);
        pos = match dir.as_str() {
            "R" => { hs.insert(pos + 1); hs.insert(pos + *cnt as i32); pos + *cnt as i32 },
            "L" => { hs.insert(pos - 1); hs.insert(pos - *cnt as i32); pos - *cnt as i32 },
            _ => pos,
        };
        (hs, pos)
    }).0.into_iter().collect::<Vec<_>>();

    let ver_points = map.iter().fold((HashSet::new(), 0i32), |(mut hs, mut pos), (dir, cnt)| {
        hs.insert(pos);
        pos = match dir.as_str() {
            "D" => { hs.insert(pos + 1); hs.insert(pos + *cnt as i32); pos + *cnt as i32 },
            "U" => { hs.insert(pos - 1); hs.insert(pos - *cnt as i32); pos - *cnt as i32 },
            _ => pos,
        };
        (hs, pos)
    }).0.into_iter().collect::<Vec<_>>();

    (to_lenses(ver_points), to_lenses(hor_points))
}

fn to_lenses(mut points: Vec<i32>) -> (Vec<u64>, usize) {
    points.sort();
    let mut res: Vec<u64> = Vec::new();
    let mut zero_pos = 0;
    let mut prev = 0;
    for point in points {
        if res.is_empty() {
            res.push(1); // padding
            prev = point;
            continue
        }
        if prev == 0 {
            zero_pos = res.len();
        }

        res.push(1); // for prev
        if point - prev > 1 {
            res.push((point - prev - 1) as u64);
        }
        prev = point;
    }
    res.push(1); // for last
    res.push(1); // for padding

    (res, zero_pos)
}
