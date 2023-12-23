use std::arch::aarch64::poly8x8_t;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::os::macos::raw::stat;
use union_find_rs::prelude::*;

fn main() {
    let mut map = BufReader::new(File::open("day23.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            x.bytes().collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let (start, end) = ((0, 1), (map.len() - 1, map.get(0).unwrap().len() - 2));
    let mut graph = make_weighted_graph(map, start);


    // let brackpoints = find_breakpoints(graph, start, end);
    // println!("points {}", brackpoints.len());


    let mut cache = HashMap::new();
    println!("{}", dfs_by_nodes(&mut graph, start, end, &mut cache).unwrap());
}


fn dfs_by_nodes(
    graph: &mut HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    point: (usize, usize), end: (usize, usize),
    cache: &mut HashMap<(Vec<(usize, usize)>, (usize, usize)), Option<usize>>) -> Option<usize> {
    if !is_connected_to_end(graph, point, end) {
        return None;
    }

    let mut key_vec = graph.keys().map(|x| *x).collect::<Vec<_>>();
    key_vec.sort();
    let key = (key_vec, point);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    println!("{}", graph.len());

    let nbs = graph.remove(&point).unwrap();
    for (nb_k, nb_v) in nbs.iter() {
        graph.get_mut(nb_k).unwrap().remove(&point);
    }

    let mut res: Option<usize> = Some(0);
    for (nb, nb_v) in nbs.iter() {
        if let Some(m) = dfs_by_nodes(graph, *nb, end, cache) {
            res = res.map(|x| x.max(m + nb_v)).or(Some(m + nb_v))
        }
    }

    for (nb_k, nb_v) in nbs.iter() {
        graph.get_mut(nb_k).unwrap().insert(point, *nb_v);
    }
    graph.insert(point, nbs);
    cache.insert(key, res);
    return res;
}

fn is_connected_to_end(graph: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>, cur: (usize, usize), end: (usize, usize)) -> bool {
    let mut sets: DisjointSets<(usize, usize)> = DisjointSets::new();
    for point in graph.keys() {
        sets.make_set(*point).unwrap();
    }
    for (k, hs) in graph.iter() {
        for (v, _) in hs {
            if sets.find_set(k).unwrap() == sets.find_set(v).unwrap() {
                continue
            }

            sets.union(k, v).unwrap();
        }
    }

   sets.find_set(&cur).unwrap() == sets.find_set(&end).unwrap()
}

fn find_breakpoints(graph: HashMap<(usize, usize), HashSet<(usize, usize)>>, begin: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();


    for point in graph.keys() {
        if *point == begin || *point == end {
            continue
        }

        let mut sets: DisjointSets<(usize, usize)> = DisjointSets::new();
        for point in graph.keys() {
            sets.make_set(*point).unwrap();
        }

        for (k, hs) in graph.iter() {
            for v in hs {
                if sets.find_set(k).unwrap() == sets.find_set(v).unwrap() || k == point || v == point {
                    continue
                }

                sets.union(k, v).unwrap();
            }
        }

        if sets.find_set(&begin).unwrap() != sets.find_set(&end).unwrap() {
            res.push(*point)
        }
    }
    res
}



fn make_weighted_graph(mut map: Vec<Vec<u8>>, start_point: (usize, usize)) -> HashMap<(usize, usize), HashMap<(usize, usize), usize>> {
    let mut res = HashMap::new();
    res.insert(start_point, HashMap::new());
    dfs_graph(&mut map, start_point, (start_point.0 as i32, start_point.1 as i32), 0, &mut res);
    res
}

fn dfs_graph(map: &mut Vec<Vec<u8>>, mut connect_to: (usize, usize), (pos_i, pos_j): (i32, i32), mut steps: usize, res: &mut HashMap<(usize, usize), HashMap<(usize, usize), usize>>) {
    let current = (pos_i as usize, pos_j as usize);
    match map.get(pos_i as usize).and_then(|row| row.get(pos_j as usize)).clone() {
        None | Some(b'#') => (),
        Some(b'O') => {
            if connect_to == current {
                return;
            }
            if res.contains_key(&current) {
                res.get_mut(&current).unwrap().insert(connect_to, steps);
                res.get_mut(&connect_to).unwrap().insert(current, steps);
            }
        },
        Some(b) => {
            let moves = vec![(1,0), (-1,0), (0,1), (0,-1)].into_iter()
                .filter(|(add_i, add_j)| {
                    match map.get((pos_i + add_i) as usize).and_then(|row| row.get((pos_j + add_j) as usize)) {
                        None | Some(b'#') => false,
                        _ => true,
                    }
                }).collect::<Vec<_>>();
            *map.get_mut(pos_i as usize).unwrap().get_mut(pos_j as usize).unwrap() = b'O';

            if moves.len() > 2 || pos_i as usize == map.len() - 1 {
                res.insert(current, HashMap::new());
                res.get_mut(&current).unwrap().insert(connect_to, steps);
                res.get_mut(&connect_to).unwrap().insert(current, steps);
                connect_to = current;
                steps = 0
            }

            if pos_i as usize == map.len() - 1 {
                return
            }
            for (add_i, add_j) in moves {
                dfs_graph(map, connect_to, (pos_i + add_i, pos_j + add_j), steps + 1, res);
            }
        }
    }
}

fn dfs(map: &mut Vec<Vec<u8>>, (pos_i, pos_j): (i32, i32)) -> Option<usize> {

    match map.get(pos_i as usize).and_then(|row| row.get(pos_j as usize)).map(|x| *x) {
        None | Some(b'#') | Some(b'O') => None,
        Some(b) => {
            let moves = vec![(1,0), (-1,0), (0,1), (0,-1)];
            if pos_i as usize == map.len() - 1 {
                return Some(1);
            }

            let mut res: Option<usize> = None;
            *map.get_mut(pos_i as usize).unwrap().get_mut(pos_j as usize).unwrap() = b'O';
            for (add_i, add_j) in moves {
                if let Some(m) = dfs(map, (pos_i + add_i, pos_j + add_j)) {
                    res = res.map(|x| x.max(m + 1)).or(Some(m + 1))
                }
            }
            *map.get_mut(pos_i as usize).unwrap().get_mut(pos_j as usize).unwrap() = b;
            res
        }
    }
}
