use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    u32, vec,
};

fn main() {
    let mut it = BufReader::new(File::open("day11.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let (key, value) = x.split_once(": ").unwrap();
            (
                key.to_string(),
                value
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            )
        });

    let mut outs = HashMap::new();
    let mut ins = HashMap::new();
    let mut res = HashMap::new();
    res.insert("you".to_string(), 1);

    while let Some((key, links)) = it.next() {
        outs.insert(key.clone(), links.clone());
        for link in links {
            ins.entry(link).and_modify(|x| *x += 1).or_insert(1);
        }
        ins.entry(key).or_insert(0);
    }

    while !ins.is_empty() {
        println!("ins {:?}", ins);
        for (key, _) in ins
            .extract_if(|_, v| *v == 0)
            .collect::<Vec<_>>()
            .into_iter()
        {
            for link in outs.get(&key).unwrap_or(&vec![]) {
                if let Some(count) = ins.get_mut(link) {
                    *count -= 1;
                }

                if let Some(val) = res.get(&key).map(|x| *x) {
                    res.entry(link.clone())
                        .and_modify(|x| *x += val)
                        .or_insert(val);
                }
            }
        }
    }

    println!("{}", res["out"]);
}
