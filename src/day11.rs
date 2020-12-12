use rustc_hash::FxHashMap;

#[derive(PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(PartialEq, Eq, Hash)]
struct Seat {
    status: Status,
    to_change: bool,
}

#[derive(PartialEq, Eq, Hash)]
enum Status {
    Empty,
    Occupied,
    Floor,
}

fn create_hash(mut input: String) -> (FxHashMap<Coord, Seat>, isize, isize) {
    let mut h: FxHashMap<Coord, Seat> = FxHashMap::default();

    // Remove last line break for ease of parsing
    input.pop();

    let mut x_count: isize = 0;
    let mut y_count: isize = 0;
    for c in input.chars() {
        let coord = Coord {
            x: x_count,
            y: y_count,
        };
        match c {
            '\n' => {
                x_count = 0;
                y_count += 1;
            }
            'L' => {
                h.insert(
                    coord,
                    Seat {
                        status: Status::Empty,
                        to_change: false,
                    },
                );
                x_count += 1;
            }
            '.' => {
                h.insert(
                    coord,
                    Seat {
                        status: Status::Floor,
                        to_change: false,
                    },
                );
                x_count += 1;
            }
            '#' => {
                h.insert(
                    coord,
                    Seat {
                        status: Status::Occupied,
                        to_change: false,
                    },
                );
                x_count += 1;
            }
            _ => (),
        };
    }

    (h, x_count, y_count)
}

fn num_adjacent(h: &FxHashMap<Coord, Seat>, coord: &Coord) -> usize {
    let mut acc: usize = 0;
    for i in (-1 as isize)..=1 {
        for j in (-1 as isize)..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            match h.get(&Coord {
                x: coord.x + i,
                y: coord.y + j,
            }) {
                Some(seat) if seat.status == Status::Occupied => acc += 1,
                _ => (),
            }
        }
    }
    acc
}

pub fn part1(input: String) {
    let (mut h, x_count, y_count) = create_hash(input);

    loop {
        for x in 0..x_count {
            for y in 0..=y_count {
                let coord = Coord { x: x, y: y };
                let adjacent = num_adjacent(&h, &coord);
                let seat = h.get_mut(&coord).unwrap();
                let to_switch: bool = match seat.status {
                    Status::Empty if adjacent == 0 => true,
                    Status::Floor => continue,
                    Status::Occupied if adjacent >= 4 => true,
                    Status::Occupied => false,
                    Status::Empty => false,
                };

                match to_switch {
                    true => h.get_mut(&coord).unwrap().to_change = true,
                    _ => (),
                }
            }
        }

        let mut acc = 0;
        h.iter_mut().for_each(|(_, mut v)| {
            match v.status {
                Status::Empty if v.to_change => {
                    acc += 1;
                    v.to_change = false;
                    v.status = Status::Occupied;
                }
                Status::Occupied if v.to_change => {
                    acc += 1;
                    v.to_change = false;
                    v.status = Status::Empty;
                }
                _ => (),
            };
        });

        if acc == 0 {
            break;
        }
    }

    println!(
        "number of occupied seats: {}",
        h.iter().fold(0, |acc, (_, v)| {
            if v.status == Status::Occupied {
                return acc + 1;
            }
            acc
        })
    )
}

fn num_adjacent_part_2(
    h: &FxHashMap<Coord, Seat>,
    coord: &Coord,
    x_max: isize,
    y_max: isize,
) -> usize {
    let mut acc: usize = 0;
    for i in (-1 as isize)..=1 {
        for j in (-1 as isize)..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            match h.get(&Coord {
                x: coord.x + i,
                y: coord.y + j,
            }) {
                Some(seat) if seat.status == Status::Occupied => acc += 1,
                Some(seat) if seat.status == Status::Floor => {
                    let mut counter = 2;
                    loop {
                        let new_x = coord.x + counter * i;
                        let new_y = coord.y + counter * j;
                        match h.get(&Coord { x: new_x, y: new_y }) {
                            Some(seat) if seat.status == Status::Occupied => {
                                acc += 1;
                                break;
                            }
                            Some(seat) if seat.status == Status::Empty => break,
                            _ => {}
                        }
                        if new_x > x_max || new_y > y_max || new_x < 0 || new_y < 0 {
                            break;
                        }
                        counter += 1;
                    }
                }
                _ => (),
            }
        }
    }
    acc
}

