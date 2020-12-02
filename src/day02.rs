struct Input {
    min: usize,
    max: usize,
    rule: String,
    password: String,
}

fn parse_input(inp: String) -> Vec<Input> {
    let lines: Vec<&str> = inp.lines().collect();
    return lines
        .iter()
        .map(|&x| {
            let v: Vec<&str> = x.split("-").collect();
            let v2: Vec<&str> = v[1].splitn(2, " ").collect();
            let v3: Vec<&str> = v2[1].split(": ").collect();
            return Input {
                min: v[0].parse::<usize>().unwrap(),
                max: v2[0].parse::<usize>().unwrap(),
                rule: v3[0].to_string(),
                password: v3[1].to_string(),
            };
        })
        .collect();
}

pub fn part1(input: String) {
    let inputs: Vec<Input> = parse_input(input);
    let mut valid: i32 = 0;
    for i in inputs {
        let v: Vec<&str> = i.password.matches(&*i.rule).collect();
        if v.len() >= i.min && v.len() <= i.max {
            valid += 1;
        }
    }
    println!("{}", valid);
}

fn compare(pos: usize, password: &str, rule: &str) -> bool {
    return password.chars().nth(pos - 1).unwrap() == rule.chars().nth(0).unwrap();
}

pub fn part2(input: String) {
    let inputs: Vec<Input> = parse_input(input);
    let mut valid: i32 = 0;
    for i in inputs {
        if (compare(i.min, &*i.password, &*i.rule) && !compare(i.max, &*i.password, &*i.rule))
            || (!compare(i.min, &*i.password, &*i.rule) && compare(i.max, &*i.password, &*i.rule))
        {
            valid += 1;
        }
    }
    println!("{}", valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_test_case_1() {
        let str = String::from(
            "1-3 a: abcde\n\
         1-3 b: cdefg\n\
         2-9 c: ccccccccc\n",
        );
        println!("\n\n********************\n\n");
        part1(str);
        println!("\n\n********************\n\n");
    }

    #[test]
    fn day2_part2_test_case_1() {
        let str = String::from(
            "1-3 a: abcde\n\
         1-3 b: cdefg\n\
         2-9 c: ccccccccc\n",
        );
        println!("\n\n********************\n\n");
        part2(str);
        println!("\n\n********************\n\n");
    }
}
