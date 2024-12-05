use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day05.txt").expect("what could be happen?"))
        .lines()
        .map(|b| b.unwrap());

    let mut graph = HashMap::new();
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        let (l, r) = line
            .split_once('|')
            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
            .unwrap();

        graph.entry(r.clone()).or_insert(Vec::new()).push(l.clone());
    }

    let result = it
        .map(|line| {
            let order = line.split(',').map(|x| x.parse::<u32>().unwrap()).try_fold(
                Vec::new(),
                |mut prevs, cur| {
                    if contains(&graph, &prevs, cur) {
                        None
                    } else {
                        prevs.push(cur);
                        Some(prevs)
                    }
                },
            );
            match order {
                Some(order) => order[order.len() / 2],
                None => 0u32,
            }
        })
        .sum::<u32>();

    println!("{}", result);
}

fn contains(graph: &HashMap<u32, Vec<u32>>, inits: &Vec<u32>, expect: u32) -> bool {
    let mut queue = VecDeque::from(inits.clone());
    let mut visited = HashSet::new();

    while let Some(cur) = queue.pop_back() {
        if cur == expect {
            return true;
        }

        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);
        if let Some(next) = graph.get(&cur) {
            for item in next {
                if item == &expect {
                    return true;
                }
            }
        }
    }

    false
}
