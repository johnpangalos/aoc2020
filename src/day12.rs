use std::fmt;

struct NavInst {
    dir: Direction,
    val: isize,
}

impl NavInst {
    fn new(s: &str) -> NavInst {
        let (dir, val) = s.split_at(1);
        NavInst {
            dir: str_to_dir(dir),
            val: val.parse::<isize>().unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

type Coord = (isize, isize);

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
            Direction::Forward => write!(f, "Forward"),
        }
    }
}

fn str_to_dir(s: &str) -> Direction {
    match s {
        "N" => Direction::North,
        "S" => Direction::South,
        "E" => Direction::East,
        "W" => Direction::West,
        "L" => Direction::Left,
        "R" => Direction::Right,
        "F" => Direction::Forward,
        _ => panic!("Direction not expected: {}", s),
    }
}

fn get_radians(dir: Direction) -> isize {
    match dir {
        Direction::North => 0,
        Direction::East => 90,
        Direction::South => 180,
        Direction::West => 270,
        _ => panic!("Invalid compass direction: {}", dir),
    }
}

fn get_direction(rad: isize) -> Direction {
    match rad.abs() {
        0 => Direction::North,
        90 => Direction::East,
        180 => Direction::South,
        270 => Direction::West,
        _ => panic!("Invalid radian: {}", rad),
    }
}

fn turn_right(dir: Direction, val: &isize) -> Direction {
    get_direction((get_radians(dir) + val) % 360)
}

fn turn_left(dir: Direction, val: &isize) -> Direction {
    get_direction((get_radians(dir) + 360 - val) % 360)
}

fn turn_waypoint_right(val: &isize, waypoint: Coord) -> Coord {
    let (x, y) = waypoint;
    match val {
        90 => (y, -1 * x),
        180 => (-1 * x, -1 * y),
        270 => (-1 * y, x),
        _ => panic!("Right turn failed - Value: {}", val),
    }
}

fn turn_waypoint_left(val: &isize, waypoint: Coord) -> Coord {
    let (x, y) = waypoint;
    match val {
        90 => (-1 * y, x),
        180 => (-1 * x, -1 * y),
        270 => (y, -1 * x),
        _ => panic!("Left turn failed - Value: {}", val),
    }
}

pub fn part1(input: String) {
    let mut curr_facing = Direction::East;
    let nav_inst: Vec<NavInst> = input.lines().map(|l| NavInst::new(l)).collect();
    let (x, y) = nav_inst
        .iter()
        .fold((0 as isize, 0 as isize), |mut acc, NavInst { dir, val }| {
            match dir {
                Direction::North => acc.1 += *val as isize,
                Direction::South => acc.1 -= *val as isize,
                Direction::East => acc.0 += *val as isize,
                Direction::West => acc.0 -= *val as isize,
                Direction::Forward => match curr_facing {
                    Direction::North => acc.1 += *val as isize,
                    Direction::South => acc.1 -= *val as isize,
                    Direction::East => acc.0 += *val as isize,
                    Direction::West => acc.0 -= *val as isize,
                    _ => (),
                },
                Direction::Right => {
                    let next_dir = turn_right(curr_facing, val);
                    curr_facing = next_dir;
                }
                Direction::Left => {
                    let next_dir = turn_left(curr_facing, val);
                    curr_facing = next_dir;
                }
            }
            acc
        });
    println!("{}", x.abs() + y.abs());
}

pub fn part2(input: String) {
    let mut ship: Coord = (0, 0);
    let nav_inst: Vec<NavInst> = input.lines().map(|l| NavInst::new(l)).collect();
    nav_inst.iter().fold(
        (10 as isize, 1 as isize),
        |mut acc, NavInst { dir, val }| {
            let v = *val as isize;

            match dir {
                Direction::North => acc.1 += v,
                Direction::South => acc.1 -= v,
                Direction::East => acc.0 += v,
                Direction::West => acc.0 -= v,
                Direction::Forward => {
                    ship.0 += acc.0 * v;
                    ship.1 += acc.1 * v;
                }
                Direction::Right => {
                    let next = turn_waypoint_right(val, acc);
                    acc = next;
                }
                Direction::Left => {
                    let next = turn_waypoint_left(val, acc);
                    acc = next;
                }
            }
            acc
        },
    );
    println!("{}", ship.0.abs() + ship.1.abs());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "F10\n\
        N3\n\
        F7\n\
        R90\n\
        F11\n";

    #[test]
    fn day12_part1() {
        part1(TEST_CASE_1.to_string());
    }

    #[test]
    fn day12_part2() {
        part2(TEST_CASE_1.to_string());
    }
}
