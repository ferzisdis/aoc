use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let (_, stack, _) = BufReader::new(File::open("day03.txt").expect("what could be happen?"))
        .bytes()
        .map(|b| b.unwrap())
        .into_iter()
        .chain([b'.'])
        .fold((0, Vec::new(), true), |(state, stack, enabled), byte| {
            next_state(state, stack, enabled, byte)
        });

    println!("items: {}", stack.len());

    println!(
        "{}",
        stack
            .chunks(2)
            .map(|chunk| chunk[0] * chunk[1])
            .sum::<u64>()
    );
}

fn next_state(state: u32, mut stack: Vec<u64>, enabled: bool, byte: u8) -> (u32, Vec<u64>, bool) {
    match (state, byte, enabled) {
        (0, b'm', true) => (1, stack, enabled),
        (1, b'u', true) => (2, stack, enabled),
        (2, b'l', true) => (3, stack, enabled),
        (3, b'(', true) | (4, b',', true) => {
            stack.push(0);
            (state + 1, stack, enabled)
        }
        (4, b'0'..=b'9', true) | (5, b'0'..=b'9', true) => {
            let last = stack.pop().expect("state machine is broken");
            stack.push(last * 10 + (byte - b'0') as u64);
            (state, stack, enabled)
        }
        (5, b')', true) => (0, stack, enabled),

        (0, b'd', _) => (10, stack, enabled),
        (10, b'o', _) => (11, stack, enabled),
        (11, b'(', _) => (12, stack, enabled),
        (12, b')', _) => (0, stack, true),
        (11, b'n', _) => (22, stack, enabled),
        (22, b'\'', _) => (23, stack, enabled),
        (23, b't', _) => (24, stack, enabled),
        (24, b'(', _) => (25, stack, enabled),
        (25, b')', _) => (0, stack, false),
        _ => {
            if state == 4 {
                stack.pop();
            }
            if state == 5 {
                stack.pop();
                stack.pop();
            }
            if byte == b'm' {
                (1, stack, enabled)
            } else if byte == b'd' {
                (10, stack, enabled)
            } else {
                (0, stack, enabled)
            }
        }
    }
}
