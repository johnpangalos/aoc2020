pub fn part1(inp: String) {
    let mut split: Vec<&str> = inp.split("\n").collect();
    split.remove(split.len() - 1);
    let mut found: bool = false;

    for (i, s) in split.iter().enumerate() {
        if found {
            break;
        }

        let num = s.parse::<i32>().unwrap();
        let mut rest = split.clone();
        rest.remove(i);
        for t in rest {
            let compare = t.parse::<i32>().unwrap();
            if num + compare == 2020 {
                println!("{}", num * compare);
                found = true;
                break;
            }
        }
    }
}

pub fn part2(inp: String) {
    let mut split: Vec<&str> = inp.split("\n").collect();
    split.remove(split.len() - 1);
    let mut found: bool = false;

    for (i, s1) in split.iter().enumerate() {
        if found {
            break;
        }

        let n1 = s1.parse::<i32>().unwrap();
        let mut rest1 = split.clone();
        rest1.remove(i);
        for s2 in rest1 {
            if found {
                break;
            }
            let n2 = s2.parse::<i32>().unwrap();

            let mut rest2 = split.clone();
            rest2.remove(i);
            for s3 in rest2 {
                let n3 = s3.parse::<i32>().unwrap();

                if n1 + n2 + n3 == 2020 {
                    println!("{}", n1 * n2 * n3);
                    found = true;
                    break;
                }
            }
        }
    }
}
