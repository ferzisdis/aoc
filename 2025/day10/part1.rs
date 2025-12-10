use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn next_permutation(st: &mut VecDeque<usize>, max_len: usize) -> bool {
    if st.len() == 0 {
        st.push_back(0);
        return true;
    }

    let back = st.pop_back().unwrap();
    if back + 1 < max_len {
        st.push_back(back + 1);
        return true;
    }

    next_permutation(st, max_len - 1);
    assert!(*st.back().unwrap() < max_len + 1);
    st.push_back(st.back().unwrap() + 1);
    st.len() < max_len
}

fn main() {
    let res = BufReader::new(File::open("day10.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let mut it = x.split(' ');
            let signal = it.next().unwrap();
            let mut buttons = Vec::new();

            while let Some(part) = it.next() {
                if part.starts_with('(') {
                    buttons.push(part);
                } else {
                    break;
                }
            }

            (
                signal[1..signal.len() - 1]
                    .bytes()
                    .rev()
                    .fold(0u32, |acc, b| {
                        if b == b'#' {
                            acc << 1 | 1
                        } else {
                            acc << 1 | 0
                        }
                    }),
                buttons
                    .into_iter()
                    .map(|x| {
                        x[1..x.len() - 1]
                            .split(',')
                            .map(|x| x.parse::<u32>().unwrap())
                            .fold(0u32, |acc, x| acc | (1 << x))
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(signal, buttons)| {
            let mut st = VecDeque::new();
            while next_permutation(&mut st, buttons.len()) {
                if signal == st.iter().fold(0u32, |acc, &x| acc ^ buttons[x]) {
                    break;
                }
            }
            st.len()
        })
        .sum::<usize>();

    println!("{}", res)
}
