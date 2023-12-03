use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

fn main() {
    let mut lines = BufReader::new(File::open("day3.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.expect("do it"));

    let (mut prev, mut cur, mut next) = (None::<String>, lines.next(), lines.next());
    let filler = std::iter::repeat('.').take(cur.as_ref().unwrap().len()).collect::<String>();

    let mut result = 0;

    loop {
        if let Some(cur_val) = cur.as_ref() {
            let mut zipped =
                prev.as_ref().unwrap_or(&filler).bytes()
                    .zip(cur_val.bytes())
                    .zip(next.as_ref().unwrap_or(&filler).bytes());

            let mut number = None::<u32>;
            let mut has_symbol = false;

            for ((p, c), n) in zipped {
                let mut has_symbol_cur = match (p, n) {
                    (b'.' | b'0'..=b'9', b'.' | b'0'..=b'9') => false,
                    (_, _) => true
                };
                has_symbol_cur = match c {
                    b'.' | b'0'..=b'9' => has_symbol_cur,
                    _ => true
                };
                has_symbol |= has_symbol_cur;
                match c {
                    b'0'..=b'9' => {
                        let digit = u32::from(c - b'0');
                        number =
                            number.map(|x| x * 10 + digit).or(Some(digit));
                    },
                    _ => {
                        if let Some(number) = number {
                            if has_symbol {
                                result += number;
                            }
                        }
                        has_symbol = has_symbol_cur;
                        number = None;
                    },
                }
            }

            if let Some(number) = number {
                if has_symbol {
                    result += number;
                }
            }
            prev = cur;
            cur = next;
            next = lines.next();
        } else {
            break;
        }
        println!("{}", result)
    }

}
