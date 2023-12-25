use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::id;
use rand::prelude::*;


fn main() {
    let graph = BufReader::new(File::open("day25.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            let (lhs, rhs) = x.split_once(": ").unwrap();
            (lhs.to_string(), rhs.split_ascii_whitespace().map(|x| x.to_string()).collect::<Vec<_>>())
        }).fold(HashMap::new(), |mut acc, (node, nbs)| {
        if !acc.contains_key(&node) {
            acc.insert(node.clone(), Vec::new());
        }
        for nb in nbs {
            if !acc.contains_key(&nb) {
                acc.insert(nb.clone(), Vec::new());
            }
            acc.get_mut(&node).unwrap().push(nb.clone());
            acc.get_mut(&nb).unwrap().push(node.clone());
        }

        acc
    });

    let mut rng = rand::thread_rng();
    loop {
        let res = try_yolo(graph.clone(), &mut rng);
        if res == 3 {
            break;
        }
    }
}

fn try_yolo(mut graph: HashMap<String, Vec<String>>, rnd: &mut ThreadRng) -> usize {
    let mut keys = graph.keys().map(|x| x.clone()).collect::<Vec<_>>();


    while (graph.len() > 2) {
        keys.shuffle(rnd);
        if let Some(first_node) = keys.pop() {
            if !graph.contains_key(&first_node) {
                continue;
            }

            let mut nbs = graph.get(&first_node).unwrap().iter().map(|x| x.clone()).collect::<Vec<_>>();
            nbs.shuffle(rnd);
            let second_node = nbs.pop().unwrap();
            for node in graph.get(&second_node).unwrap() {
                nbs.push(node.clone());
            }
            graph.remove(&first_node);
            graph.remove(&second_node);
            let new_node = format!("{}{}", first_node, second_node);
            graph.insert(new_node.clone(), Vec::new());
            for nb in nbs {
                if nb == first_node || nb == second_node {
                    continue
                }

                let nb_collection = graph.get_mut(&nb).unwrap();
                *nb_collection = nb_collection.iter().map(|x| x.clone()).filter(|x| x != &first_node && x != &second_node)
                    .collect::<Vec<_>>();
                nb_collection.push(new_node.clone());
                graph.get_mut(&new_node).unwrap().push(nb);
            }
            keys.push(new_node.clone());
        }
    }

    let mut it = graph.into_iter();
    let (first_node, second_node) = (it.next().unwrap(), it.next().unwrap());
    println!("first node cnt: {}, second_node_cnt {}", first_node.1.len(), second_node.1.len());
    println!("res: {}", (first_node.0.len() / 3) * (second_node.0.len() / 3));

    return first_node.1.len()
}
