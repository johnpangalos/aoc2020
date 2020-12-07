use std::collections::HashMap;

const CONTAINS: &str = " contain ";
const EMPTY_BAG: &str = "no other bags";
const BAGS: &str = "bags";
const BAG: &str = "bag";

fn get_parents(h: &mut HashMap<String, Vec<String>>, bag: String) -> Vec<String> {
    let bag_parents = match h.get(&bag) {
        Some(vector) => vector,
        None => return vec![],
    };
    let mut vec_parents = bag_parents.to_vec();
    for b in bag_parents {
        let v = get_parents(&mut h.clone(), b.to_string());
        for item in v {
            match vec_parents.contains(&item) {
                true => (),
                false => vec_parents.push(item),
            }
        }
    }
    vec_parents
}

pub fn part1(input: String) {
    let mut h: HashMap<String, Vec<String>> = HashMap::new();

    for l in input.lines().collect::<Vec<_>>().iter() {
        let s_clean = l.replace(BAGS, "").replace(BAG, "").replace(".", "");
        let v = s_clean.split(CONTAINS).collect::<Vec<_>>();

        if v[1].contains(EMPTY_BAG) {
            continue;
        }

        let contents = v[1].split(',').collect::<Vec<_>>();

        for c in contents.iter() {
            let item = c.trim().splitn(2, " ").collect::<Vec<_>>();

            match h.contains_key(item[1]) {
                true => {
                    let parents = h.get_mut(item[1]).unwrap();
                    parents.push(v[0].trim().to_string());
                }
                false => {
                    h.insert(item[1].to_string(), vec![v[0].trim().to_string()]);
                }
            };
        }
    }

    println!("{}", get_parents(&mut h, "shiny gold".to_string()).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn day7_part_1_test_case_1() {
        println!("\n\n********************\n\n");
        part1(TEST_CASE_1.to_owned());
        println!("\n\n********************\n\n");
    }
}
