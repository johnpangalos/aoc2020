pub fn part1(input: String) {
    println!("{}", get_list(input).iter().max().unwrap());
}

pub fn part2(input: String) {
    let mut list = get_list(input);
    list.sort();
    let mut curr = list[0] - 1;
    for i in list.iter() {
        match i {
            _ if i - curr > 1 => {
                println!("{}", i - 1);
                break;
            }
            _ => curr += 1,
        }
    }
}

fn get_row(s: &str) -> usize {
    let col_str: &str = s.split_at(s.len() - 3).0;
    let mut lower: usize = 0;
    let mut upper: usize = 127;

    col_str.chars().for_each(|c| match c {
        'F' => upper -= (upper - lower + 1) / 2,
        'B' => lower += (upper - lower + 1) / 2,
        _ => (),
    });

    return upper;
}

fn get_col(s: &str) -> usize {
    let col_str: &str = s.split_at(s.len() - 3).1;
    let mut lower: usize = 0;
    let mut upper: usize = 7;

    col_str.chars().for_each(|c| match c {
        'L' => upper -= (upper - lower + 1) / 2,
        'R' => lower += (upper - lower + 1) / 2,
        _ => (),
    });

    return upper;
}

fn get_seat_id(s: &str) -> usize {
    let row = get_row(s);
    let col = get_col(s);
    row * 8 + col
}

fn get_list(input: String) -> Vec<usize> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| get_seat_id(l))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "FBFBBFFRLR";
    const TEST_CASE_2: &str = "BFFFBBFRRR";
    const TEST_CASE_3: &str = "FFFBBBFRRR";
    const TEST_CASE_4: &str = "BBFFBBFRLL";

    #[test]
    fn day5_test_row() {
        assert_eq!(get_row(TEST_CASE_1), 44);
        assert_eq!(get_row(TEST_CASE_2), 70);
        assert_eq!(get_row(TEST_CASE_3), 14);
        assert_eq!(get_row(TEST_CASE_4), 102);
    }

    #[test]
    fn day5_test_col() {
        assert_eq!(get_col(TEST_CASE_1), 5);
        assert_eq!(get_col(TEST_CASE_2), 7);
        assert_eq!(get_col(TEST_CASE_3), 7);
        assert_eq!(get_col(TEST_CASE_4), 4);
    }

    #[test]
    fn day5_test_seat_id() {
        assert_eq!(get_seat_id(TEST_CASE_1), 357);
        assert_eq!(get_seat_id(TEST_CASE_2), 567);
        assert_eq!(get_seat_id(TEST_CASE_3), 119);
        assert_eq!(get_seat_id(TEST_CASE_4), 820);
    }
}
