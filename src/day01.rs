fn get_nums(inp: String) -> Vec<i32> {
    let mut split: Vec<&str> = inp.split("\n").collect();
    split.remove(split.len() - 1);
    return split.iter().map(|&x| x.parse::<i32>().unwrap()).collect();
}

pub fn part1(inp: String) {
    let nums: Vec<i32> = get_nums(inp);

    'outer: for (i, x) in nums.iter().enumerate() {
        for (_, y) in nums[i..nums.len()].iter().enumerate() {
            if x + y == 2020 {
                println!("{}", x * y);
                break 'outer;
            }
        }
    }
}

pub fn part2(inp: String) {
    let nums: Vec<i32> = get_nums(inp);

    'outer: for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums[i..nums.len()].iter().enumerate() {
            if x + y > 2020 {
                continue;
            }
            for (_, z) in nums[j..nums.len()].iter().enumerate() {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    break 'outer;
                }
            }
        }
    }
}
