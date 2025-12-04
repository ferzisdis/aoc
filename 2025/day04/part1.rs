use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let m = BufReader::new(File::open("day04.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.into_bytes())
        .collect::<Vec<Vec<u8>>>();

    let counted_m = m
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

    println!(
        "{}",
        counted_m
            .into_iter()
            .flat_map(|x| x.into_iter())
            .filter(|x| x != &-1 && x < &4)
            .count()
    );
}
