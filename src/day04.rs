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
pub fn part2(input: String) {
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
                        ("byr", _) => tracked_req_fields.retain(|f| f != &"byr"),

                        ("iyr", val) if to_num(val) < 2010 => continue,
                        ("iyr", val) if to_num(val) > 2020 => continue,
                        ("iyr", _) => tracked_req_fields.retain(|f| f != &"iyr"),

                        ("eyr", val) if to_num(val) < 2020 => continue,
                        ("eyr", val) if to_num(val) > 2030 => continue,
                        ("eyr", _) => tracked_req_fields.retain(|f| f != &"eyr"),

                        ("hgt", val) => {
                            let (v, unit) = val.split_at(val.len() - 2);
                            if unit != "cm" && unit != "in" {
                                continue;
                            }
                            let val_num: usize = v.parse().unwrap();

                            match unit {
                                "cm" if val_num < 150 => continue,
                                "cm" if val_num > 193 => continue,
                                "cm" => tracked_req_fields.retain(|f| f != &"hgt"),

                                "in" if val_num < 59 => continue,
                                "in" if val_num > 76 => continue,
                                "in" => tracked_req_fields.retain(|f| f != &"hgt"),

                                _ => (),
                            }
                        }

                        ("hcl", val) if val.len() != 7 => continue,
                        ("hcl", val) => {
                            let chars: Vec<char> = val.chars().collect();
                            let valid: bool = chars.iter().enumerate().fold(true, |acc, (i, c)| {
                                if i == 0 {
                                    return acc && c == &'#';
                                }
                                return acc && c.is_alphanumeric();
                            });
                            if valid {
                                tracked_req_fields.retain(|f| f != &"hcl")
                            }
                        }

                        ("ecl", val) => {
                            let colors: Vec<&str> =
                                vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                            if colors.contains(&val) {
                                tracked_req_fields.retain(|f| f != &"ecl")
                            }
                        }
                        ("pid", val) if val.len() != 9 => continue,
                        ("pid", val) => {
                            let chars: Vec<char> = val.chars().collect();
                            let valid: bool =
                                chars.iter().fold(true, |acc, c| acc && c.is_numeric());
                            if valid {
                                tracked_req_fields.retain(|f| f != &"pid")
                            }
                        }

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
