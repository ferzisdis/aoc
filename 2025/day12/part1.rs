use std::{
    collections::{vec_deque, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    u32, vec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Diff {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    form: Vec<Diff>,
    coat: Vec<Diff>,
}

impl Shape {
    fn from(shape: Vec<String>) -> Self {
        let form = shape
            .into_iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(j, c)| {
                        if c == '#' {
                            Some(Diff {
                                x: j as i32,
                                y: i as i32,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let coat = form
            .iter()
            .flat_map(|&Diff { x, y }| {
                vec![
                    Diff { x: x + 1, y },
                    Diff { x: x - 1, y },
                    Diff { x, y: y + 1 },
                    Diff { x, y: y - 1 },
                    Diff { x: x + 1, y: y + 1 },
                    Diff { x: x - 1, y: y - 1 },
                    Diff { x: x + 1, y: y - 1 },
                    Diff { x: x - 1, y: y + 1 },
                ]
            })
            .filter(|diff| !form.contains(diff))
            .collect::<Vec<_>>();

        Shape { form, coat }
    }
}

fn main() {
    let mut it = BufReader::new(File::open("day12.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap());

    let mut shapes = Vec::new();
    for _ in 0..6 {
        it.next().unwrap(); // line with num
        let shape = (0..3).map(|_| it.next().unwrap()).collect::<Vec<String>>();
        shapes.push(shape);
        it.next().unwrap(); // empty line
    }

    let shapes = rotate_shapes(shapes);

    for shape in shapes.iter().flat_map(|s| s).take(10) {
        let mut field = vec![vec![0; 6 as usize]; 6 as usize];
        //let mut starts = HashSet::new();
        println!("shape: {:?} ({:?})", shape.form.len(), shape.coat.len());
        // let new_starts = place_shape(3, shape, &mut field, &mut starts, 3, 3);
        // print_field(&field, &starts);
        // println!();
        // unplace_shape(3, shape, &mut field, &mut starts, new_starts, 3, 3);
    }

    let mut total = 0;
    for line in it {
        println!("solve {}", line);

        let (region, quantities) = line.split_once(": ").unwrap();
        let (width, height) = region.split_once("x").unwrap();
        let (width, height) = (
            width.parse::<u32>().unwrap(),
            height.parse::<u32>().unwrap(),
        );
        let mut quantities = quantities
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();

        let mut field = vec![vec![0; width as usize]; height as usize];
        let mut starts = HashSet::new();
        let area = width * height;
        let wanted_area = quantities
            .iter()
            .enumerate()
            .map(|(i, &count)| count * shapes[i][0].form.len() as u32)
            .sum::<u32>();

        for i in 0..3 {
            for j in 0..3 {
                starts.insert((i, j));
            }
        }
        let mut cache = HashSet::new();
        let result = solve(
            &shapes,
            &mut quantities,
            &mut field,
            &mut starts,
            0,
            &mut cache,
            area,
            wanted_area,
        );
        if result {
            total += 1;
        }
        println!("result {} ({})", result, cache.len());
    }

    println!("total: {}", total);
}

fn solve(
    shapes: &Vec<Vec<Shape>>,
    quantities: &mut Vec<u32>,
    field: &mut Vec<Vec<u8>>,
    starts: &mut HashSet<(u32, u32)>,
    depth: u32,
    cache: &mut HashSet<(Vec<Vec<u8>>, Vec<u32>)>,
    mut available_area: u32,
    wanted_area: u32,
) -> bool {
    if available_area < wanted_area {
        return false;
    }

    if quantities.iter().all(|x| *x == 0) {
        println!("found");
        print_field(field, starts);
        return true;
    }

    let mut unreached = Vec::new();
    let mut reached = Vec::new();

    for (x, y) in starts.iter() {
        let mut placed = false;
        for (shape_id, _) in quantities.iter().enumerate().filter(|(_, x)| **x != 0) {
            let shapes = &shapes[shape_id];

            for shape in shapes {
                if can_place(shape, field, *x, *y) {
                    placed = true;
                    reached.push((*x, *y, shape_id, shape));
                }
            }
        }
        if !placed && field[*x as usize][*y as usize] == 0 {
            unreached.push((*x, *y, true));
        }
    }

    // let before_unreached = field.clone();
    let mut i = 0;
    while i < unreached.len() {
        let (x, y, removed) = unreached[i];
        i += 1;

        if removed {
            starts.remove(&(x, y));
        }

        assert_eq!(field[x as usize][y as usize], 0);
        available_area -= 1;
        field[x as usize][y as usize] = 2;

        for (add_x, add_y) in [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ] {
            let (x, y) = (x.wrapping_add_signed(add_x), y.wrapping_add_signed(add_y));
            if x >= field.len() as u32
                || y >= field[0].len() as u32
                || field[x as usize][y as usize] != 0
            {
                continue;
            }

            let mut placed = false;
            for (shape_id, _) in quantities.iter().enumerate().filter(|(_, x)| **x != 0) {
                let shapes = &shapes[shape_id];

                for shape in shapes {
                    if can_place(shape, field, x, y) {
                        placed = true;
                        break;
                    }
                }
                if placed {
                    break;
                }
            }
            if !placed {
                if !unreached.contains(&(x, y, true)) && !unreached.contains(&(x, y, false)) {
                    unreached.push((x, y, false));
                }
            }
        }
    }

    if cache.insert((field.clone(), quantities.clone())) {
        if cache.len() % 1000 == 0 {
            println!(
                "place in cache {} ({} of {})",
                cache.len(),
                wanted_area,
                available_area
            );
            // print_field(&before_unreached, starts);
            println!("{:?}", quantities);
            print_field(field, starts);
        }
        // println!("place in cache {}", cache.len());

        reached.sort_by_key(|(x, y, _, _)| (*x, *y));
        for (x, y, shape_id, shape) in reached {
            let marker = 1;
            let mut available_area = available_area;
            let mut wanted_area = wanted_area;
            let new_starts = place_shape(
                marker,
                shape,
                field,
                starts,
                x,
                y,
                &mut available_area,
                &mut wanted_area,
            );
            quantities[shape_id] -= 1;
            if solve(
                shapes,
                quantities,
                field,
                starts,
                depth + 1,
                cache,
                available_area,
                wanted_area,
            ) {
                return true;
            }
            quantities[shape_id] += 1;
            unplace_shape(marker, shape, field, starts, new_starts, x, y);
        }
    } else {
        // println!("cache hit");
    }

    for (x, y, removed) in unreached {
        assert!(field[x as usize][y as usize] == 2);
        field[x as usize][y as usize] = 0;
        if removed {
            starts.insert((x, y));
        }
    }
    false
}

fn print_field(field: &Vec<Vec<u8>>, starts: &HashSet<(u32, u32)>) {
    for (i, row) in field.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            print!(
                "{}",
                (if *cell == 9 {
                    b'.'
                } else if *cell > 0 {
                    b'0' + *cell
                } else if starts.contains(&(i as u32, j as u32)) {
                    b'^'
                } else {
                    b'.'
                }) as char
            );
        }
        println!();
    }
}

fn can_place(shape: &Shape, field: &Vec<Vec<u8>>, x: u32, y: u32) -> bool {
    for diff in shape.form.iter() {
        let (x, y) = (x.wrapping_add_signed(diff.x), y.wrapping_add_signed(diff.y));
        if x >= field.len() as u32
            || y >= field[0].len() as u32
            || field[x as usize][y as usize] != 0
        {
            return false;
        }
    }
    true
}

fn place_shape(
    shape_idx: usize,
    shape: &Shape,
    field: &mut Vec<Vec<u8>>,
    starts: &mut HashSet<(u32, u32)>,
    x: u32,
    y: u32,
    available_area: &mut u32,
    wanted_area: &mut u32,
) -> Vec<(u32, u32)> {
    for diff in shape.form.iter() {
        let (x, y) = (x.wrapping_add_signed(diff.x), y.wrapping_add_signed(diff.y));
        assert_eq!(field[x as usize][y as usize], 0);
        field[x as usize][y as usize] = shape_idx as u8 + 1;
        *available_area -= 1;
        *wanted_area -= 1;
    }

    let mut new_starts = Vec::new();
    for diff in shape.coat.iter() {
        let (x, y) = (x.wrapping_add_signed(diff.x), y.wrapping_add_signed(diff.y));
        if x < field.len() as u32 && y < field[0].len() as u32 && field[x as usize][y as usize] == 0
        {
            if starts.insert((x, y)) {
                new_starts.push((x, y));
            }
        }
    }
    new_starts
}

fn unplace_shape(
    shape_idx: usize,
    shape: &Shape,
    field: &mut Vec<Vec<u8>>,
    starts: &mut HashSet<(u32, u32)>,
    new_starts: Vec<(u32, u32)>,
    x: u32,
    y: u32,
) {
    for diff in shape.form.iter() {
        let (x, y) = (x.wrapping_add_signed(diff.x), y.wrapping_add_signed(diff.y));
        assert!(
            field[x as usize][y as usize] == shape_idx as u8 + 1,
            "Val: {}, Shape: {}, Field: ({}, {})",
            field[x as usize][y as usize],
            shape_idx,
            x,
            y
        );
        field[x as usize][y as usize] = 0;
    }

    for (x, y) in new_starts {
        starts.remove(&(x, y));
    }
}

fn rotate_shapes(shapes: Vec<Vec<String>>) -> Vec<Vec<Shape>> {
    let mut result = Vec::new();
    for mut shape in shapes {
        let mut hs = HashSet::new();
        for _ in 0..4 {
            hs.insert(shape.clone());
            turn_right(&mut shape);
        }
        flip(&mut shape);
        for _ in 0..4 {
            hs.insert(shape.clone());
            turn_right(&mut shape);
        }

        result.push(
            hs.into_iter()
                .flat_map(|shape| {
                    let shape = Shape::from(shape);
                    let mut shapes = Vec::new();
                    for i in 0..3 {
                        for j in 0..3 {
                            shapes.push(Shape {
                                form: shape
                                    .form
                                    .iter()
                                    .map(|diff| Diff {
                                        x: diff.x - i,
                                        y: diff.y - j,
                                    })
                                    .collect::<Vec<_>>(),
                                coat: shape
                                    .coat
                                    .iter()
                                    .map(|diff| Diff {
                                        x: diff.x - i,
                                        y: diff.y - j,
                                    })
                                    .collect::<Vec<_>>(),
                            });
                        }
                    }
                    shapes
                })
                .collect::<Vec<_>>(),
        )
    }
    result
}

fn turn_right(shape: &mut Vec<String>) {
    let mut new_shape = vec![String::new(); shape.len()];
    for (_, line) in shape.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            new_shape[j].push(c);
        }
    }
    *shape = new_shape;
}

fn flip(shape: &mut Vec<String>) {
    let mut new_shape = Vec::new();
    for line in shape.iter() {
        let mut new_line = String::new();
        for c in line.chars().rev() {
            new_line.push(c);
        }
        new_shape.push(new_line);
    }
    *shape = new_shape;
}
