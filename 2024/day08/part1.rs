use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let field = BufReader::new(File::open("day08.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.into_bytes())
        .collect::<Vec<_>>();

    let groups = field
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(move |(y, val)| (x as i32, y as i32, val.clone()))
        })
        .fold(HashMap::new(), |mut acc, (x, y, val)| {
            acc.entry(val).or_insert(Vec::new()).push((x, y));
            acc
        });

    let res = groups
        .into_iter()
        .filter(|(key, _)| key != &b'.')
        .flat_map(|(_, val)| {
            let mut combs = Vec::new();
            let mut it = val.into_iter();
            while let Some((l_x, l_y)) = it.next() {
                let mut it_inner = it.clone();
                while let Some((r_x, r_y)) = it_inner.next() {
                    combs.push((r_x + (r_x - l_x), r_y + (r_y - l_y)));
                    combs.push((l_x - (r_x - l_x), l_y - (r_y - l_y)));
                }
            }
            combs
        })
        .filter(|coord| get_cell(&field, coord.clone()).is_some())
        .collect::<HashSet<_>>();

    println!("{}", res.len());
}

fn get_cell(field: &Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field.get(x as usize).and_then(|line| line.get(y as usize))
}
