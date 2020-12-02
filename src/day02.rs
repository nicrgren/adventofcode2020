//! --- Day 2: Password Philosophy ---
//!
//! Your flight departs in a few days from the coastal airport;
//! the easiest way down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
//! "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted:
//! some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy
//! that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input)
//! of passwords (according to the corrupted database) and the corporate policy
//! when that password was set.
//!
//! For example, suppose you have the following list:
//!
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//!
//! Each line gives the password policy and then the password.
//! The password policy indicates the lowest and highest number of times a given letter
//! must appear for the password to be valid. For example, 1-3 a means that the password
//! must contain a at least 1 time and at most 3 times.
//!
//! In the above example,
//! 2 passwords are valid.
//! The middle password, cdefg, is not; it contains no instances of b, but needs at least 1.
//! The first and third passwords are valid: they contain one a or nine c,
//! both within the limits of their respective policies.
//!
//! How many passwords are valid according to their policies?
//!

//! --- Part Two ---

//! While it appears you validated the passwords correctly, they don't seem to
//! be what the Official Toboggan Corporate Authentication System is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained the
//! password policy rules from his old job at the sled rental place down the street!
//! The Official Toboggan Corporate Policy actually works a little differently.
//!
//! Each policy actually describes two positions in the password, where 1 means
//! the first character, 2 means the second character, and so on.
//! (Be careful; Toboggan Corporate Policies have no concept of "index zero"!)
//! Exactly one of these positions must contain the given letter.
//! Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//!     1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//!     1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//!     2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//!
//! How many passwords are valid according to the new interpretation of the policies?

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day02.txt")?;
    println!("Day02 part1: {}", solve_part1(&input));
    println!("Day02 part2: {}", solve_part2(&input));

    Ok(())
}

fn solve_part1(s: &str) -> usize {
    parse(s)
        .into_iter()
        .filter(|(pol, pw)| {
            let count = pw.chars().filter(|&c| c == pol.c).count();
            pol.min <= count && count <= pol.max
        })
        .count()
}

fn solve_part2(s: &str) -> usize {
    parse(s)
        .into_iter()
        .filter(|(pol, pw)| {
            let count = pw
                .chars()
                .enumerate()
                .map(|(i, c)| (i + 1, c))
                .filter(|&(_, c)| c == pol.c)
                .filter(|&(i, _)| i == pol.min || i == pol.max)
                .count();
            count == 1
        })
        .count()
}

fn parse(s: &str) -> impl IntoIterator<Item = (Policy, &'_ str)> {
    s.trim().lines().map(|s| s.trim()).map(|s| {
        let mut split: std::str::Split<'_, _> = s.split(':');
        let policy = split.next().map(Policy::parse).expect("policy part");
        let password = split.next().map(|s| s.trim()).expect("Password part");
        (policy, password)
    })
}

struct Policy {
    min: usize,
    max: usize,
    c: char,
}

impl Policy {
    fn parse(s: &str) -> Self {
        let mut split = s.split(' ');
        let minmax = split.next().expect("policy minmax");
        let mut minmax_split = minmax
            .split('-')
            .map(|s| s.parse::<usize>().expect("policy num"));

        let min = minmax_split.next().expect("policy min");
        let max = minmax_split.next().expect("policy max");
        let c = s.chars().last().expect("policy char");

        Self { min, max, c }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXAMPLE_INPUT: &str = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 1);
    }
}
