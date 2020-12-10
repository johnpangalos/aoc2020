fn input_to_joltages(s: String) -> Vec<isize> {
    let mut vec: Vec<isize> = s.lines().map(|x| x.parse::<isize>().unwrap()).collect();
    vec.push(0);
    vec.push((vec.iter().max().unwrap() + 3) as isize);
    vec.sort();
    vec
}

pub fn part1(input: String) {
    let joltages = input_to_joltages(input);

    let (num_diff_one, num_diff_three) =
        joltages
            .iter()
            .enumerate()
            .fold((0, 0), |mut acc, (i, val)| {
                let compare: isize = match i {
                    0 => 0,
                    _ => *joltages.get(i - 1).unwrap(),
                };

                match val - compare {
                    1 => acc.0 += 1,
                    3 => acc.1 += 1,
                    _ => (),
                }
                acc
            });
    println!("{}", num_diff_one * (num_diff_three));
}

pub fn part2(input: String) {
    let joltages = input_to_joltages(input);
    let temp = joltages
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let compare: isize = match i {
                0 => 0,
                _ => *joltages.get(i - 1).unwrap(),
            };

            val - compare
        })
        .collect::<Vec<isize>>();

    let mut acc = 0;

    let mut v: Vec<isize> = vec![];
    for i in temp {
        match i {
            1 => (acc += 1),
            3 if acc >= 1 => {
                v.push(acc + 1);
                acc = 0;
            }
            _ => (),
        }
    }

    let mut answer: isize = 1;
    for i in v {
        match i {
            5 => answer *= 7,
            4 => answer *= 4,
            3 => answer *= 2,
            _ => (),
        }
    }

    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "16\n\
        10\n\
        15\n\
        5\n\
        1\n\
        11\n\
        7\n\
        19\n\
        6\n\
        12\n\
        4\n";

    const TEST_CASE_2: &str = "28\n\
        33\n\
        18\n\
        42\n\
        31\n\
        14\n\
        46\n\
        20\n\
        48\n\
        47\n\
        24\n\
        23\n\
        49\n\
        45\n\
        19\n\
        38\n\
        39\n\
        11\n\
        1\n\
        32\n\
        25\n\
        35\n\
        8\n\
        17\n\
        7\n\
        9\n\
        4\n\
        2\n\
        34\n\
        10\n\
        3\n";

    #[test]
    fn day10_part1_case1() {
        part1(TEST_CASE_1.to_string());
    }

    #[test]
    fn day10_part1_case2() {
        part1(TEST_CASE_2.to_string());
    }

    #[test]
    fn day10_part2_case1() {
        part2(TEST_CASE_1.to_string());
    }

    #[test]
    fn day10_part2_case2() {
        part2(TEST_CASE_2.to_string());
    }
}
