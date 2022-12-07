use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::{Peekable, TakeWhile};
use std::ops::IndexMut;
use crate::Cmd::Ls;

enum Cmd {
    Cd(String), Ls
}

fn read_to_eol(it: &mut impl Iterator<Item = u8>) {
    loop {
        match it.next() {
            Some(b'\n') | None => break,
            _ => continue
        }
    }
}

fn read_cmd(it: &mut impl Iterator<Item = u8>) -> Option<Cmd> {
    it.next().and_then(|b| {
        assert_eq!(b'$', b);
        assert_eq!(Some(b' '), it.next());

        match it.next() {
            Some(b'c') => {
                assert_eq!(Some(b'd'), it.next());
                assert_eq!(Some(b' '), it.next());

                let mut dir = String::new();
                loop {
                    match it.next() {
                        Some(b'\r') => continue,
                        Some(b'\n') | None => break,
                        Some(b) => dir.push(char::from(b))
                    }
                }
                Some(Cmd::Cd(dir))
            }
            Some(b'l') => {
                read_to_eol(it);
                Some(Cmd::Ls)
            },
            _ => panic!("unexpected format!")
        }
    })
}

enum LsOut {
    File(i32), Dir
}

fn read_int(it: &mut impl Iterator<Item = u8>) -> i32
{
    let mut res: i32 = 0;
    for b in it {
        match b {
            b'0'..=b'9' => {
                res = res * 10 + i32::from(b - b'0')
            }
            _ => break
        }
    }
    return res;
}

fn read_ls_out(it: &mut Peekable<impl Iterator<Item = u8>>) -> Option<LsOut> {
    it.peek().copied().and_then(|b| {
        match b {
            b'0'..=b'9' => {
                let size = read_int(it);
                read_to_eol(it);
                Some(LsOut::File(size))
            },
            b'$' => {
                None
            },
            _ => {
                read_to_eol(it);
                Some(LsOut::Dir)
            }
        }
    })
}

fn main() {
    let mut it = BufReader::new(File::open("day7.txt").expect("I know you are existed"))
        .bytes()
        .map(|x| x.expect("You can do it!"))
        .peekable();

    let mut st = VecDeque::new();
    let mut res = 0;
    loop {
        match read_cmd(&mut it) {
            Some(cmd) => {
                match cmd {
                    Cmd::Cd(s) => {
                        if s.eq("..") {
                            let size = st.pop_back().expect("valid input");
                            if size <= 100000 {
                                res += size;
                            }
                            *st.back_mut().expect("valid input") += size;
                        } else {
                            st.push_back(0);
                        }
                    },
                    Cmd::Ls => {
                        loop {
                            match read_ls_out(&mut it) {
                                Some(lsOut) => {
                                    match lsOut {
                                        LsOut::File(size) => {
                                            *st.back_mut().expect("valid input") += size;
                                        }
                                        LsOut::Dir => continue
                                    }
                                }
                                None => break
                            }
                        }
                    }
                }
            },
            None => {
                loop {
                    let mut prev = 0;
                    match st.pop_back() {
                        Some(size) => {
                            prev += size;
                            if prev <= 100000 {
                                res += prev
                            }
                        },
                        None => break
                    }
                }
                break;
            }
        }
    }
    println!("{}", res);

}
