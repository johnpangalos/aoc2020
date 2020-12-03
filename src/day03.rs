use std::collections::HashMap;

type Coord = (usize, usize);
type Slope = (usize, usize);

fn parse_input(mut input: String) -> (HashMap<Coord, char>, usize, usize) {
    let mut count_x = 0;
    let mut count_y = 0;
    let mut max_x = 0;
    let mut m = HashMap::new();

    // Remove trailing \n character
    input.pop();

    for c in input.chars() {
        if c == '\n' {
            count_x = 0;
            count_y += 1;
            continue;
        }

        m.insert((count_x, count_y), c);

        if max_x < count_x {
            max_x += 1;
        }
        count_x += 1;
    }
    return (m, max_x, count_y);
}

fn count_trees(m: &HashMap<Coord, char>, max_x: usize, max_y: usize, pattern: Coord) -> usize {
    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut num_trees = 0;
    let (x, y) = pattern;

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
    let mut sum = 1;
    let slopes: Vec<Slope> = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].to_vec();
    for s in slopes {
        sum *= count_trees(&m, max_x, max_y, s);
    }
    println!("{}", sum);
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
