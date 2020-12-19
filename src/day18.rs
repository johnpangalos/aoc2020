type Stack = Vec<String>;

fn new_stack(expr: String) -> Stack {
    let mut stack: Stack = vec![];

    for c in expr.chars() {
        match c {
            ' ' => (),
            x => {
                stack.push(x.to_string());
            }
        }
    }
    stack
}

fn eval(stack: Stack) -> isize {
    let mut acc: isize = -1;
    let mut curr_oper = "";
    let mut skip: Vec<usize> = vec![];
    let open_parenthesis = "(".to_string();
    let operations = ["*".to_string(), "+".to_string()];
    let close_parenthesis = ")".to_string();

    for (idx, s) in stack.iter().enumerate() {
        if skip.contains(&idx) {
            continue;
        }
        let next = match s {
            c if *c == open_parenthesis => {
                let mut extra_parenthesis = -1;
                let end = stack[idx..]
                    .iter()
                    .position(|x| {
                        if *x == open_parenthesis {
                            extra_parenthesis += 1;
                            return false;
                        }

                        if *x == close_parenthesis && extra_parenthesis > 0 {
                            extra_parenthesis -= 1;
                            return false;
                        }
                        *x == close_parenthesis
                    })
                    .unwrap();
                skip.append(&mut (idx..=(idx + end)).collect());
                eval(stack[(idx + 1)..(idx + end)].to_vec())
            }
            c if *c == close_parenthesis => continue,
            c if operations.contains(c) => {
                curr_oper = c;
                continue;
            }
            num => num.parse::<isize>().unwrap(),
        };
        match curr_oper {
            "" => acc = next,
            "+" => acc += next,
            "*" => acc *= next,
            _ => panic!("received operator {}", curr_oper),
        }
    }
    acc
}

fn eval_precedence(stack: Stack) -> isize {
    let mut to_mult: Vec<isize> = vec![];

    let mut curr_oper = "";
    let mut skip: Vec<usize> = vec![];
    let open_parenthesis = "(".to_string();
    let operations = ["*".to_string(), "+".to_string()];
    let close_parenthesis = ")".to_string();

    for (idx, s) in stack.iter().enumerate() {
        if skip.contains(&idx) {
            continue;
        }
        let next = match s {
            c if *c == open_parenthesis => {
                let mut extra_parenthesis = -1;
                let end = stack[idx..]
                    .iter()
                    .position(|x| {
                        if *x == open_parenthesis {
                            extra_parenthesis += 1;
                            return false;
                        }

                        if *x == close_parenthesis && extra_parenthesis > 0 {
                            extra_parenthesis -= 1;
                            return false;
                        }
                        *x == close_parenthesis
                    })
                    .unwrap();
                skip.append(&mut (idx..=(idx + end)).collect());
                eval_precedence(stack[(idx + 1)..(idx + end)].to_vec())
            }
            c if *c == close_parenthesis => continue,
            c if operations.contains(c) => {
                curr_oper = c;
                continue;
            }
            num => num.parse::<isize>().unwrap(),
        };

        let len = to_mult.len();
        match curr_oper {
            "" => {
                to_mult.push(next);
            }
            "+" => {
                to_mult[len - 1] += next;
            }
            "*" => {
                to_mult.push(next);
            }
            _ => panic!("received operator {}", curr_oper),
        }
    }
    to_mult.iter().product()
}

pub fn part1(input: String) {
    println!(
        "{}",
        input
            .lines()
            .fold(0, |acc, x| { acc + eval(new_stack(x.to_string())) })
    );
}

pub fn part2(input: String) {
    println!(
        "{}",
        input.lines().fold(0, |acc, x| {
            acc + eval_precedence(new_stack(x.to_string()))
        })
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_ts1() {
        assert_eq!(eval(new_stack("1 + 2 * 3 + 4 * 5 + 6".to_owned())), 71);
    }

    #[test]
    fn day18_ts2() {
        assert_eq!(
            eval(new_stack("1 + (2 * 3) + (4 * (5 + 6))".to_owned())),
            51
        );
    }

    #[test]
    fn day18_ts3() {
        assert_eq!(eval(new_stack("2 * 3 + (4 * 5)".to_owned())), 26);
    }

    #[test]
    fn day18_ts4() {
        assert_eq!(
            eval(new_stack("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned())),
            437
        );
    }

    #[test]
    fn day18_ts5() {
        assert_eq!(
            eval(new_stack(
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned()
            )),
            12240
        );
    }

    #[test]
    fn day18_ts6() {
        assert_eq!(
            eval(new_stack(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned()
            )),
            13632
        );
    }

    #[test]
    fn day18_ts7() {
        assert_eq!(
            eval_precedence(new_stack("1 + 2 * 3 + 4 * 5 + 6".to_owned())),
            231
        );
    }

    #[test]
    fn day18_ts8() {
        assert_eq!(
            eval_precedence(new_stack("1 + (2 * 3) + (4 * (5 + 6))".to_owned())),
            51
        );
    }

    #[test]
    fn day18_ts9() {
        assert_eq!(eval_precedence(new_stack("2 * 3 + (4 * 5)".to_owned())), 46);
    }

    #[test]
    fn day18_ts10() {
        assert_eq!(
            eval_precedence(new_stack("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned())),
            1445
        );
    }

    #[test]
    fn day18_ts11() {
        assert_eq!(
            eval_precedence(new_stack(
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned()
            )),
            669060
        );
    }

    #[test]
    fn day18_ts12() {
        assert_eq!(
            eval_precedence(new_stack(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned()
            )),
            23340
        );
    }
}
