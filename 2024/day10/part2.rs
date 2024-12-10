use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let field = BufReader::new(File::open("day10.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.into_bytes())
        .collect::<Vec<_>>();

    let result = field
        .iter()
        .enumerate()
        .flat_map(|(x, line)| line.iter().enumerate().map(move |(y, val)| (x, y, val)))
        .filter(|(_, _, val)| val == &&b'0')
        .map(|(x, y, _)| found_hikings(&field, (x as i32, y as i32)))
        .sum::<usize>();

    println!("{}", result);
}

fn found_hikings(field: &Vec<Vec<u8>>, start: (i32, i32)) -> usize {
    let mut tops = Vec::new();
    let mut st = Vec::new();
    st.push(start);

    while let Some(cur) = st.pop() {
        match get_cell(field, cur) {
            Some(&b'9') => {
                tops.push(cur);
            }
            Some(val) => {
                for add in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    let next = (cur.0 + add.0, cur.1 + add.1);
                    if get_cell(field, next)
                        .map(|x| x == &(val + 1))
                        .unwrap_or(false)
                    {
                        st.push(next);
                    }
                }
            }
            _ => (),
        }
    }

    tops.len()
}

fn get_cell(field: &Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field.get(x as usize).and_then(|line| line.get(y as usize))
}
