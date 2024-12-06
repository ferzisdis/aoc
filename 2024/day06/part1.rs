use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let field = BufReader::new(File::open("day06.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cur = field
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, byte)| byte == &&b'^')
                .map(|(col, _)| (row as i32, col as i32))
                .collect::<Vec<_>>()
        })
        .into_iter()
        .next()
        .unwrap();

    let mut direction = 0;
    let mut visited = HashSet::new();
    while cur.0 >= 0
        && cur.1 >= 0
        && field
            .get(cur.0 as usize)
            .map(|line| line.get(cur.1 as usize))
            .is_some()
    {
        println!("visit ({},{})", cur.0, cur.1);
        visited.insert(cur);

        let next = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .get(direction)
            .map(|add| (cur.0 + add.0, cur.1 + add.1))
            .unwrap();
        if next.0 >= 0
            && next.1 >= 0
            && field
                .get(next.0 as usize)
                .and_then(|line| line.get(next.1 as usize))
                .map(|x| x == &b'#')
                .unwrap_or(false)
        {
            direction = (direction + 1).rem_euclid(4)
        } else {
            cur = next
        }
    }

    println!("{}", visited.len());
}
