use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use union_find::{QuickFindUf, UnionBySize, UnionFind};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

#[derive(Debug, Clone, Copy)]
struct ColoredRect {
    r: Rect,
    left: bool,
}

fn area(lhs: &Point, rhs: &Point) -> i64 {
    (rhs.x.abs_diff(lhs.x) + 1) as i64 * (rhs.y.abs_diff(lhs.y) + 1) as i64
}

fn is_line_intersect(l1: i64, l2: i64, r1: i64, r2: i64) -> bool {
    l1 <= r2 && l2 >= r1
}

fn is_intersect(r1: &Rect, r2: &Rect) -> bool {
    is_line_intersect(r1.x1, r1.x2, r2.x1, r2.x2) && is_line_intersect(r1.y1, r1.y2, r2.y1, r2.y2)
}

fn split_rect(
    first: Point,
    second: Point,
    third: Point,
    colored_rect: ColoredRect,
    res: &mut Vec<ColoredRect>,
) {
    let rect = colored_rect.r;
    if !is_intersect(
        &Rect {
            x1: first.x.min(second.x),
            x2: first.x.max(second.x),
            y1: first.y.min(second.y),
            y2: first.y.max(second.y),
        },
        &rect,
    ) {
        res.push(colored_rect);
        return;
    }

    match second.x - first.x {
        0 => {
            assert!(first.x == second.x);
            assert!(first.x != third.x);

            let is_up_to_down = first.y < second.y;
            if first.x + 1 <= rect.x2 {
                res.push(ColoredRect {
                    r: Rect {
                        x1: first.x + 1,
                        x2: rect.x2,
                        y1: rect.y1,
                        y2: rect.y2,
                    },
                    left: is_up_to_down,
                });
            }
            if first.x - 1 >= rect.x1 {
                res.push(ColoredRect {
                    r: Rect {
                        x1: rect.x1,
                        x2: first.x - 1,
                        y1: rect.y1,
                        y2: rect.y2,
                    },
                    left: !is_up_to_down,
                });
            }

            if is_up_to_down {
                if second.y + 1 <= rect.y2 {
                    res.push(ColoredRect {
                        r: Rect {
                            x1: second.x,
                            x2: second.x,
                            y1: second.y + 1,
                            y2: rect.y2,
                        },
                        left: third.x < first.x,
                    });
                }
                if first.y - 1 >= rect.y1 {
                    res.push(ColoredRect {
                        r: Rect {
                            x1: second.x,
                            x2: second.x,
                            y1: rect.y1,
                            y2: first.y - 1,
                        },
                        left: true,
                    });
                }
            } else {
                if second.y - 1 >= rect.y1 {
                    res.push(ColoredRect {
                        r: Rect {
                            x1: second.x,
                            x2: second.x,
                            y1: rect.y1,
                            y2: second.y - 1,
                        },
                        left: third.x > first.x,
                    });
                }
                if first.y + 1 <= rect.y2 {
                    res.push(ColoredRect {
                        r: Rect {
                            x1: second.x,
                            x2: second.x,
                            y1: first.y + 1,
                            y2: rect.y2,
                        },
                        left: true,
                    });
                }
            }
        }
        _ => {
            assert!(first.y == second.y);
            assert!(first.y != third.y);

            let is_left_to_right = first.x < second.x;
            if first.y + 1 <= rect.y2 {
                res.push(ColoredRect {
                    r: Rect {
                        x1: rect.x1,
                        x2: rect.x2,
                        y1: first.y + 1,
                        y2: rect.y2,
                    },
                    left: !is_left_to_right,
                });
            }
            if first.y - 1 >= rect.y1 {
                res.push(ColoredRect {
                    r: Rect {
                        x1: rect.x1,
                        x2: rect.x2,
                        y1: rect.y1,
                        y2: first.y - 1,
                    },
                    left: is_left_to_right,
                });
            }

            if is_left_to_right {
                if second.x + 1 <= rect.x2 {
                    res.push(ColoredRect {
                        r: Rect {
                            x1: second.x + 1,
                            x2: rect.x2,
                            y1: first.y,
                            y2: first.y,
                        },
                        left: third.y > first.y,
                    });
                }
                if first.x - 1 >= rect.x1 {
                    res.push(ColoredRect {
                        r: Rect {
                            x1: rect.x1,
                            x2: first.x - 1,
                            y1: first.y,
                            y2: first.y,
                        },
                        left: true,
                    });
                }
            } else {
                if second.x - 1 >= rect.x1 {
                    res.push(ColoredRect {
                        r: Rect {
                            x1: rect.x1,
                            x2: second.x - 1,
                            y1: first.y,
                            y2: first.y,
                        },
                        left: third.y < first.y,
                    });
                }
                if first.x + 1 <= rect.x2 {
                    res.push(ColoredRect {
                        r: Rect {
                            x1: first.x + 1,
                            x2: rect.x2,
                            y1: first.y,
                            y2: first.y,
                        },
                        left: true,
                    });
                }
            }
        }
    }
}

