use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("day21.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"));

    let mut start = (0, 0);
    let mut plots = HashSet::new();

    let (mut width, mut height) = (0, 0);
    for (i, line) in lines.enumerate() {
        (width, height) = (line.len() as i32, (i + 1) as i32);
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (i as i32, j as i32);
                plots.insert((i, j));
            }
            if ch == '.' {
                plots.insert((i, j));
            }
        }
    }




    let mut res = 0u128;
    let steps = 26501365;
    //
    let mut visited = HashSet::new();
    // let mut moves: HashSet<(i32, i32)> = HashSet::new();
    // moves.insert(start);
    //
    // for i in 1..=steps {
    //     moves = moves.iter().flat_map(|(move_i, move_j)| {
    //         vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
    //             .into_iter().map(|(add_i, add_j)| (move_i + add_i, move_j + add_j))
    //             .collect::<Vec<_>>()
    //     })
    //         .filter_map(|(i, j)| plots.get(&(i.rem_euclid(height) as usize, j.rem_euclid(width) as usize)).map(|_| (i, j)))
    //         .filter(|x| !visited.contains(x))
    //         .filter(|(i, j)| !(*i < 0 && *j < 0 || *i < 0 && *j >= width || *i >= height && *j < 0 || *i >= height && *j >= width))
    //         .collect::<HashSet<_>>();
    //     for m in moves.iter() {
    //         visited.insert(*m);
    //     }
    //
    //     if i % 100 == 0{
    //         println!("iteration {}", i )
    //     }
    //     if i % 2 == steps % 2 {
    //         res += moves.len() as u128;
    //     }
    // }


    let mut cache = HashMap::new();

    let (corners, plots) = get_corners(start, (width, height), &plots);
    let mut first_quoter = (0i32, - (steps as i32) / width as i32);

    println!("corners: {:?}", corners_count(first_quoter, (width, height), steps, &corners, &plots, &mut HashSet::new(), &mut cache));

    let ((even_res, even_res_full), (odd_res, odd_res_full)) = (
        corners_count((0, 0), (width, height), steps, &corners, &plots, &mut HashSet::new(), &mut cache),
        corners_count((0, 1), (width, height), steps, &corners, &plots, &mut HashSet::new(),&mut cache)
        );

    // assert!(even_res_full && odd_res_full);


    let mut contur = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(first_quoter);



    let mut intervals = HashMap::new();
    while let Some((map_i, map_j)) = queue.pop_front() {

        if contur.contains(&(map_i, map_j)) {
            continue
        }

        // println!("processed {} {:?}", i, (map_i, map_j));

        let (cnt, is_full) = corners_count((map_i, map_j), (width, height), steps, &corners, &plots, &mut visited, &mut cache);
        if cnt == 0 {
            continue
        }

        contur.insert((map_i, map_j));

        if is_full {
            let (beg, end) = intervals.get(&map_i).map(|x|*x).unwrap_or((0, 0));
            intervals.insert(map_i, (beg.min(map_j), end.max(map_j)));

        } else {
            res += cnt as u128;
        }

        // res += cnt as u128;

        for next_map in vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .filter(|(i, j)| {
                if is_full {
                    if (map_i > 0 && *i < 0) || (map_i < 0 && *i > 0) || (map_j > 0 && *j < 0) || (map_j < 0 && *j > 0) {
                        return false;
                    }
                }
                true
            })
            .map(|(add_i, add_j)| (map_i + add_i, map_j + add_j)) {

            queue.push_back(next_map);
        }
    }

    for (i, (beg_j, end_j)) in intervals {
        let is_even = (i.abs() + beg_j.abs()) % 2 == 0;
        let maps_cnt = end_j - beg_j + 1;
        res += ((maps_cnt  as u128 / 2) * (even_res as u128 + odd_res  as u128) + (maps_cnt  as u128 % 2) * (if is_even { even_res  as u128 } else { odd_res  as u128 }) ) as u128;
    }

    for i in -50..50i32 {
        for j in -50..50i32 {
            if !plots.contains(&((i.rem_euclid(height)) as usize, (j.rem_euclid(width)) as usize)) {
                print!("#")
            } else {
                if visited.contains(&(i, j)) {
                    print!("O")
                } else {
                    print!(".")
                }

            }
        }
        println!()
    }

    println!("cnt: {}; cache: {}; res {}", contur.len(), cache.len(), res)
}

