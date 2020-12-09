use rayon::prelude::*;

const ACC_INST: &str = "acc";
const NOOP_INST: &str = "nop";
const JUMP_INST: &str = "jmp";

fn parse_instruction(s: &str) -> (&str, isize) {
    let v: Vec<_> = s.split_ascii_whitespace().collect();
    (v[0], v[1].to_string().parse::<isize>().unwrap())
}

pub fn part1(input: String) {
    let list: Vec<String> = input
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("{}", int_comp(&list, 1000000000000).0);
}

fn int_comp(list: &Vec<String>, idx: usize) -> (isize, bool) {
    let mut acc: isize = 0;
    let mut curr_inst: isize = 0;
    let mut run_inst: Vec<isize> = vec![];
    loop {
        match run_inst.contains(&curr_inst) || curr_inst as usize == list.len() {
            true => break,
            false => run_inst.push(curr_inst),
        }

        match parse_instruction(&list[curr_inst as usize]) {
            (ACC_INST, val) => acc += val,
            (NOOP_INST, val) if curr_inst as usize == idx => {
                curr_inst += val;
                continue;
            }
            (NOOP_INST, _) => (),
            (JUMP_INST, _) if curr_inst as usize == idx => (),
            (JUMP_INST, val) => {
                curr_inst += val;
                continue;
            }
            _ => (),
        }
        curr_inst += 1;
    }
    (acc, curr_inst as usize == list.len())
}

pub fn part2(input: String) {
    let list: Vec<String> = input
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.to_string())
        .collect();

    let mut v_changed: Vec<usize> = vec![];
    for (i, item) in list.iter().enumerate() {
        if !item.contains(JUMP_INST) && !item.contains(NOOP_INST) {
            continue;
        }
        v_changed.push(i);
    }

    v_changed.par_iter().find_any(|&&idx| {
        let (acc, to_break) = int_comp(&list, idx);
        if to_break {
            println!("{}", acc);
        }
        return to_break;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "nop +0\n\
        acc +1\n\
        jmp +4\n\
        acc +3\n\
        jmp -3\n\
        acc -99\n\
        acc +1\n\
        jmp -4\n\
        acc +6\n";

    #[test]
    fn day8_part1_case_1() {
        println!("\n\n********************\n\n");
        part1(TEST_CASE_1.to_owned());
        println!("\n\n********************\n\n");
    }

    #[test]
    fn day8_part2_case_1() {
        println!("\n\n********************\n\n");
        part2(TEST_CASE_1.to_owned());
        println!("\n\n********************\n\n");
    }
}
