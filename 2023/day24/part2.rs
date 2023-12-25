use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::id;

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64
}

#[derive(Copy, Clone)]
struct VectorInterval {
    coord_range: (i64, i64),
    angle_range: (i64, i64),
}

fn main() {
    println!("{}", i64::MAX);
    let points = BufReader::new(File::open("day24.txt").expect("blocked by antivirus"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            let mut it = x.split('@').flat_map(|x| x.split(',')).map(|x| x.trim()).map(|x| x.parse::<i64>().unwrap());
            Point {

                x: it.next().unwrap(),
                y: it.next().unwrap(),
                z: it.next().unwrap(),


                dx: it.next().unwrap(),
                dy: it.next().unwrap(),
                dz: it.next().unwrap(),
            }
        }).collect::<Vec<_>>();


    for alpha in -1000..=1000 {
        let mut is_break = false;
        let mut combination = None;
        for (idx, lhs) in points.iter().enumerate() {
            for rhs in points.iter().skip(idx + 1) {
                let coll = find_collision_ex((lhs.x, lhs.dx - alpha), (rhs.x, rhs.dx - alpha));
                if coll.0 == 0 {
                    is_break = true;
                    break
                }

                if combination.is_none() {
                    combination = Some(coll);
                } else {
                    let res = find_collision_ex(combination.unwrap(), (rhs.x, rhs.dx - alpha));
                    if res.0 == 0 {
                        is_break = true;
                        break;
                    }
                    combination = Some(res);
                }
            }
            if is_break {
                break
            }
        }
        if is_break {
            continue
        }

        println!("alpha x {} {:?}", alpha, combination);
    }

    for alpha in -1000..=1000 {
        let mut is_break = false;
        let mut combination = None;
        for (idx, lhs) in points.iter().enumerate() {
            for rhs in points.iter().skip(idx + 1) {
                let coll = find_collision_ex((lhs.y, lhs.dy - alpha), (rhs.y, rhs.dy - alpha));
                if coll.0 == 0 {
                    is_break = true;
                    break
                }

                if combination.is_none() {
                    combination = Some(coll);
                } else {
                    let res = find_collision_ex(combination.unwrap(), (rhs.y, rhs.dy - alpha));
                    if res.0 == 0 {
                        is_break = true;
                        break;
                    }
                    combination = Some(res);
                }
            }
            if is_break {
                break
            }
        }
        if is_break {
            continue
        }

        println!("alpha y {} {:?}", alpha, combination);
    }

    for alpha in -1000..=1000 {
        let mut is_break = false;
        let mut combination = None;
        for (idx, lhs) in points.iter().enumerate() {
            for rhs in points.iter().skip(idx + 1) {
                if combination.is_none() {
                    let coll = find_collision_ex((lhs.z, lhs.dz - alpha), (rhs.z, rhs.dz - alpha));
                    if coll.0 == 0 && coll.1 == 0 {
                        is_break = true;
                        break
                    }
                    combination = Some(coll);
                } else {
                    let res = find_collision_ex(combination.unwrap(), (rhs.z, rhs.dz - alpha));
                    if res.0 == 0 && res.1 == 0 {
                        is_break = true;
                        break;
                    }
                    combination = Some(res);
                }
            }
            if is_break {
                break
            }
        }
        if is_break {
            continue
        }

        println!("alpha z {} {:?}", alpha, combination);
    }

}

fn find_collision(lhs: Point, rhs: Point, alpha: i64) -> (i64, i64) {
    let mut dx = rhs.x - lhs.x;
    let mut ddxl = lhs.dx - alpha;
    let mut ddxr = rhs.dx - alpha;

    if ddxl < 0 {
        (dx, ddxl, ddxr) = (dx * -1, ddxl * -1, ddxr * -1)
    }
    if ddxr == 0 {
        if ddxl == 0 {
            if rhs.x == lhs.x {
                return (lhs.x, 0);
            } else {
                return (0, 0);
            }
        }
        if dx.rem_euclid(ddxl) == 0  {
            let t = dx / ddxl;
            return (lhs.x + t * (lhs.dx - alpha), 0)
        }
        return (0, 0)
    }
    if ddxl == 0 {
        return find_collision(rhs, lhs, alpha)
    }

    let t2 = (-dx / ddxr).max(0);
    assert!(t2 >= 0);
    let mut hs = HashSet::new();
    let mut t = t2;
    loop {
        if t.checked_mul(ddxr).is_none() {
            return (0, 0)
        }
        let re = (dx + t * ddxr).rem_euclid(ddxl);
        let mut tadd = 1;
        let rem_ost = (ddxl - re) / ddxr;
        if rem_ost > 0 {
            tadd = rem_ost;
        }
        if hs.contains(&re) {
            return (0, 0)
        }
        hs.insert(re);
        if re == 0 {
            let t1 = (dx + t * ddxr) / ddxl;
            if t1 < 0 {
                if t1 < 10 && ddxr < 0 {
                    return (0, 0)
                }
                hs.remove(&re);
                t = t + tadd;
                continue
            }

            // println!("t0 = {}, t1= {}", t1, t);
            assert_eq!(rhs.x + t * (rhs.dx - alpha), lhs.x + t1 * (lhs.dx - alpha));
            return (rhs.x + t * (rhs.dx - alpha), lcm(rhs.dx - alpha, lhs.dx - alpha))
        }

        t = t + tadd
    }
    (0, 0)
}

