use rustc_hash::FxHashMap;
use std::convert::TryFrom;

type Bit36 = String;

trait CovertToNum {
    fn to_num(&self) -> Num36;
}

impl CovertToNum for Bit36 {
    fn to_num(&self) -> Num36 {
        self.chars()
            .enumerate()
            .fold(Num36::try_from(0).unwrap(), |acc, (idx, val)| {
                let base: u64 = 2;
                acc + base.pow(35 as u32 - idx as u32) * val.to_string().parse::<Num36>().unwrap()
            })
    }
}

type Num36 = u64;

trait CovertToBit {
    fn to_bit(&self) -> Bit36;
}

impl CovertToBit for Num36 {
    fn to_bit(&self) -> Bit36 {
        let mut num = self.clone();
        let mut b: Bit36 = "".to_string();
        let base: u64 = 2;
        for i in 0..36 {
            let curr_bin = base.pow(35 - i);
            match num >= curr_bin {
                true => {
                    num -= curr_bin;
                    b.push('1');
                }
                false => b.push('0'),
            }
        }
        b
    }
}

fn apply_mask(num: Num36, m: &str) -> Num36 {
    let b_chars: Vec<char> = num.to_bit().chars().collect();
    let new_b: Bit36 = m
        .chars()
        .enumerate()
        .fold("".to_string(), |mut acc, (idx, val)| {
            match val {
                'X' => acc.push(*b_chars.get(idx).unwrap()),
                v => acc.push(v),
            }
            acc
        });
    new_b.to_num()
}

fn apply_mask_v2(num: Num36, m: &str) -> Vec<Num36> {
    let mut curr_bins: Vec<String> = vec!["".to_string()];
    let mut curr_buff = "".to_string();
    let b_chars: Vec<char> = num.to_bit().chars().collect();
    for (idx, c) in m.chars().enumerate() {
        match c {
            'X' => {
                let mut temp: Vec<String> = vec![];
                curr_bins.iter().for_each(|x| {
                    let mut x_clone1 = x.clone();
                    let mut x_clone2 = x.clone();
                    x_clone1.push_str(&curr_buff);
                    x_clone2.push_str(&curr_buff);
                    x_clone2.push('1');
                    x_clone1.push('0');
                    temp.push(x_clone1.to_string());
                    temp.push(x_clone2.to_string());
                });
                curr_bins = temp;
                curr_buff = "".to_string();
            }
            '1' => curr_buff.push('1'),
            '0' => curr_buff.push(*b_chars.get(idx).unwrap()),
            _ => panic!("huh??"),
        }
    }
    curr_bins
        .iter()
        .map(|x| {
            let mut temp = x.clone();
            temp.push_str(&curr_buff);
            temp.to_num()
        })
        .collect::<Vec<Num36>>()
}

pub fn part1(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let mut curr_mask = "";
    let mut h: FxHashMap<usize, Num36> = FxHashMap::default();

    for l in lines {
        match l.contains("mask") {
            true => {
                curr_mask = l.split_at(7).1;
                continue;
            }
            false => (),
        }

        let inst_array: Vec<&str> = l.split(|c| c == '[' || c == ']' || c == '=').collect();
        let (addr, val) = (
            inst_array.get(1).unwrap().parse::<usize>().unwrap(),
            inst_array.get(3).unwrap().trim().parse::<Num36>().unwrap(),
        );

        h.insert(addr, apply_mask(val, curr_mask));
    }
    println!("{}", h.values().sum::<u64>());
}

pub fn part2(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let mut curr_mask = "";
    let mut h: FxHashMap<u64, Num36> = FxHashMap::default();

    for l in lines {
        match l.contains("mask") {
            true => {
                curr_mask = l.split_at(7).1;
                continue;
            }
            false => (),
        }
        let inst_array: Vec<&str> = l.split(|c| c == '[' || c == ']' || c == '=').collect();
        let (addr, val) = (
            inst_array.get(1).unwrap().parse::<usize>().unwrap(),
            inst_array.get(3).unwrap().trim().parse::<Num36>().unwrap(),
        );

        for &x in apply_mask_v2(addr as u64, curr_mask).iter() {
            h.insert(x, val);
        }
    }
    println!("{}", h.values().sum::<u64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "mask = 000000000000000000000000000000X1001X\n\
                                mem[42] = 100\n\
                                mask = 00000000000000000000000000000000X0XX\n\
                                mem[26] = 1\n";

    #[test]
    fn day14_part2() {
        part2(TEST_CASE_1.to_string());
    }
}
