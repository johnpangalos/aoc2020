use rustc_hash::FxHashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Point {
        Point { x: x, y: y, z: z }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point4D {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Point4D {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Point4D {
        Point4D {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

struct Range {
    min: isize,
    max: isize,
}

type Space = FxHashMap<Point, bool>;
type Space4D = FxHashMap<Point4D, bool>;
type Ranges = (Range, Range, Range);
type Ranges4D = (Range, Range, Range, Range);

fn num_adjacent(m: &Space, Point { x, y, z }: Point) -> usize {
    let mut acc: usize = 0;
    for i in (-1 as isize)..=1 {
        for j in (-1 as isize)..=1 {
            for k in (-1 as isize)..=1 {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                match m.get(&Point::new(x + i, y + j, z + k)) {
                    Some(&val) if val => acc += 1,
                    _ => (),
                }
            }
        }
    }
    acc
}

fn get_ranges(m: &Space) -> Ranges {
    (
        Range {
            min: m.keys().min_by_key(|p| p.x).unwrap().x,
            max: m.keys().max_by_key(|p| p.x).unwrap().x,
        },
        Range {
            min: m.keys().min_by_key(|p| p.y).unwrap().y,
            max: m.keys().max_by_key(|p| p.y).unwrap().y,
        },
        Range {
            min: m.keys().min_by_key(|p| p.z).unwrap().z,
            max: m.keys().max_by_key(|p| p.z).unwrap().z,
        },
    )
}

pub fn part1(input: String) {
    let mut space: Space = FxHashMap::default();

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            space.insert(
                Point {
                    x: j as isize,
                    y: i as isize,
                    z: 0,
                },
                match c {
                    '#' => true,
                    _ => false,
                },
            );
        }
    }

    let mut count = 0;
    while count < 6 {
        let (x_range, y_range, z_range) = get_ranges(&space);
        let mut next: Space = FxHashMap::default();
        for i in (x_range.min - 1)..=(x_range.max + 1) {
            for j in (y_range.min - 1)..=(y_range.max + 1) {
                for k in (z_range.min - 1)..=(z_range.max + 1) {
                    let point = Point::new(i, j, k);
                    let active: bool = match space.get(&point) {
                        Some(&val) => val,
                        None => false,
                    };
                    match num_adjacent(&space, point) {
                        num if (num == 2 || num == 3) && active => (),
                        num if num == 3 && !active => {
                            next.insert(point.clone(), true);
                        }
                        _ => {
                            next.insert(point.clone(), false);
                        }
                    }
                }
            }
        }

        for (k, v) in next.iter() {
            space.insert(*k, *v);
        }
        count += 1;
    }

    println!(
        "{}",
        space.values().fold(0, |acc, &x| {
            if x {
                return acc + 1;
            }
            acc
        })
    );
}

fn num_adjacent_4_d(m: &Space4D, Point4D { x, y, z, w }: Point4D) -> usize {
    let mut acc: usize = 0;
    for i in (-1 as isize)..=1 {
        for j in (-1 as isize)..=1 {
            for k in (-1 as isize)..=1 {
                for l in (-1 as isize)..=1 {
                    if i == 0 && j == 0 && k == 0 && l == 0 {
                        continue;
                    }
                    match m.get(&Point4D::new(x + i, y + j, z + k, w + l)) {
                        Some(&val) if val => acc += 1,
                        _ => (),
                    }
                }
            }
        }
    }
    acc
}

fn get_ranges_4_d(m: &Space4D) -> Ranges4D {
    (
        Range {
            min: m.keys().min_by_key(|p| p.x).unwrap().x,
            max: m.keys().max_by_key(|p| p.x).unwrap().x,
        },
        Range {
            min: m.keys().min_by_key(|p| p.y).unwrap().y,
            max: m.keys().max_by_key(|p| p.y).unwrap().y,
        },
        Range {
            min: m.keys().min_by_key(|p| p.z).unwrap().z,
            max: m.keys().max_by_key(|p| p.z).unwrap().z,
        },
        Range {
            min: m.keys().min_by_key(|p| p.w).unwrap().w,
            max: m.keys().max_by_key(|p| p.w).unwrap().w,
        },
    )
}

pub fn part2(input: String) {
    let mut space: Space4D = FxHashMap::default();

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            space.insert(
                Point4D {
                    x: j as isize,
                    y: i as isize,
                    z: 0,
                    w: 0,
                },
                match c {
                    '#' => true,
                    _ => false,
                },
            );
        }
    }

    let mut count = 0;
    while count < 6 {
        let (x_range, y_range, z_range, w_range) = get_ranges_4_d(&space);
        let mut next: Space4D = FxHashMap::default();
        for i in (x_range.min - 1)..=(x_range.max + 1) {
            for j in (y_range.min - 1)..=(y_range.max + 1) {
                for k in (z_range.min - 1)..=(z_range.max + 1) {
                    for l in (w_range.min - 1)..=(w_range.max + 1) {
                        let point = Point4D::new(i, j, k, l);
                        let active: bool = match space.get(&point) {
                            Some(&val) => val,
                            None => false,
                        };
                        match num_adjacent_4_d(&space, point) {
                            num if (num == 2 || num == 3) && active => (),
                            num if num == 3 && !active => {
                                next.insert(point.clone(), true);
                            }
                            _ => {
                                next.insert(point.clone(), false);
                            }
                        }
                    }
                }
            }
        }

        for (k, v) in next.iter() {
            space.insert(*k, *v);
        }
        count += 1;
    }

    println!(
        "{}",
        space.values().fold(0, |acc, &x| {
            if x {
                return acc + 1;
            }
            acc
        })
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = ".#.\n\
        ..#\n\
        ###\n";

    #[test]
    fn day17_ts1() {
        part1(TEST_CASE_1.to_owned());
    }

    #[test]
    fn day17_ts2() {
        part2(TEST_CASE_1.to_owned());
    }
}
