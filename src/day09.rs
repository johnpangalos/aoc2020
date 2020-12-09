fn find_invalid(vals: &Vec<usize>, size: usize) -> &usize {
    let (_, val) = vals
        .iter()
        .enumerate()
        .find(|(i, _)| !check_prev(&vals, *i, size))
        .unwrap();
    val
}

fn check_prev(vals: &Vec<usize>, idx: usize, size: usize) -> bool {
    if idx < size {
        return true;
    }
    (idx - size..=idx).any(|i| {
        for j in idx - size..=idx {
            match j {
                index if index == i => continue,
                index if vals[idx] == vals[i] + vals[index] => return true,
                _ => (),
            }
        }
        false
    })
}

fn get_vals(input: String) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

pub fn part1(input: String) {
    let vals: Vec<usize> = get_vals(input);

    let val = find_invalid(&vals, 25);
    println!("{}", val);
}

fn find_set(vals: &Vec<usize>, sum: usize) -> (usize, usize) {
    'outer: for (curr_idx, &val) in vals.iter().enumerate() {
        let mut acc = val;
        for next_idx in curr_idx + 1..vals.len() {
            acc += vals[next_idx];
            match acc {
                acc if acc > sum => continue 'outer,
                acc if acc == sum => return (curr_idx, next_idx),
                _ => (),
            }
        }
    }
    (0, 0)
}

pub fn part2(input: String) {
    let vals: Vec<usize> = get_vals(input);

    let &sum = find_invalid(&vals, 25);

    let (start, end) = find_set(&vals, sum);
    let sol_vec = vals.get(start..end).unwrap();
    println!(
        "{}",
        sol_vec.iter().min().unwrap() + sol_vec.iter().max().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "35\n\
        20\n\
        15\n\
        25\n\
        47\n\
        40\n\
        62\n\
        55\n\
        65\n\
        95\n\
        102\n\
        117\n\
        150\n\
        182\n\
        127\n\
        219\n\
        299\n\
        277\n\
        309\n\
        576\n";

    #[test]
    fn day9_part1_case_1() {
        assert_eq!(find_invalid(&get_vals(TEST_CASE_1.to_owned()), 5), &127);
    }

    #[test]
    fn day9_part2_case_1() {
        assert_eq!(find_set(&get_vals(TEST_CASE_1.to_owned()), 127), (2, 5));
    }
}
