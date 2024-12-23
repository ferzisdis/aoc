use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut it = BufReader::new(File::open("day19.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap());

    let binding = it.next().unwrap();
    let (parts, max_size) =
        binding
            .split(", ")
            .fold((HashSet::new(), 0), |(mut hs, max_size), val| {
                hs.insert(val);
                (hs, max_size.max(val.len()))
            });
    it.next();

    let mut cache = HashMap::new();
    let res = it
        .map(|line| {
            println!("{}", line);
            is_valid(&mut cache, &parts, max_size, line.as_str())
        })
        .sum::<usize>();

    println!("{}", res);
    println!("Finish!");
}

fn is_valid<'a>(
    cache: &mut HashMap<String, usize>,
    parts: &HashSet<&str>,
    max_size: usize,
    line: &str,
) -> usize {
    if line.len() == 0 {
        return 1;
    }
    if cache.contains_key(line) {
        return cache[line];
    }
    let mut variants = 0;
    for size in 0..=max_size {
        if line.len() < size {
            break;
        }
        let (left, right) = (&line[0..size], &line[size..]);
        if !parts.contains(left) {
            continue;
        }
        variants += is_valid(cache, parts, max_size, right);
    }
    cache.insert(line.to_string(), variants);
    variants
}
