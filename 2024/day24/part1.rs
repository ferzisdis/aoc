use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day24.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap());

    let mut values = HashMap::new();
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        let (name, val) = line
            .split_once(": ")
            .map(|(l, r)| (l.to_string(), r.parse::<u8>().unwrap()))
            .unwrap();
        values.insert(name, val);
    }

    let mut nodes = Vec::new();
    let mut inbound_links = HashMap::new();
    let mut outbound_links = HashMap::new();
    while let Some(line) = it.next() {
        let mut it = line.split(" ");
        let l = it.next().unwrap().to_string();
        let op = it.next().unwrap().to_string();
        let r = it.next().unwrap().to_string();
        _ = it.next().unwrap();
        let t = it.next().unwrap().to_string();
        nodes.push((l.clone(), op, r.clone(), t.clone()));
        inbound_links
            .entry(t)
            .or_insert(HashSet::new())
            .insert(nodes[nodes.len() - 1].clone());
        outbound_links
            .entry(l)
            .or_insert(HashSet::new())
            .insert(nodes[nodes.len() - 1].clone());
        outbound_links
            .entry(r)
            .or_insert(HashSet::new())
            .insert(nodes[nodes.len() - 1].clone());
    }

    let mut ordered_nodes = Vec::new();
    let mut queue = nodes
        .iter()
        .filter(|(l, _, r, _)| {
            inbound_links.get(l).map(|x| x.len()).unwrap_or(0) == 0
                && inbound_links.get(r).map(|x| x.len()).unwrap_or(0) == 0
        })
        .collect::<VecDeque<_>>();

    while let Some(node) = queue.pop_front() {
        println!("in order {:?}", node);
        ordered_nodes.push(node);
        if inbound_links
            .entry(node.3.clone())
            .and_modify(|x| {
                x.remove(node);
            })
            .or_default()
            .len()
            == 0
        {
            if outbound_links.contains_key(&node.3) {
                for next in &outbound_links[&node.3] {
                    if inbound_links.get(&next.0).map(|x| x.len()).unwrap_or(0) == 0
                        && inbound_links.get(&next.2).map(|x| x.len()).unwrap_or(0) == 0
                    {
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    for (l, op, r, t) in ordered_nodes {
        println!("processed: {:?}", (l, op, r, t));
        let res = match op.as_str() {
            "AND" => values[l] & values[r],
            "OR" => values[l] | values[r],
            "XOR" => values[l] ^ values[r],
            _ => unreachable!(),
        };
        values.insert(t.to_string(), res);
    }

    let mut res = values
        .keys()
        .filter(|x| x.starts_with("z"))
        .collect::<Vec<_>>();
    res.sort_by_key(|x| x[1..].parse::<u32>().unwrap());
    res.reverse();
    let mut total = 0u64;
    for b in res {
        total = (total << 1) + values[b] as u64;
        println!("{}: {}", b, values[b])
    }
    println!("{}", total);
}
