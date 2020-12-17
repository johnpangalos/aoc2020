use rustc_hash::FxHashMap;

pub fn part1(input: String) {
    let lines = input
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut sections = lines.split(|x| x.is_empty());

    let mut rules: Vec<usize> = vec![];
    for rule in sections.next().unwrap() {
        fn get_usize(i: Option<&str>) -> usize {
            i.unwrap().parse::<usize>().unwrap()
        }
        let val_str = rule.split(':').nth(1).unwrap();
        let mut iter = val_str.split(|x| x == ' ' || x == '-');
        rules.append(&mut (get_usize(iter.nth(1))..=get_usize(iter.next())).collect());
        rules.append(&mut (get_usize(iter.nth(1))..=get_usize(iter.next())).collect());
    }

    let mut nearby_tickets: Vec<usize> = vec![];
    for (i, ticket) in sections.nth(1).unwrap().iter().enumerate() {
        if i == 0 {
            continue;
        }
        nearby_tickets.append(
            &mut ticket
                .split(',')
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    let mut acc = 0;
    for i in nearby_tickets {
        if !rules.contains(&i) {
            acc += i;
        }
    }
    println!("{}", acc);
}

pub fn part2(input: String) {
    let lines = input
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut sections = lines.split(|x| x.is_empty());

    let mut rule_map: FxHashMap<String, Vec<usize>> = FxHashMap::default();
    for rule in sections.next().unwrap() {
        fn get_usize(i: Option<&str>) -> usize {
            i.unwrap().parse::<usize>().unwrap()
        }
        let mut iter = rule.split(':');
        let name = iter.next().unwrap();
        let mut r = iter.next().unwrap().split(|x| x == ' ' || x == '-');

        let mut vals: Vec<usize> = (get_usize(r.nth(1))..=get_usize(r.next())).collect();
        vals.append(&mut (get_usize(r.nth(1))..=get_usize(r.next())).collect());
        vals.dedup();
        rule_map.insert(name.to_string(), vals);
    }

    let mut rules: Vec<&usize> = rule_map.values().flatten().collect();
    rules.dedup();
    let mut our_ticket: Vec<usize> = vec![];
    for (i, ticket) in sections.next().unwrap().iter().enumerate() {
        if i == 0 {
            continue;
        }
        our_ticket = ticket
            .split(',')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    }

    let mut nearby_tickets: Vec<Vec<usize>> = vec![];

    for (i, ticket) in sections.next().unwrap().iter().enumerate() {
        if i == 0 {
            continue;
        }
        let ticket_vals = ticket
            .split(',')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        match ticket_vals.iter().all(|x| rules.contains(&x)) {
            true => nearby_tickets.push(ticket_vals),
            false => (),
        }
    }

    let mut order_map_dup: FxHashMap<usize, Vec<String>> = FxHashMap::default();

    for i in 0..nearby_tickets[0].len() {
        let temp: Vec<String> = rule_map
            .iter()
            .filter_map(|(name, value)| {
                match nearby_tickets
                    .iter()
                    .all(|ticket| value.contains(&ticket[i]))
                {
                    true => Some(name.to_string()),
                    false => None,
                }
            })
            .collect();
        order_map_dup.insert(i, temp);
    }

    let mut order_map: FxHashMap<String, usize> = FxHashMap::default();
    for (k, v) in order_map_dup.iter() {
        if v.len() == 1 {
            order_map.insert(v[0].to_string(), *k);
            continue;
        }

        let compare = order_map_dup
            .values()
            .find(|compare| v.len() as isize - compare.len() as isize == 1)
            .unwrap();

        let name = v.iter().find(|x| !compare.contains(x)).unwrap();
        order_map.insert(name.to_string(), *k);
    }

    let mut res = 1;
    order_map.iter().for_each(|(k, v)| {
        if !k.contains("departure") {
            return;
        }
        res *= our_ticket[*v];
    });
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "class: 1-3 or 5-7\n\
        row: 6-11 or 33-44\n\
        seat: 13-40 or 45-50\n\n\
        your ticket:\n\
        7,1,14\n\n\
        nearby tickets:\n\
        7,3,47\n\
        40,4,50\n\
        55,2,20\n\
        38,6,12\n";

    const TEST_CASE_2: &str = "class: 0-1 or 4-19\n\
        row: 0-5 or 8-19\n\
        seat: 0-13 or 16-19\n\n\
        your ticket:\n\
        11,12,13\n\n\
        nearby tickets:\n\
        3,9,18\n\
        1,2,30\n\
        15,1,5\n\
        5,14,9\n";

    #[test]
    fn day16_ts1() {
        part1(TEST_CASE_1.to_string());
    }

    #[test]
    fn day16_ts2() {
        part2(TEST_CASE_2.to_string());
    }
}
