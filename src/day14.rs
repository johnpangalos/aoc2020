// use rustc_hash::FxHashMap;
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

struct QuickMap<T, U> {
    keys: Vec<T>,
    vals: Vec<U>,
}

impl<T, U> QuickMap<T, U> {
    fn new() -> QuickMap<T, U> {
        let k: Vec<T> = vec![];
        let v: Vec<U> = vec![];
        QuickMap { keys: k, vals: v }
    }

    fn insert(&mut self, key: T, value: U)
    where
        T: Eq + Copy,
    {
        match self.keys.iter().enumerate().find(|(_, &x)| x == key) {
            Some((idx, _)) => {
                self.vals[idx] = value;
            }
            None => {
                self.keys.push(key);
                self.vals.push(value);
            }
        }
    }

    // fn get(&self, key: T) -> Option<&U>
    // where
    // T: Eq + Copy,
    // {
    // let (idx, _) = match self.keys.iter().enumerate().find(|(_, &x)| x == key) {
    // Some(val) => val,
    // None => return None,
    // };

    // self.vals.get(idx)
    // }
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

pub fn part1(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let mut curr_mask = "";
    let mut h: QuickMap<usize, Num36> = QuickMap::new();

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
    println!("{}", h.vals.iter().sum::<u64>());
}