pub fn part2(input: String) {
    let (mut h, x_count, y_count) = create_hash(input);

    loop {
        for y in 0..=y_count {
            for x in 0..x_count {
                let coord = Coord { x: x, y: y };
                let adjacent = num_adjacent_part_2(&h, &coord, x_count - 1, y_count);
                let seat = h.get_mut(&coord).unwrap();
                let to_switch: bool = match seat.status {
                    Status::Empty if adjacent == 0 => true,
                    Status::Floor => continue,
                    Status::Occupied if adjacent >= 5 => true,
                    Status::Occupied => false,
                    Status::Empty => false,
                };

                match to_switch {
                    true => h.get_mut(&coord).unwrap().to_change = true,
                    _ => (),
                }
            }
        }

        let mut acc = 0;
        h.iter_mut().for_each(|(_, mut v)| {
            match v.status {
                Status::Empty if v.to_change => {
                    acc += 1;
                    v.to_change = false;
                    v.status = Status::Occupied;
                }
                Status::Occupied if v.to_change => {
                    acc += 1;
                    v.to_change = false;
                    v.status = Status::Empty;
                }
                _ => (),
            };
        });

        if acc == 0 {
            break;
        }
        // for y in 0..=y_count {
        // for x in 0..x_count {
        // let coord = Coord { x: x, y: y };
        // match h.get(&coord).unwrap().status {
        // Status::Empty => print!("L"),
        // Status::Floor => print!("."),
        // Status::Occupied => print!("#"),
        // }
        // }
        // println!();
        // }
        // println!();
    }

    println!(
        "number of occupied seats: {}",
        h.iter().fold(0, |acc, (_, v)| {
            if v.status == Status::Occupied {
                return acc + 1;
            }
            acc
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL\n";

    const TEST_CASE_2: &str = ".##.##.\n\
        #.#.#.#\n\
        ##...##\n\
        ...L...\n\
        ##...##\n\
        #.#.#.#\n\
        .##.##.\n";

    const TEST_CASE_3: &str = ".............\n\
        .L.L.#.#.#.#.\n\
        .............\n";

    const TEST_CASE_4: &str = ".......#.\n\
        ...#.....\n\
        .#.......\n\
        .........\n\
        ..#L....#\n\
        ....#....\n\
        .........\n\
        #........\n\
        ...#.....\n";

    #[test]
    fn day11_part_1() {
        part1(TEST_CASE_1.to_string());
    }

    #[test]
    fn day11_part_2() {
        part2(TEST_CASE_1.to_string());
    }

    #[test]
    fn day11_part_2_adj_test_case_1() {
        let (h, x_count, y_count) = create_hash(TEST_CASE_2.to_string());
        assert_eq!(
            num_adjacent_part_2(&h, &Coord { x: 3, y: 3 }, x_count, y_count),
            0
        );
    }

    #[test]
    fn day11_part_2_adj_test_case_2() {
        let (h, x_count, y_count) = create_hash(TEST_CASE_3.to_string());
        assert_eq!(
            num_adjacent_part_2(&h, &Coord { x: 1, y: 1 }, x_count, y_count),
            0
        );
    }

    #[test]
    fn day11_part_2_adj_test_case_3() {
        let (h, x_count, y_count) = create_hash(TEST_CASE_4.to_string());
        assert_eq!(
            num_adjacent_part_2(&h, &Coord { x: 3, y: 4 }, x_count, y_count),
            8
        );
    }
}
