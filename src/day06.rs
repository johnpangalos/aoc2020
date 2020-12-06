pub fn part1(input: String) {
    let mut curr_chars: Vec<char> = vec![];
    let mut lines = input.lines().collect::<Vec<&str>>();
    lines.push("");
    let sum = lines.iter().fold(0, |mut acc, l| {
        match l {
            l if l.is_empty() => {
                acc += curr_chars.len();
                curr_chars = vec![];
            }
            _ => l.chars().for_each(|c| match c {
                c if curr_chars.contains(&c) => (),
                _ => curr_chars.push(c),
            }),
        }
        return acc;
    });
    println!("{}", sum);
}

pub fn part2(input: String) {
    let mut curr_chars: Vec<char> = vec![];
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut lines_fixed = vec![""];
    lines_fixed.append(&mut lines);
    lines_fixed.append(&mut vec![""]);

    let sum = lines_fixed.iter().enumerate().fold(0, |mut acc, (i, l)| {
        match l {
            l if l.is_empty() => {
                acc += curr_chars.len();
                if i != lines_fixed.len() - 1 {
                    curr_chars = lines_fixed[i + 1].chars().collect();
                }
            }

            _ => {
                let mut common_chars: Vec<char> = vec![];
                l.chars().for_each(|c| match c {
                    c if curr_chars.contains(&c) => common_chars.push(c),
                    _ => (),
                });
                curr_chars = common_chars;
            }
        }
        return acc;
    });
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "abc\n\n\

        a\n\
        b\n\
        c\n\n\

        ab\n\
        ac\n\n\

        a\n\
        a\n\
        a\n\
        a\n\n\

        b\n";

    #[test]
    fn day6_part_1_test_case_1() {
        println!("\n\n********************\n\n");
        part1(TEST_CASE_1.to_owned());
        println!("\n\n********************\n\n");
    }

    #[test]
    fn day6_part_2_test_case_1() {
        println!("\n\n********************\n\n");
        part2(TEST_CASE_1.to_owned());
        println!("\n\n********************\n\n");
    }
}
