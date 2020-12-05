pub fn part1(input: String) {
    let mut lines: Vec<&str> = input.lines().collect();

    // lines removes the last empty space line, this adds it back in
    lines.push("");

    let req_fields: Vec<&str> = vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    let mut tracked_req_fields = req_fields.clone();
    let mut valid_passports = 0;
    for l in lines {
        match l {
            "" => {
                if tracked_req_fields.len() == 0 {
                    valid_passports += 1;
                }
                tracked_req_fields = req_fields.clone();
            }
            _ => {
                let key_vals: Vec<&str> = l.split(|c| (c == ' ') || (c == ':')).collect();
                for (i, kv) in key_vals.iter().enumerate() {
                    if i % 2 == 0 {
                        tracked_req_fields.retain(|f| f != kv);
                    }
                }
            }
        }
    }

    println!("{}", valid_passports);
}

fn to_num(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn get_unit(s: &str) -> &str {
    &s.split_at(s.len() - 2).1
}

fn is_inches(s: &str) -> bool {
    get_unit(s) == "in"
}

fn is_cm(s: &str) -> bool {
    get_unit(s) == "cm"
}

fn get_val_num(s: &str) -> usize {
    s.split_at(s.len() - 2).0.parse().unwrap()
}

fn is_valid_eye_color(s: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)
}

fn is_valid_hair_color(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    chars.iter().enumerate().fold(true, |acc, (i, c)| {
        if i == 0 {
            return acc && c == &'#';
        }
        return acc && c.is_alphanumeric();
    })
}

fn is_valid_passport_id(s: &str) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .iter()
        .fold(true, |acc, c| acc && c.is_numeric())
}

pub fn part2(input: String) {
    let mut lines: Vec<&str> = input.lines().collect();

    // lines removes the last empty space line, this adds it back in
    lines.push("");

    let req_fields: Vec<&str> = vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    let mut curr_fields = req_fields.clone();
    let mut valid_passports = 0;
    for l in lines {
        match l {
            "" => {
                if curr_fields.len() == 0 {
                    valid_passports += 1;
                }
                curr_fields = req_fields.clone();
            }
            _ => {
                let key_vals: Vec<&str> = l.split(|c| (c == ' ') || (c == ':')).collect();
                let kv_tuples: Vec<(&str, &str)> =
                    key_vals.iter().enumerate().fold(vec![], |mut acc, (i, s)| {
                        if i % 2 == 1 {
                            return acc;
                        }
                        acc.push((s, key_vals[i + 1]));
                        return acc;
                    });

                for kv in kv_tuples.iter() {
                    match *kv {
                        ("byr", val) if to_num(val) < 1920 => continue,
                        ("byr", val) if to_num(val) > 2002 => continue,

                        ("iyr", val) if to_num(val) < 2010 => continue,
                        ("iyr", val) if to_num(val) > 2020 => continue,

                        ("eyr", val) if to_num(val) < 2020 => continue,
                        ("eyr", val) if to_num(val) > 2030 => continue,

                        ("hgt", val) if !vec!["cm", "in"].contains(&get_unit(val)) => continue,
                        ("hgt", val) if is_cm(val) && get_val_num(val) < 150 => continue,
                        ("hgt", val) if is_cm(val) && get_val_num(val) > 193 => continue,
                        ("hgt", val) if is_inches(val) && get_val_num(val) < 59 => continue,
                        ("hgt", val) if is_inches(val) && get_val_num(val) > 76 => continue,

                        ("hcl", val) if val.len() != 7 => continue,
                        ("hcl", val) if !is_valid_hair_color(val) => continue,

                        ("ecl", val) if !is_valid_eye_color(val) => continue,

                        ("pid", val) if val.len() != 9 => continue,
                        ("pid", val) if !is_valid_passport_id(val) => continue,

                        (key, _) if req_fields.contains(&key) => curr_fields.retain(|f| f != &key),
                        _ => (),
                    }
                }
            }
        }
    }

    println!("{}", valid_passports);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
        byr:1937 iyr:2017 cid:147 hgt:183cm\n\n\

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
        hcl:#cfa07d byr:1929\n\n\

        hcl:#ae17e1 iyr:2013\n\
        eyr:2024\n\
        ecl:brn pid:760753108 byr:1931\n\
        hgt:179cm\n\n\

        hcl:#cfa07d eyr:2025 pid:166559648\n\
        iyr:2011 ecl:brn hgt:59in\n\n";

    const TEST_CASE_2: &str = "eyr:1972 cid:100\n\
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\n\

        iyr:2019\n\
        hcl:#602927 eyr:1967 hgt:170cm\n\
        ecl:grn pid:012533040 byr:1946\n\n\

        hcl:dab227 iyr:2012\n\
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\n\

        hgt:59cm ecl:zzz\n\
        eyr:2038 hcl:74454a iyr:2023\n\
        pid:3556412378 byr:2007\n\n";

    const TEST_CASE_3: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
        hcl:#623a2f\n\n\

        eyr:2029 ecl:blu cid:129 byr:1989\n\
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\n\

        hcl:#888785\n\
        hgt:164cm byr:2001 iyr:2015 cid:88\n\
        pid:545766238 ecl:hzl\n\
        eyr:2022\n\n\

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n\n";

    #[test]
    fn day4_part1_test_case_1() {
        part1(TEST_STR.to_owned());
    }

    #[test]
    fn day4_part2_test_case_1() {
        println!("\n\n********************\n\n");
        println!("Four invalid cases - should return 0");
        part2(TEST_CASE_2.to_owned());
        println!("\n\n********************\n\n");
    }

    #[test]
    fn day4_part2_test_case_2() {
        println!("\n\n********************\n\n");
        println!("Four valid cases - should return 4");
        part2(TEST_CASE_3.to_owned());
        println!("\n\n********************\n\n");
    }
}
