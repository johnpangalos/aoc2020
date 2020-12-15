use rustc_hash::FxHashMap;

pub fn part1(input: String) {
    let nums: Vec<usize> = input
        .split(",")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let mut turn = 1;
    let mut last_spoken = 0;
    let mut h: FxHashMap<usize, Vec<usize>> = FxHashMap::default();

    while turn != 2021 {
        if turn <= nums.len() {
            let start_num = *nums.get(turn - 1).unwrap();
            h.insert(start_num, vec![turn]);
            last_spoken = start_num;
        } else {
            let next = match h.get(&last_spoken) {
                None => 0,
                Some(v) if v.len() == 1 => 0,
                Some(v) => v.get(v.len() - 1).unwrap() - v.get(v.len() - 2).unwrap(),
            };
            match h.get(&next) {
                None => {
                    h.insert(next, vec![turn]);
                }
                Some(val) => {
                    let mut temp = val.clone();
                    temp.push(turn);
                    h.insert(next, temp);
                }
            }
            last_spoken = next;
        }

        turn += 1;
    }

    println!("{}", last_spoken);
}

pub fn part2(input: String) {
    let nums: Vec<usize> = input
        .split(",")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let mut turn = 1;
    let mut last_spoken = 0;
    let mut h: FxHashMap<usize, Vec<usize>> = FxHashMap::default();

    while turn != 30000001 {
        if turn <= nums.len() {
            let start_num = *nums.get(turn - 1).unwrap();
            h.insert(start_num, vec![turn]);
            last_spoken = start_num;
        } else {
            let next = match h.get(&last_spoken) {
                None => 0,
                Some(v) if v.len() == 1 => 0,
                Some(v) => v.get(v.len() - 1).unwrap() - v.get(v.len() - 2).unwrap(),
            };
            match h.get(&next) {
                None => {
                    h.insert(next, vec![turn]);
                }
                Some(val) => {
                    let mut temp = val.clone();
                    temp.push(turn);
                    h.insert(next, temp);
                }
            }
            last_spoken = next;
        }

        turn += 1;
        if turn % 100000 == 0 {
            println!("Turn: {}", turn);
        }
    }

    println!("{}", last_spoken);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_ts1() {
        part1("0,3,6".to_owned());
    }

    #[test]
    fn day15_ts2() {
        part1("1,3,2".to_owned());
    }

    #[test]
    fn day15_ts3() {
        part2("1,3,2".to_owned());
    }
}
