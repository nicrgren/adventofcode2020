//! --- Day 4: Passport Processing ---
//!
//! You arrive at the airport only to realize that you grabbed your North Pole Credentials
//! instead of your passport. While these documents are extremely similar,
//! North Pole Credentials aren't issued by a country and therefore aren't actually
//! valid documentation for travel in most of the world.
//!
//! It seems like you're not the only one having problems, though; a very long line
//! has formed for the automatic passport scanners, and the delay could upset your travel itinerary.
//!
//! Due to some questionable network security, you realize you might be able to solve both
//! of these problems at the same time.
//!
//! The automatic passport scanners are slow because they're having trouble detecting
//! which passports have all required fields. The expected fields are as follows:
//!
//!     byr (Birth Year)
//!     iyr (Issue Year)
//!     eyr (Expiration Year)
//!     hgt (Height)
//!     hcl (Hair Color)
//!     ecl (Eye Color)
//!     pid (Passport ID)
//!     cid (Country ID)
//!
//! Passport data is validated in batch files (your puzzle input). Each passport is represented
//! as a sequence of key:value pairs separated by spaces or newlines. Passports are
//! separated by blank lines.
//!
//! Here is an example batch file containing four passports:
//!
//! ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
//! byr:1937 iyr:2017 cid:147 hgt:183cm
//!
//! iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
//! hcl:#cfa07d byr:1929
//!
//! hcl:#ae17e1 iyr:2013
//! eyr:2024
//! ecl:brn pid:760753108 byr:1931
//! hgt:179cm
//!
//! hcl:#cfa07d eyr:2025 pid:166559648
//! iyr:2011 ecl:brn hgt:59in
//!
//! The first passport is valid - all eight fields are present. The second passport
//! is invalid - it is missing hgt (the Height field).
//!
//! The third passport is interesting; the only missing field is cid, so it looks like data
//! from North Pole Credentials, not a passport at all! Surely, nobody would mind if you
//! made the system temporarily ignore missing cid fields. Treat this "passport" as valid.
//!
//! The fourth passport is missing two fields, cid and byr. Missing cid is fine,
//! but missing any other field is not, so this passport is invalid.
//!
//! According to the above rules, your improved system would report 2 valid passports.
//!
//! Count the number of valid passports - those that have all required fields.
//! Treat cid as optional. In your batch file, how many passports are valid?
//!
//!
//!
//! --- Part Two ---
//!
//! The line is moving more quickly now, but you overhear airport security talking about how
//! passports with invalid data are getting through. Better add some data validation, quick!
//!
//! You can continue to ignore the cid field, but each other field has strict rules about
//! what values are valid for automatic validation:
//!
//!     byr (Birth Year) - four digits; at least 1920 and at most 2002.
//!     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//!     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//!     hgt (Height) - a number followed by either cm or in:
//!         If cm, the number must be at least 150 and at most 193.
//!         If in, the number must be at least 59 and at most 76.
//!     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//!     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//!     pid (Passport ID) - a nine-digit number, including leading zeroes.
//!     cid (Country ID) - ignored, missing or not.
//!
//! Your job is to count the passports where all required fields are both present and
//! valid according to the above rules. Here are some example values:
//!
//! byr valid:   2002
//! byr invalid: 2003
//!
//! hgt valid:   60in
//! hgt valid:   190cm
//! hgt invalid: 190in
//! hgt invalid: 190
//!
//! hcl valid:   #123abc
//! hcl invalid: #123abz
//! hcl invalid: 123abc
//!
//! ecl valid:   brn
//! ecl invalid: wat
//!
//! pid valid:   000000001
//! pid invalid: 0123456789
//!
//! Here are some invalid passports:
//!
//! eyr:1972 cid:100
//! hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
//!
//! iyr:2019
//! hcl:#602927 eyr:1967 hgt:170cm
//! ecl:grn pid:012533040 byr:1946
//!
//! hcl:dab227 iyr:2012
//! ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
//!
//! hgt:59cm ecl:zzz
//! eyr:2038 hcl:74454a iyr:2023
//! pid:3556412378 byr:2007
//!
//! Here are some valid passports:
//!
//! pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
//! hcl:#623a2f
//!
//! eyr:2029 ecl:blu cid:129 byr:1989
//! iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
//!
//! hcl:#888785
//! hgt:164cm byr:2001 iyr:2015 cid:88
//! pid:545766238 ecl:hzl
//! eyr:2022
//!
//! iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
//!
//! Count the number of valid passports - those that have all required fields and valid values.
//! Continue to treat cid as optional. In your batch file, how many passports are valid?

use std::{fmt, str::FromStr};

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day04.txt")?;
    println!("Day04 part1: {}", part1(&input));
    println!("Day04 part2: {}", part2(&input));
    Ok(())
}

fn part1(s: &str) -> usize {
    parse(s)
        .into_iter()
        .filter(|p| p.meets_p1_standards())
        .count()
}

fn part2(s: &str) -> usize {
    let (valid, invalid): (Vec<_>, Vec<_>) =
        parse(s).into_iter().partition(|p| p.meets_p2_standards());

    let arg = std::env::args().nth(2);

    match arg.as_deref() {
        Some("invalid") => {
            invalid.iter().for_each(|pp| {
                println!("{}", pp);
            });

            println!("Total (invalid): {}", invalid.len());
        }

        Some("valid") => {
            valid.iter().for_each(|pp| {
                println!("{}", pp);
            });
            println!("Total (valid): {}", valid.len());
        }

        _ => (),
    }

    valid.len()
}

