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

        graph
            .entry(r.clone())
            .or_insert(HashSet::new())
            .insert(l.clone());
    }

    let result = it
        .map(|line| {
            let order = line.split(',').map(|x| x.parse::<u32>().unwrap()).try_fold(
                (Vec::new(), true),
                |(prevs, correct), cur| {
                    let mut next = Vec::new();
                    let mut inserted = false;

                    for item in prevs {
                        if graph
                            .get(&item)
                            .map(|hs| hs.contains(&cur))
                            .unwrap_or(false)
                            && !inserted
                        {
                            inserted = true;
                            next.push(cur);
                        }
                        next.push(item);
                    }
                    if !inserted {
                        next.push(cur);
                    }

                    Some((next, correct && !inserted))
                },
            );
            match order {
                Some((order, false)) => order[order.len() / 2],
                _ => 0u32,
            }
        })
        .sum::<u32>();

    println!("{}", result);
}