fn build_outside_rects(path: &Vec<Point>) -> Vec<Rect> {
    let mut init = Vec::new();
    init.push(ColoredRect {
        r: Rect {
            x1: 0,
            x2: 100000,
            y1: 0,
            y2: 100000,
        },
        left: true,
    });
    let (rects, left_cnt) = (0..path.len()).fold((init, 0i32), |(rects, mut left_cnt), i| {
        let mut new_rects = Vec::new();
        let (first, second, third) = (
            path[i],
            path[(i + 1) % path.len()],
            path[(i + 2) % path.len()],
        );
        println!("rects cnt: {}", rects.len());
        for rect in rects {
            split_rect(first, second, third, rect, &mut new_rects);
        }

        if first.y == second.y
            && (first.x < second.x && third.y < second.y
                || first.x > second.x && third.y > second.y)
            || first.x == second.x
                && (first.y < second.y && third.x > second.x
                    || first.y > second.y && third.x < second.x)
        {
            println!("turn left");
            left_cnt += 1
        } else {
            println!("turn right");
            left_cnt -= 1
        }

        (new_rects, left_cnt)
    });

    println!("lefts {}", left_cnt);
    rects
        .into_iter()
        .filter(|cr| left_cnt > 0 && !cr.left || left_cnt < 0 && cr.left)
        .map(|cr| cr.r)
        .collect::<Vec<_>>()
}

fn main() {
    let points = BufReader::new(File::open("day09.txt").expect("Happy Christmas!!!"))
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let mut it = x.split(',');
            Point {
                x: it.next().unwrap().parse::<i64>().unwrap(),
                y: it.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let outside_rects = build_outside_rects(&points);
    for rect in outside_rects.iter() {
        assert!(rect.x1 <= rect.x2);
        assert!(rect.y1 <= rect.y2);
        println!(
            "rect: x1={}, x2={}, y1={}, y2={}",
            rect.x1, rect.x2, rect.y1, rect.y2
        );
    }

    let mut pairs = (0..points.len())
        .flat_map(|l| ((l + 1)..points.len()).map(move |r| (l.clone(), r.clone())))
        .collect::<Vec<_>>();

    pairs.sort_by_key(|(l, r)| -area(&points[*l], &points[*r]));

    for pair in pairs {
        let rect = Rect {
            x1: points[pair.0].x.min(points[pair.1].x),
            x2: points[pair.0].x.max(points[pair.1].x),
            y1: points[pair.0].y.min(points[pair.1].y),
            y2: points[pair.0].y.max(points[pair.1].y),
        };
        if outside_rects
            .iter()
            .any(|out_rect| is_intersect(&rect, out_rect))
        {
            continue;
        }

        println!("found!");
        println!("{}", area(&points[pair.0], &points[pair.1]));
        return;
    }
}
