use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let field = BufReader::new(File::open("day16.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.into_bytes())
        .collect::<Vec<_>>();

    let start = (field.len() as i32 - 2, 1);
    let best_score = get_min_score(&field, start);
    let mut path = HashMap::new();
    get_best_tiles(&field, &mut path, best_score as i32, start, (0, 1));
    println!(
        "{}",
        path.into_iter()
            .filter(|(_, v)| v.1 == true)
            .map(|((pos, _), _)| pos)
            .collect::<HashSet<_>>()
            .len()
    )
}

fn get_best_tiles(
    field: &Vec<Vec<u8>>,
    path: &mut HashMap<((i32, i32), (i32, i32)), (i32, bool)>,
    score: i32,
    cur: (i32, i32),
    v: (i32, i32),
) -> bool {
    if score < 0 {
        return false;
    }
    let cache_key = (cur, v);
    if path.contains_key(&cache_key) {
        let cache_val = path[&cache_key];
        if cache_val.1 == true {
            return cache_val.0 == score;
        }
        if cache_val.0 >= score {
            return cache_val.1;
        }
    }

    let res = match get_cell(field, cur) {
        Some(&b'#') => false,
        Some(&b'E') => score == 0,
        Some(&b'S' | &b'.') => {
            get_best_tiles(field, path, score - 1, (cur.0 + v.0, cur.1 + v.1), v)
                | get_best_tiles(field, path, score - 1000, cur, (v.1, v.0))
                | get_best_tiles(field, path, score - 1000, cur, (v.1 * -1, v.0 * -1))
        }
        _ => unreachable!(),
    };
    path.insert(cache_key, (score, res));
    return res;
}

fn get_min_score(field: &Vec<Vec<u8>>, start: (i32, i32)) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start, (0, 1))));
    let mut visited = HashSet::new();

    while let Some(Reverse((score, cur, v))) = heap.pop() {
        if visited.contains(&(cur, v)) {
            continue;
        }
        visited.insert((cur, v));
        match get_cell(field, cur) {
            Some(&b'#') => (),
            Some(&b'E') => return score,
            Some(&b'S' | &b'.') => {
                heap.push(Reverse((score + 1, (cur.0 + v.0, cur.1 + v.1), v)));
                heap.push(Reverse((score + 1000, cur, (v.1, v.0))));
                heap.push(Reverse((score + 1000, cur, (v.1 * -1, v.0 * -1))));
            }
            _ => unreachable!(),
        }
    }

    unreachable!()
}

fn get_cell(field: &Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field.get(x as usize).and_then(|line| line.get(y as usize))
}
