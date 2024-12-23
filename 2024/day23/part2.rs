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
    let mut results = Vec::new();
    for (k, vs) in &graph {
        let mut st = Vec::new();
        let mut group = Vec::new();
        group.push(k);
        st.push(vs.iter());

        while let Some(mut it) = st.pop() {
            if let Some(v_inner) = it.next() {
                if !visited.contains(v_inner)
                    && group.iter().all(|v| {
                        graph
                            .get(*v)
                            .map(|hs| hs.contains(v_inner))
                            .unwrap_or(false)
                    })
                {
                    group.push(v_inner);
                    st.push(it.clone());
                }
                st.push(it);
            } else {
                if group.len() > 3 {
                    results.push(group.clone());
                }
                group.pop();
            }
        }
        visited.insert(k);
    }

    println!(
        "{}",
        results
            .into_iter()
            .max_by_key(|v| v.len())
            .map(|v| {
                let mut res = v.into_iter().map(|x| x.clone()).collect::<Vec<_>>();
                res.sort();
                res.join(",")
            })
            .unwrap()
    );
}
