use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let coords = BufReader::new(File::open("day04.txt").expect("what could be happen?"))
        .lines()
        .map(|b| b.unwrap())
        .enumerate()
        .map(|(row, line)| line.into_bytes().into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starts = coords
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, symbol)| symbol == &&b'X')
                .map(|(col, _)| (row as i32, col as i32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = starts
        .into_iter()
        .map(|(row, col)| {
            [-1, 0, 1]
                .into_iter()
                .flat_map(|row_add| [-1, 0, 1].map(|col_add| (row_add, col_add)))
                .filter(|(row_add, col_add)| {
                    [b'M', b'A', b'S']
                        .into_iter()
                        .enumerate()
                        .all(|(idx, symbol)| {
                            let row = row + (idx as i32 + 1) * *row_add as i32;
                            let col = col + (idx as i32 + 1) * *col_add as i32;
                            row >= 0
                                && col >= 0
                                && coords
                                    .get(row as usize)
                                    .map(|row| {
                                        row.get(col as usize).map(|s| s == &symbol).unwrap_or(false)
                                    })
                                    .unwrap_or(false)
                        })
                })
                .count()
        })
        .sum::<usize>();

    println!("{}", result);
}
