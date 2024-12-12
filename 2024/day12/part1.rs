use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let field = BufReader::new(File::open("day12.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.into_bytes())
        .collect::<Vec<_>>();

    let mut st = Vec::new();
    st.push((0, 0));
    let mut visited = HashSet::new();
    let mut res = 0;

    while let Some(start) = st.pop() {
        if get_cell(&field, start).is_none() || visited.contains(&start) {
            continue;
        }
        println!("look at ({}, {})", start.0, start.1);
        let mut st_inner = Vec::new();
        st_inner.push(start);
        let mut perimeter = 0;
        let mut area = 0;
        while let Some(cur) = st_inner.pop() {
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur);
            area += 1;
            let mut local_perimeter = 0;
            for add in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let next = (cur.0 + add.0, cur.1 + add.1);
                if get_cell(&field, next)
                    .is_some_and(|x| get_cell(&field, cur).is_some_and(|y| x == y))
                {
                    st_inner.push(next);
                } else {
                    local_perimeter += 1;
                    st.push(next);
                }
            }
            perimeter += local_perimeter;
        }
        res += perimeter * area;
    }

    println!("{}", res);
}

fn get_cell(field: &Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field.get(x as usize).and_then(|line| line.get(y as usize))
}
