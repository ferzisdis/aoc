use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() {
    let (state, mut stack) =
        BufReader::new(File::open("day03.txt").expect("what could be happen?"))
            .bytes()
            .map(|b| b.unwrap())
            .fold((0, Vec::new()), |(state, stack), byte| {
                next_state(state, stack, byte)
            });

    println!("items: {}", stack.len());

    if state >= 4 {
        stack.pop();
    }
    if state >= 5 {
        stack.pop();
    }

    println!(
        "{}",
        stack
            .chunks(2)
            .map(|chunk| chunk[0] * chunk[1])
            .sum::<u64>()
    );
}

fn next_state(state: u32, mut stack: Vec<u64>, byte: u8) -> (u32, Vec<u64>) {
    match (state, byte) {
        (0, b'm') => (1, stack),
        (1, b'u') => (2, stack),
        (2, b'l') => (3, stack),
        (3, b'(') | (4, b',') => {
            stack.push(0);
            (state + 1, stack)
        }
        (4, b'0'..=b'9') | (5, b'0'..=b'9') => {
            let last = stack.pop().expect("state machine is broken");
            stack.push(last * 10 + (byte - b'0') as u64);
            (state, stack)
        }
        (5, b')') => (0, stack),
        _ => {
            if state >= 4 {
                stack.pop();
            }
            if state >= 5 {
                stack.pop();
            }
            (0, stack)
        }
    }
}