fn get_corners((start_i, start_j): (i32, i32), (width, height): (i32, i32), plots: &HashSet<(usize, usize)>) -> (Vec<(i32, i32, i32)>, HashSet<(usize, usize)>) {
    let mut contur = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_i, start_j, 0));

    while let Some((map_i, map_j, step)) = queue.pop_front() {
        if !plots.contains(&(map_i as usize, map_j as usize)) {
            continue
        }

        if contur.contains_key(&(map_i, map_j)) {
            continue
        }

        contur.insert((map_i, map_j), step);

        for next_map in vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|(add_i, add_j)| (map_i + add_i, map_j + add_j, step + 1)) {

            queue.push_back(next_map);
        }
    }

    let columns = (0..width).filter(|j| (0..height).all(|i| plots.contains(&(i as usize, *j as usize))))
        .collect::<Vec<_>>();
    let rows = (0..height).filter(|i| (0..width).all(|j| plots.contains(&(*i as usize, j as usize))))
        .collect::<Vec<_>>();

    (rows.into_iter().flat_map(|i| columns.iter().map(move |j| (i, *j)))
         .filter(|x| *x != (start_i, start_j))
        .map(|(i, j)| (i, j, *contur.get(&(i, j)).expect(format!("not found ({}, {})", i, j).as_str())))
        .collect::<Vec<_>>(), contur.keys().map(|(i, j)| (*i as usize, *j as usize)).collect::<HashSet<_>>())
}

fn corners_count(
    (map_i, map_j): (i32, i32),
    (width, height): (i32, i32),
    steps: u32,
    corners: &Vec<(i32, i32, i32)>,
    plots: &HashSet<(usize, usize)>,
    visited: &mut HashSet<(i32, i32)>,
    cache: &mut HashMap<VecDeque<(i32, i32, i32)>, (usize, usize, bool)>
) -> (usize, bool) {
    let mut queue = corners.iter()
        .map(|(i, j, _)| {
        (i + height * map_i, j + width * map_j)
    }).map(|(i, j)| {
        (i.rem_euclid(height), j.rem_euclid(width), steps as i32 - corners.iter()
            .map(|(cor_i, cor_j, add)| i.abs_diff(*cor_i) + j.abs_diff(*cor_j) + *add as u32)
            .min().unwrap() as i32)
    }).collect::<VecDeque<_>>();

    if cache.contains_key(&queue) {
        let (even, odd, is_full) = *cache.get(&queue).unwrap();
        return (even, is_full);
    }

    let key = queue.clone();
    let mut contur = HashMap::new();
    while let Some((i, j, step)) = queue.pop_front() {
        if !plots.contains(&((i) as usize, (j) as usize)) {
            continue
        }

        if step < 0 {
            continue
        }

        if contur.contains_key(&(i, j)) && *contur.get(&(i, j)).unwrap() >= step {
            continue
        }

        contur.insert((i, j), step);

        for next_map in vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|(add_i, add_j)| (i + add_i, j + add_j, step - 1)) {

            queue.push_back(next_map);
        }
    }

    // for ((i, j), _) in contur.iter() {
    //     visited.insert((i + height * map_i, j + width * map_j));
    // }

    let (even, odd) = (contur.values().filter(|x| *x % 2 == 0).count(), contur.values().filter(|x| *x % 2 == 1).count());
    cache.insert(key, (even, odd, contur.len() == plots.len()));
    (even, contur.len() == plots.len())
}
