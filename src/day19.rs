use rustc_hash::FxHashMap;

fn create_cases_from_rule(rules: &FxHashMap<usize, String>, idx: usize) -> Vec<String> {
    let rule = rules.get(&idx).unwrap();
    match &rule[..] {
        "\"a\"" => return vec!["a".to_owned()],
        "\"b\"" => return vec!["b".to_owned()],
        _ => (),
    };

    let mut cases: Vec<String> = vec![];
    let parts = rule.split_whitespace();
    let mut buffer: Vec<String> = vec![];

    for part in parts {
        match part {
            "|" => {
                cases.append(&mut buffer);
                buffer = vec![];
            }
            x => {
                let sub_cases = create_cases_from_rule(rules, x.parse::<usize>().unwrap());
                if buffer.len() == 0 {
                    buffer = sub_cases;
                    continue;
                }

                let mut sub_case_buffer = vec![];
                for elem in buffer.iter() {
                    sub_case_buffer.append(&mut sub_cases.iter().fold(vec![], |mut acc, s| {
                        let mut temp = "".to_owned();
                        temp.push_str(elem);
                        temp.push_str(s);
                        acc.push(temp);
                        acc
                    }));
                }
                buffer = sub_case_buffer;
            }
        };
    }
    cases.append(&mut buffer);
    cases
}

fn parse_rules(v: Vec<String>) -> FxHashMap<usize, String> {
    let mut h: FxHashMap<usize, String> = FxHashMap::default();
    v.iter().for_each(|x| {
        let v = x.splitn(2, ": ").collect::<Vec<&str>>();
        let key = v.get(0).unwrap().parse::<usize>().unwrap();
        let value = v.get(1).unwrap().to_string();
        h.insert(key, value);
    });
    h
}

fn parse(s: String) -> (FxHashMap<usize, String>, Vec<String>) {
    let lines = s
        .lines()
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    let mut iter = lines.split(|x| x.is_empty());

    let rules = iter.next().unwrap().to_vec();
    let cases = iter.next().unwrap().to_vec();
    (parse_rules(rules), cases)
}

pub fn part1(input: String) {
    let (rules, cases) = parse(input);
    let forty_two = create_cases_from_rule(&rules, 42);
    let thirty_one = create_cases_from_rule(&rules, 31);
    let mut acc = 0;
    for c in cases {
        if c.len() != 24 {
            continue;
        }
        let valid_42: bool = (0..2)
            .collect::<Vec<usize>>()
            .iter()
            .all(|i| forty_two.contains(&c[(i * 8)..((i + 1) * 8)].to_owned()));

        let valid_31 = thirty_one.contains(&c[(2 * 8)..].to_owned());

        if valid_42 && valid_31 {
            acc += 1;
        }
    }
    println!("{}", acc);
}

pub fn part2(input: String) {
    let (rules, cases) = parse(input);
    let forty_two = create_cases_from_rule(&rules, 42);
    let thirty_one = create_cases_from_rule(&rules, 31);
    let mut acc = 0;
    fn chunk_range(i: usize) -> std::ops::Range<usize> {
        (i * 8)..((i + 1) * 8)
    }
    for c in cases {
        let num_chunks = c.len() / 8;
        let num_42 = match num_chunks % 2 == 0 {
            true => num_chunks - (num_chunks / 2 - 1),
            false => num_chunks - ((num_chunks - 1) / 2),
        };

        let num_both = num_chunks - num_42 - 1;

        let valid_42: bool = (0..num_42)
            .collect::<Vec<usize>>()
            .iter()
            .all(|&i| forty_two.contains(&c[chunk_range(i)].to_owned()));

        let valid_both = match num_both > 0 {
            true => (num_42..(num_both + num_42))
                .collect::<Vec<usize>>()
                .iter()
                .all(|&i| {
                    forty_two.contains(&c[chunk_range(i)].to_owned())
                        || thirty_one.contains(&c[chunk_range(i)].to_owned())
                }),
            false => true,
        };

        let valid_31 = thirty_one.contains(&c[chunk_range(num_chunks - 1)].to_owned());
        if valid_42 && valid_both && valid_31 {
            acc += 1;
        }
    }
    println!("{}", acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "0: 4 1 5\n\
        1: 2 3 | 3 2\n\
        2: 4 4 | 5 5\n\
        3: 4 5 | 5 4\n\
        4: \"a\"\n\
        5: \"b\"\n\n\
        ababbb\n\
        bababa\n\
        abbbab\n\
        aaabbb\n\
        aaaabbb\n";

    #[test]
    fn day19_ts1() {
        part1(TEST_CASE_1.to_owned());
    }
}
