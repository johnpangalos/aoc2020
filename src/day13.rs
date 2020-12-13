fn get_next_bus_time(time: isize, bus: &isize) -> isize {
    ((time / bus + 1) * bus) - time
}

pub fn part1(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let (time, buses) = (
        lines[0].parse::<isize>().unwrap(),
        lines[1].split(",").fold(vec![], |mut acc, s| {
            match s {
                "x" => (),
                val => {
                    acc.push(val.parse::<isize>().unwrap());
                }
            }
            acc
        }),
    );
    let min = buses
        .iter()
        .min_by(|&x, &y| get_next_bus_time(time, x).cmp(&get_next_bus_time(time, y)))
        .unwrap();
    println!("{}", min * get_next_bus_time(time, min));
}

pub fn part2(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let (_time, buses) = (
        lines[0].parse::<isize>().unwrap(),
        lines[1].split(",").fold(vec![], |mut acc, s| {
            match s {
                "x" => acc.push(0),
                val => acc.push(val.parse::<isize>().unwrap()),
            }
            acc
        }),
    );

    let m: isize = buses.iter().fold(1, |acc, &x| {
        if x == 0 {
            return acc;
        }
        acc * x
    });
    let ys: Vec<isize> = buses
        .iter()
        .map(|&x| {
            if x == 0 {
                return 0;
            }
            let mut temp = 1;
            loop {
                if ((m / x * temp) - 1) % x == 0 {
                    return temp;
                }

                temp += 1;
            }
        })
        .collect();

    let mod2 = buses.iter().enumerate().fold(0, |acc, (i, &val)| {
        if val == 0 {
            return acc;
        }
        acc + (val - i as isize) * m / val * ys[i]
    });

    println!("{}", mod2 % m);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "939\n\
        7,13,x,x,59,x,31,19\n";

    const TEST_CASE_2: &str = "1\n\
                               1789,37,47,1889\n";

    const TEST_CASE_3: &str = "1\n\
                               67,7,x,59,61\n";

    const TEST_CASE_4: &str = "1\n\
                               67,x,7,59,61\n";

    const TEST_CASE_5: &str = "1\n\
                               67,7,59,61\n";

    const TEST_CASE_6: &str = "1\n\
                               x,7,3,5,2\n";
    #[test]
    fn day13_part1() {
        part1(TEST_CASE_1.to_string());
    }

    #[test]
    fn day13_part2_ts1() {
        part2(TEST_CASE_1.to_string());
    }

    #[test]
    fn day13_part2_ts2() {
        part2(TEST_CASE_2.to_string());
    }

    #[test]
    fn day13_part2_ts3() {
        part2(TEST_CASE_3.to_string());
    }

    #[test]
    fn day13_part2_ts4() {
        part2(TEST_CASE_4.to_string());
    }

    #[test]
    fn day13_part2_ts5() {
        part2(TEST_CASE_5.to_string());
    }

    #[test]
    fn day13_part2_ts6() {
        part2(TEST_CASE_6.to_string());
    }
}
