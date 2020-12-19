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
        if c.len() == 24
            && forty_two.contains(&c[0..=7].to_owned())
            && forty_two.contains(&c[8..=15].to_owned())
            && thirty_one.contains(&c[16..].to_owned())
        {
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
    for c in cases {
        if c.len() == 24
            && forty_two.contains(&c[0..8].to_owned())
            && forty_two.contains(&c[8..16].to_owned())
            && thirty_one.contains(&c[16..].to_owned())
        {
            acc += 1;
        }
        if c.len() == 32
            && forty_two.contains(&c[0..8].to_owned())
            && forty_two.contains(&c[8..16].to_owned())
            && forty_two.contains(&c[16..24].to_owned())
            && thirty_one.contains(&c[24..].to_owned())
        {
            acc += 1;
        }

        if c.len() == 40
            && forty_two.contains(&c[0..8].to_owned())
            && forty_two.contains(&c[8..16].to_owned())
            && forty_two.contains(&c[16..24].to_owned())
            && (forty_two.contains(&c[24..32].to_owned())
                || thirty_one.contains(&c[24..32].to_owned()))
            && thirty_one.contains(&c[32..].to_owned())
        {
            acc += 1;
        }

        if c.len() == 48
            && forty_two.contains(&c[0..8].to_owned())
            && forty_two.contains(&c[8..16].to_owned())
            && forty_two.contains(&c[16..24].to_owned())
            && forty_two.contains(&c[24..32].to_owned())
            && (forty_two.contains(&c[32..40].to_owned())
                || thirty_one.contains(&c[32..40].to_owned()))
            && thirty_one.contains(&c[40..].to_owned())
        {
            acc += 1;
        }

        if c.len() == 56
            && forty_two.contains(&c[0..8].to_owned())
            && forty_two.contains(&c[8..16].to_owned())
            && forty_two.contains(&c[16..24].to_owned())
            && forty_two.contains(&c[24..32].to_owned())
            && (forty_two.contains(&c[32..40].to_owned())
                || thirty_one.contains(&c[32..40].to_owned()))
            && (forty_two.contains(&c[40..48].to_owned())
                || thirty_one.contains(&c[40..48].to_owned()))
            && thirty_one.contains(&c[48..].to_owned())
        {
            acc += 1;
        }

        if c.len() == 64
            && forty_two.contains(&c[0..8].to_owned())
            && forty_two.contains(&c[8..16].to_owned())
            && forty_two.contains(&c[16..24].to_owned())
            && forty_two.contains(&c[24..32].to_owned())
            && forty_two.contains(&c[32..40].to_owned())
            && (forty_two.contains(&c[40..48].to_owned())
                || thirty_one.contains(&c[40..48].to_owned()))
            && (forty_two.contains(&c[48..56].to_owned())
                || thirty_one.contains(&c[48..56].to_owned()))
            && thirty_one.contains(&c[56..].to_owned())
        {
            acc += 1;
        }

        if c.len() == 72
            && forty_two.contains(&c[0..8].to_owned())
            && forty_two.contains(&c[8..16].to_owned())
            && forty_two.contains(&c[16..24].to_owned())
            && forty_two.contains(&c[24..32].to_owned())
            && forty_two.contains(&c[32..40].to_owned())
            && forty_two.contains(&c[40..48].to_owned())
            && (forty_two.contains(&c[48..56].to_owned())
                || thirty_one.contains(&c[48..56].to_owned()))
            && (forty_two.contains(&c[56..64].to_owned())
                || thirty_one.contains(&c[56..64].to_owned()))
            && thirty_one.contains(&c[64..].to_owned())
        {
            acc += 1;
        }

        if c.len() == 80
            && forty_two.contains(&c[0..8].to_owned())
            && forty_two.contains(&c[8..16].to_owned())
            && forty_two.contains(&c[16..24].to_owned())
            && forty_two.contains(&c[24..32].to_owned())
            && forty_two.contains(&c[32..40].to_owned())
            && forty_two.contains(&c[40..48].to_owned())
            && (forty_two.contains(&c[48..56].to_owned())
                || thirty_one.contains(&c[48..56].to_owned()))
            && (forty_two.contains(&c[56..64].to_owned())
                || thirty_one.contains(&c[56..64].to_owned()))
            && (forty_two.contains(&c[64..72].to_owned())
                || thirty_one.contains(&c[64..72].to_owned()))
            && thirty_one.contains(&c[72..].to_owned())
        {
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
        // let (rules, cases) = parse(TEST_CASE_1.to_owned());
        // println!("rules: {:?}", rules);
        // println!("cases: {:?}", cases);
        // println!("all cases: {:?}", create_cases_from_rule(&rules, 0));
    }
}