fn find_collision_ex((lhs_x, lhs_dx): (i64, i64), (rhs_x, rhs_dx): (i64, i64))  -> (i64, i64) {
    let mut dx = rhs_x - lhs_x;
    let mut ddl = lhs_dx;
    let mut ddr = rhs_dx;
    if ddl < 0 {
        (dx, ddl, ddr) = (dx * -1, ddl * -1, ddr * -1)
    }

    if ddr == 0 {
        if ddl == 0 {
            if rhs_x == lhs_x {
                return (lhs_x, 0);
            } else {
                return (0, 0);
            }
        }
        if dx.rem_euclid(ddl) == 0  {
            let t = dx / ddl;
            return (lhs_x + t * lhs_dx, 0)
        }
        return (0, 0)
    }

    if ddl == 0 {
        return find_collision_ex((rhs_x, rhs_dx), (lhs_x, lhs_dx))
    }

    if ddr < 0 {
        if dx < 0 {
            return (0, 0);
        }
        ddr = -ddr;
        dx = dx - (dx / ddr) * ddr
    }

    let t = find_period(dx, ddl, ddr);
    if t < 0 {
        return (0, 0);
    }

    return (lhs_x + t * lhs_dx, lcm(ddl, ddr))
}

fn find_period(mut dx: i64, ddl: i64, ddr: i64) -> i64 {
    assert!(ddl > 0 && ddr > 0);

    if dx < 0 {
        let ost = dx - (dx / ddr) * ddr;
        if ost < 0 {
            dx = ost + ddr;
        } else {
            dx = ost;
        }
    }

    assert!(dx >= 0);

    let mut hs = HashSet::new();
    let mut t = 0i64;

    loop {
        if t.checked_mul(ddr).is_none() {
            return -1
        }
        let re = (dx + t * ddr).rem_euclid(ddl);
        let mut tadd = 1;
        let rem_ost = (ddl - re) / ddr;
        if rem_ost > 0 {
            tadd = rem_ost;
        }
        if hs.contains(&re) {
            return -1
        }
        hs.insert(re);
        if re == 0 {
            let t1 = (dx + t * ddr) / ddl;
            return t1;
        }

        t = t + tadd
    }
}

fn merge_intervals(lhs: Vec<(i128, i128)>, rhs: Vec<(i128, i128)>) -> Vec<(i128, i128)> {
    let mut res = Vec::new();
    for l in lhs {
        for r in &rhs {
            let intersect = (l.0.max(r.0), l.1.min(r.1));
            if intersect.0 <= intersect.1 {
                res.push(intersect)
            }
        }
    }
    res
}

fn compress_intervals(intervals: Vec<VectorInterval>) -> Vec<VectorInterval> {
    intervals.into_iter().fold(vec![VectorInterval { coord_range: (i64::MIN, i64::MAX), angle_range: (i64::MIN, i64::MAX) }], |acc, point| {
        let mut v = Vec::new();
        for prev in acc {
            let intersect = VectorInterval {
                coord_range: (prev.coord_range.0.max(point.coord_range.0), prev.coord_range.1.min(point.coord_range.1)),
                angle_range: (prev.angle_range.0.max(point.angle_range.0), prev.angle_range.1.min(point.angle_range.1)),
            };
            if intersect.coord_range.0 < intersect.coord_range.1  {
                if intersect.angle_range.0 < intersect.angle_range.1 {
                    v.push(intersect);
                }
                if intersect.coord_range.0 > prev.coord_range.0 {
                    v.push(VectorInterval {
                        coord_range: (prev.coord_range.0, intersect.coord_range.0),
                        angle_range: prev.angle_range,
                    })
                }
                if intersect.coord_range.1 < prev.coord_range.1 {
                    v.push(VectorInterval {
                        coord_range: (intersect.coord_range.1, prev.coord_range.1),
                        angle_range: prev.angle_range,
                    })
                }
            }else {
                v.push(prev)
            }
        }
        v
    })
}

fn lcm(first: i64, second: i64) -> i64 {
    first.checked_mul(second).map(|x| x / gcd(first, second))
        .map(|x| if x > 15983154909268200 || x < -15983154909268200 { 0 } else { x }).unwrap_or(0)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
