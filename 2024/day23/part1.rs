use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day23.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.split_once("-")
                .map(|(l, r)| (l.to_string(), r.to_string()))
                .unwrap()
        });

    let mut graph = HashMap::new();
    for (l, r) in it {
        graph
            .entry(l.clone())
            .or_insert(HashSet::new())
            .insert(r.clone());
        graph.entry(r).or_insert(HashSet::new()).insert(l);
    }

    let mut visited = HashSet::new();
    let mut triplets = Vec::new();
    for (k, vs) in &graph {
        let mut it = vs.iter();
        while let Some(v) = it.next() {
            if visited.contains(v) {
                continue;
            }
            let mut it_inner = it.clone();
            while let Some(v_inner) = it_inner.next() {
                if visited.contains(v_inner) {
                    continue;
                }
                if graph.get(v).map(|hs| hs.contains(v_inner)).unwrap_or(false) {
                    triplets.push((k, v, v_inner));
                }
            }
            visited.insert(k);
        }
    }

    println!(
        "{}",
        triplets
            .into_iter()
            .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
            .count()
    );
}
