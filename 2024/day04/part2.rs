use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let coords = BufReader::new(File::open("day04.txt").expect("what could be happen?"))
        .lines()
        .map(|b| b.unwrap())
        .map(|line| line.into_bytes().into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starts = coords
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, symbol)| symbol == &&b'A')
                .map(|(col, _)| (row as i32, col as i32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = starts
        .into_iter()
        .filter(|(row, col)| {
            [-1, 1]
                .into_iter()
                .flat_map(|row_add| [-1, 1].map(|col_add| (row_add, col_add)))
                .filter(|(row_add, col_add)| {
                    let row_m = row + *row_add as i32;
                    let col_m = col + *col_add as i32;
                    let row_s = row + (*row_add as i32) * -1;
                    let col_s = col + (*col_add as i32) * -1;
                    contains_symbol(&coords, b'M', (row_m, col_m))
                        && contains_symbol(&coords, b'S', (row_s, col_s))
                })
                .count()
                == 2
        })
        .count();

    println!("{}", result);
}

fn contains_symbol(coords: &Vec<Vec<u8>>, symbol: u8, (row, col): (i32, i32)) -> bool {
    row >= 0
        && col >= 0
        && coords
            .get(row as usize)
            .map(|row| row.get(col as usize).map(|s| s == &symbol).unwrap_or(false))
            .unwrap_or(false)
}
