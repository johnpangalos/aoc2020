use std::collections::HashMap;

type Coord = (usize, usize);
type Slope = (usize, usize);

fn parse_input(mut input: String) -> (HashMap<Coord, char>, usize, usize) {
    let (mut count_x, mut count_y, mut max_x) = (0, 0, 0);
    let mut m = HashMap::new();

    // Remove trailing \n character
    input.pop();

    for c in input.chars() {
        match c {
            '\n' => {
                count_x = 0;
                count_y += 1;
            }
            _ => {
                m.insert((count_x, count_y), c);
                if max_x < count_x {
                    max_x += 1;
                }
                count_x += 1;
            }
        }
    }
    return (m, max_x, count_y);
}

fn count_trees(m: &HashMap<Coord, char>, max_x: usize, max_y: usize, slope: Slope) -> usize {
    let (mut curr_x, mut curr_y, mut num_trees) = (0, 0, 0);
    let (x, y) = slope;

    loop {
        curr_x += x;
        curr_y += y;
        if curr_x > max_x {
            curr_x = (curr_x % max_x) - 1;
        }
        if curr_y > max_y {
            break;
        }
        if *m.get(&(curr_x, curr_y)).unwrap() == '#' {
            num_trees += 1;
        }
    }
    return num_trees;
}

pub fn part1(input: String) {
    let (m, max_x, max_y) = parse_input(input);
    println!("{}", count_trees(&m, max_x, max_y, (3, 1)));
}

pub fn part2(input: String) {
    let (m, max_x, max_y) = parse_input(input);
    let slopes: Vec<Slope> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product = slopes
        .iter()
        .fold(1, |acc, s| acc * count_trees(&m, max_x, max_y, *s));
    println!("{}", product);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "..##.......\n\
     #...#...#..\n\
     .#....#..#.\n\
     ..#.#...#.#\n\
     .#...##..#.\n\
     ..#.##.....\n\
     .#.#.#....#\n\
     .#........#\n\
     #.##...#...\n\
     #...##....#\n\
     .#..#...#.#\n";

    #[test]
    fn day3_part1_test_case_1() {
        part1(TEST_STR.to_owned());
    }
    #[test]
    fn day3_part2_test_case_1() {
        part2(TEST_STR.to_owned());
    }
}
