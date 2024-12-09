use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};

struct Block {
    offset: u64,
    size: u8,
    idx: usize,
}

fn main() {
    let (_, blocks) = BufReader::new(File::open("day09.txt").expect("what could be happen?"))
        .bytes()
        .map(|b| b.unwrap() - b'0')
        .enumerate()
        .fold((0u64, Vec::new()), |(offset, mut res), (idx, size)| {
            res.push(Block { offset, size, idx });
            (offset + size as u64, res)
        });

    let mut free_memory = HashMap::new();
    let mut files = Vec::new();

    for size in 1u8..=9u8 {
        free_memory.insert(size, BinaryHeap::new());
    }

    for block in blocks {
        if block.idx.rem_euclid(2) == 0 {
            files.push(block);
        } else {
            if block.size == 0 {
                continue;
            }
            free_memory
                .get_mut(&block.size)
                .unwrap()
                .push(Reverse(block.offset));
        }
    }

    files.reverse();

    let checksum = files
        .into_iter()
        .map(|file| {
            let nearest_size = (file.size..=9)
                .filter_map(|size| free_memory.get(&size).map(|heap| (size, heap)))
                .filter_map(|(size, heap)| heap.peek().map(|offset| (size, offset.0.clone())))
                .filter(|(_, offset)| offset < &file.offset)
                .min_by_key(|(_, offset)| *offset)
                .map(|(size, _)| size);

            if let Some(nearest_size) = nearest_size {
                let nearest_offset = free_memory.get_mut(&nearest_size).unwrap().pop().unwrap().0;
                if file.size < nearest_size {
                    free_memory
                        .get_mut(&(nearest_size - file.size))
                        .unwrap()
                        .push(Reverse(nearest_offset + file.size as u64));
                }
                calc_part_checksum(nearest_offset, file.size, file.idx / 2)
            } else {
                calc_part_checksum(file.offset, file.size, file.idx / 2)
            }
        })
        .sum::<u64>();

    println!("{}", checksum);
}

fn calc_part_checksum(offset: u64, size: u8, id: usize) -> u64 {
    ((size as u64 * offset as u64) + ((size - 1) * size / 2) as u64) * id as u64
}
