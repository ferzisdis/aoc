use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let m = BufReader::new(File::open("day04.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.into_bytes())
        .collect::<Vec<Vec<u8>>>();

    let mut counted_m = m
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, cur)| {
                    if cur == &b'@' {
                        [
                            (-1, -1),
                            (0, -1),
                            (1, -1),
                            (-1, 0),
                            (1, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                        ]
                        .into_iter()
                        .filter_map(|(add_i, add_j)| {
                            m.get(i.wrapping_add_signed(add_i))
                                .and_then(|row| row.get(j.wrapping_add_signed(add_j)).copied())
                        })
                        .filter(|val| val == &b'@')
                        .count() as i32
                    } else {
                        -1
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut vd = counted_m
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, x)| x != &&-1 && x < &&4)
                .map(|(j, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .collect::<VecDeque<_>>();

    let mut res = 0;
    while let Some((i, j)) = vd.pop_front() {
        if counted_m[i][j] == -1 {
            continue;
        }
        res += 1;
        counted_m[i][j] = -1;

        for (add_i, add_j) in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
            if let Some(next_i) = i.checked_add_signed(add_i) {
                if let Some(next_j) = j.checked_add_signed(add_j) {
                    if let Some(next_val) = counted_m
                        .get_mut(next_i)
                        .and_then(|row| row.get_mut(next_j))
                    {
                        if next_val == &-1 {
                            continue;
                        }
                        *next_val -= 1;
                        if *next_val < 4 {
                            vd.push_back((next_i, next_j));
                        }
                    }
                }
            }
        }
    }

    println!("{}", res);
}
