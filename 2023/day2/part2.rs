use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

fn read_number<T>(it: &mut T) -> u32
    where
        T: Iterator<Item=u8>,
{
    let mut res: u32 = 0;
    for b in it {
        match b {
            b'0'..=b'9' => res = res * 10 + u32::from(b - b'0'),
            _ => break,
        }
    }

    return res;
}

fn read_word<T>(it: &mut T) -> (String, Option<u8>)
    where
        T: Iterator<Item=u8>,
{
    let mut res = String::new();
    for b in it {
        match b {
            b' ' | b',' | b';' | b':' => return (res, Option::from(b)),
            _ => res.push(char::from(b)),
        }
    }

    return (res, Option::None);
}

fn read_subset<T>(it: &mut T) -> Option<(u32, u32, u32)>
    where
        T: Iterator<Item=u8>,
{
    if it.next().is_none() {
        // read space
        return None;
    }

    let (mut red, mut green, mut blue) = (0, 0, 0);
    loop {
        let num = read_number(it);
        let (word, sep) = read_word(it);
        match word.as_str() {
            "red" => red += num,
            "green" => green += num,
            "blue" => blue += num,
            _ => panic!("Unexpected token [{}]", word),
        }

        match sep {
            Some(b';') | None => break,
            _ => { it.next(); } // read space
        }
    }

    return Some((red, green, blue));
}

fn main() {
    println!(
        "{}",
        BufReader::new(File::open("day2.txt").expect("what could be happen?"))
            .lines()
            .map(|x| x.expect("do it"))
            .map(|x| {
                let mut it = x.into_bytes().into_iter();
                read_word(&mut it); // Skip 'Game '
                read_number(&mut it);
                let (mut red, mut green, mut blue) = (0, 0, 0);

                loop {
                    match read_subset(&mut it) {
                        None => break,
                        Some((r, g, b)) => {
                            red = u32::max(r, red);
                            green = u32::max(g, green);
                            blue = u32::max(b, blue);
                        }
                    }
                }

                red * green * blue
            })
            .sum::<u32>()
    );
}
