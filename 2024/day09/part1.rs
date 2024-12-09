use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() {
    let mut memory = BufReader::new(File::open("day09.txt").expect("what could be happen?"))
        .bytes()
        .map(|b| (b.unwrap() - b'0') as i32)
        .enumerate()
        .collect::<VecDeque<_>>();

    let mut checksum = 0u64;
    let mut len = 0u64;
    while memory.len() > 0 {
        let (idx, size) = memory.pop_front().unwrap();
        let (size, id) = if idx.rem_euclid(2) == 0 {
            (size, idx / 2)
        } else {
            match memory.pop_back() {
                Some((idx_last, size_last)) => {
                    let (idx_last, size_last) = if idx_last.rem_euclid(2) == 1 {
                        match memory.pop_back() {
                            Some((idx_last, size_last)) => (idx_last, size_last),
                            None => (0, 0),
                        }
                    } else {
                        (idx_last, size_last)
                    };
                    if size_last > size {
                        memory.push_back((idx_last, size_last - size));
                    }
                    if size > size_last {
                        memory.push_front((idx, size - size_last));
                    }
                    (size.min(size_last), idx_last / 2)
                }
                None => (0, 0),
            }
        };

        println!("len = {}, size = {}, id = {}", len, size, id);
        checksum += ((size as u64 * len) + ((size - 1) * size / 2) as u64) * id as u64;
        len += size as u64;
    }

    println!("{}", checksum);
}
