use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let numpad = Vec::from([
        [b'7', b'8', b'9'],
        [b'4', b'5', b'6'],
        [b'1', b'2', b'3'],
        [b' ', b'0', b'A'],
    ]);
    let numpad_idx = to_hashmap(&numpad);
    let arraypad = Vec::from([[b' ', b'^', b'A'], [b'<', b'v', b'>']]);
    let arraypad_idx = to_hashmap(&arraypad);

    // <A^A^^>AvvvA
    //
    // ^A<<^^A
    // ^A^^<<A >>AvvvA
    //
    // <A>A v<<AA>^AA>A vAA
    // <A>A <AAv<AA>>^A vAA^Av<AAA^>A
    //
    // <v<A >>^A vA ^A <vA <A A >>^A A vA <^A >A A vA ^A     <vA >^A A <A >A <v<A >A >^A A A vA <^A >A
    // v<<A >>^A vA ^A v<<A >>^A A v<A <A >>^A A vA A ^<A >A v<A ^>A A <A >A v<A <A >>^A A A <A v>A ^A

    let res = BufReader::new(File::open("day21.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            println!("{}", x);
            let num = x.as_str()[..x.len() - 1].parse::<u32>().unwrap();
            let (_, first) =
                x.into_bytes()
                    .into_iter()
                    .fold((b'A', Vec::new()), |(prev, mut acc), next| {
                        type_on_pad(&numpad, &numpad_idx, prev, next, &mut acc);
                        (next, acc)
                    });
            println!("{}", String::from_utf8(first.clone()).unwrap());
            let (_, second) =
                first
                    .into_iter()
                    .fold((b'A', Vec::new()), |(prev, mut acc), next| {
                        type_on_pad(&arraypad, &arraypad_idx, prev, next, &mut acc);
                        (next, acc)
                    });
            println!("{}", String::from_utf8(second.clone()).unwrap());
            let (_, third) =
                second
                    .into_iter()
                    .fold((b'A', Vec::new()), |(prev, mut acc), next| {
                        type_on_pad(&arraypad, &arraypad_idx, prev, next, &mut acc);
                        (next, acc)
                    });
            println!("{}", String::from_utf8(third.clone()).unwrap());
            println!("{} * {} = {}", num, third.len(), num * third.len() as u32);
            num * third.len() as u32
        })
        .sum::<u32>();

    println!("{}", res);
}

fn type_on_pad(
    pad: &Vec<[u8; 3]>,
    pad_index: &HashMap<u8, (usize, usize)>,
    mut cur: u8,
    next: u8,
    out: &mut Vec<u8>,
) {
    while cur != next {
        let (cur_coord, next_coord) = (pad_index[&cur], pad_index[&next]);
        if cur_coord.1 > next_coord.1 {
            let tmp = pad[cur_coord.0][next_coord.1];
            if tmp != b' ' {
                cur = tmp;
                for _ in 0..(cur_coord.1 - next_coord.1) {
                    out.push(b'<');
                }
                continue;
            }
        }
        if cur_coord.0 < next_coord.0 {
            let tmp = pad[next_coord.0][cur_coord.1];
            if tmp != b' ' {
                cur = tmp;
                for _ in 0..(next_coord.0 - cur_coord.0) {
                    out.push(b'v');
                }
                continue;
            }
        }
        if cur_coord.1 < next_coord.1 {
            let tmp = pad[cur_coord.0][next_coord.1];
            if tmp != b' ' {
                cur = tmp;
                for _ in 0..(next_coord.1 - cur_coord.1) {
                    out.push(b'>');
                }
                continue;
            }
        }
        if cur_coord.0 > next_coord.0 {
            let tmp = pad[next_coord.0][cur_coord.1];
            if tmp != b' ' {
                cur = tmp;
                for _ in 0..(cur_coord.0 - next_coord.0) {
                    out.push(b'^');
                }
                continue;
            }
        }

        unreachable!()
    }
    out.push(b'A');
}

fn to_hashmap(pad: &Vec<[u8; 3]>) -> HashMap<u8, (usize, usize)> {
    pad.iter()
        .enumerate()
        .flat_map(|(x, row)| row.into_iter().enumerate().map(move |(y, val)| (x, y, val)))
        .fold(HashMap::new(), |mut acc, (x, y, val)| {
            acc.insert(val.clone(), (x, y));
            acc
        })
}

fn get_cell(field: &Vec<Vec<u8>>, (x, y): (i32, i32)) -> Option<&u8> {
    if x < 0 || y < 0 {
        return None;
    }
    field.get(x as usize).and_then(|line| line.get(y as usize))
}
