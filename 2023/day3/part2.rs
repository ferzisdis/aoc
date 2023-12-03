use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

fn main() {
    let mut lines = BufReader::new(File::open("day3.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.expect("do it"));

    let (mut prev, mut cur, mut next) = (None::<String>, lines.next(), lines.next());
    let filler = std::iter::repeat('.').take(cur.as_ref().unwrap().len()).collect::<String>();
    let mut gears = HashMap::new();

    for i in 1.. {
        if let Some(cur_val) = cur.as_ref() {
            let mut zipped =
                prev.as_ref().unwrap_or(&filler).bytes()
                    .zip(cur_val.bytes())
                    .zip(next.as_ref().unwrap_or(&filler).bytes());

            let mut number = None::<u32>;
            let mut symbols = Vec::new();

            for (j, ((p, c), n)) in zipped.enumerate() {
                let mut symbols_cur = Vec::new();
                if p == b'*' {
                    symbols_cur.push((i, j))
                }
                if c == b'*' {
                    symbols_cur.push((i+1, j))
                }
                if n == b'*' {
                    symbols_cur.push((i+2, j))
                }

                symbols.append(&mut symbols_cur.clone());
                match c {
                    b'0'..=b'9' => {
                        let digit = u32::from(c - b'0');
                        number =
                            number.map(|x| x * 10 + digit).or(Some(digit));
                    },
                    _ => {
                        if let Some(number) = number {
                            for s in symbols {
                                gears.entry(s).or_insert(vec![])
                                    .push(number);
                            }
                        }
                        symbols = symbols_cur;
                        number = None;
                    },
                }
            }

            if let Some(number) = number {
                for s in symbols {
                    gears.entry(s).or_insert(vec![])
                        .push(number);
                }
            }
            prev = cur;
            cur = next;
            next = lines.next();
        } else {
            break;
        }
    }

    let result = gears.iter().map(|(_, v)| {
        if v.len() == 2 {
            v.iter().fold(1u32, |acc, x| acc * x)
        } else {
            0
        }
    }).sum::<u32>();

    println!("{}", result)
}
