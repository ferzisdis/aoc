use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (res, _) = BufReader::new(File::open("day07.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .fold((0, HashSet::new()), |(mut count, mut set), line| {
            for (i, ch) in line.bytes().enumerate() {
                match ch {
                    b'S' => {
                        set.insert(i);
                    }
                    b'^' => {
                        if set.remove(&i) {
                            count += 1;
                            set.insert(i + 1);
                            set.insert(i - 1);
                        }
                    }
                    _ => {}
                }
            }
            (count, set)
        });

    println!("{}", res);
}