fn parse(s: &str) -> impl IntoIterator<Item = Passport> {
    s.trim().split("\n\n").map(|seg| {
        let mut p = Passport::default();
        seg.split('\n')
            .map(|s| s.trim().split(' '))
            .flatten()
            .for_each(|pair| {
                let mut split = pair.trim().split(':');
                let key = split.next().expect("Reading key");
                let val = split
                    .next()
                    .ok_or_else(|| println!("Reading val from `{}`", pair))
                    .expect("Reading val");

                match key {
                    "byr" => p.byr = Some(val),
                    "iyr" => p.iyr = Some(val),
                    "eyr" => p.eyr = Some(val),
                    "hgt" => p.hgt = Some(val),
                    "hcl" => p.hcl = Some(val),
                    "ecl" => p.ecl = Some(val),
                    "pid" => p.pid = Some(val),
                    "cid" => p.cid = Some(val),
                    s => panic!("Invalid key `{}`", s),
                }
            });

        p
    })
}

impl fmt::Display for Passport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "byr:{byr:6} iyr:{iyr:6} eyr:{eyr:6} hgt:{hgt:7} hcl:{hcl:8} ecl:{ecl:7} pid:{pid:11} cid:{cid:6}",
            byr = self.byr.unwrap_or("-"),
            iyr = self.iyr.unwrap_or("-"),
            eyr = self.eyr.unwrap_or("-"),
            hgt = self.hgt.unwrap_or("-"),
            hcl = self.hcl.unwrap_or("-"),
            ecl = self.ecl.unwrap_or("-"),
            pid = self.pid.unwrap_or("-"),
            cid = self.cid.unwrap_or("-"),
        )
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
struct Passport<'a> {
    pub byr: Option<&'a str>,
    pub iyr: Option<&'a str>,
    pub eyr: Option<&'a str>,
    pub hgt: Option<&'a str>,
    pub hcl: Option<&'a str>,
    pub ecl: Option<&'a str>,
    pub pid: Option<&'a str>,
    pub cid: Option<&'a str>,
}

impl Passport<'_> {
    fn meets_p1_standards(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn meets_p2_standards(&self) -> bool {
        self.byr
            .and_then(|s| s.parse::<usize>().ok())
            .map(|yr| matches!(yr, 1920..=2002))
            .unwrap_or(false)
            && self
                .iyr
                .and_then(|s| s.parse::<usize>().ok())
                .map(|yr| matches!(yr, 2010..=2020))
                .unwrap_or(false)
            && self
                .eyr
                .and_then(|s| s.parse::<usize>().ok())
                .map(|yr| matches!(yr, 2020..=2030))
                .unwrap_or(false)
            && self
                .hgt
                .map(Height::from_str)
                .map(|res| res.is_ok())
                .unwrap_or(false)
            && self.valid_hcl()
            && self
                .ecl
                .map(|s| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s.trim()))
                .unwrap_or(false)
            && self
                .pid
                .map(|s| s.chars().filter(|c| matches!(c, '0'..='9')).count() == 9)
                .unwrap_or(false)
    }

    fn valid_hcl(&self) -> bool {
        self.hcl
            .map(|s| {
                let mut it = s.chars();
                it.next().expect("hcl #") == '#' && (it.filter(|c| c.is_digit(16)).count() == 6)
            })
            .unwrap_or(false)
    }
}

#[allow(dead_code)]
struct Height {
    value: usize,
    unit: HeightUnit,
}

impl FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let alpha_start = s
            .chars()
            .position(|c| !matches!(c, '0'..='9'))
            .ok_or_else(|| format!("no find end of digits in height str `{}`", s))?;

        let (value_s, unit_s) = s.split_at(alpha_start);

        let value = value_s
            .parse::<usize>()
            .map_err(|_| format!("Invalid height: `{}`", s))?;

        let unit = match unit_s {
            "cm" => {
                if matches!(value, 150..=193) {
                    HeightUnit::Cm
                } else {
                    return Err(format!(
                        "Invalid Height `{}`. Cm must be in [150..193]",
                        value
                    ));
                }
            }
            "in" => {
                if matches!(value, 56..=76) {
                    HeightUnit::In
                } else {
                    return Err(format!(
                        "Invalid Height `{}`. In must be in [150..193]",
                        value
                    ));
                }
            }

            s => return Err(format!("Unknown height unit `{}`", s)),
        };

        Ok(Height { value, unit })
    }
}

enum HeightUnit {
    Cm,
    In,
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        let example = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

        assert_eq!(super::part1(example), 2);
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day04.txt").expect("reading input");
        assert_eq!(super::part1(&input), 170);
    }

    #[test]
    fn part2_example_invalid() {
        let s = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;

        assert_eq!(super::part2(s), 0);
    }

    #[test]
    fn part2_example_valid() {
        let s = r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;

        assert_eq!(super::part2(s), 4);
    }

    #[test]
    fn valid_hcl() {
        let tests = vec!["#15bd97", "#18171d", "#7d3b0c"];

        for t in tests {
            assert!(
                super::Passport {
                    hcl: Some("#c0946f"),
                    ..Default::default()
                }
                .valid_hcl(),
                "{} should be valid hcl",
                t
            );
        }
    }

    #[test]
    fn part2() {
        let input = crate::read_input("day04.txt").expect("reading input");
        assert_eq!(super::part2(&input), 103);
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
//     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//     hgt (Height) - a number followed by either cm or in:
//         If cm, the number must be at least 150 and at most 193.
//         If in, the number must be at least 59 and at most 76.
//     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//     pid (Passport ID) - a nine-digit number, including leading zeroes.
//     cid (Country ID) - ignored, missing or not.
