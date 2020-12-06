//! --- Day 6: Custom Customs ---
//!
//! As your flight approaches the regional airport where you'll switch to a
//! much larger plane, customs declaration forms are distributed to the passengers.
//!
//! The form asks a series of 26 yes-or-no questions marked a through z.
//! All you need to do is identify the questions for which anyone in your
//! group answers "yes". Since your group is just you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a
//! language barrier and asks if you can help. For each of the people in their group,
//! you write down the questions for which they answer "yes", one per line. For example:
//!
//! abcx
//! abcy
//! abcz
//!
//! In this group, there are 6 questions to which anyone answered
//! "yes": a, b, c, x, y, and z.
//! (Duplicate answers to the same question don't count extra; each question counts at most once.)
//!
//! Another group asks for your help, then another, and eventually you've collected answers
//! from every group on the plane (your puzzle input). Each group's answers are
//! separated by a blank line, and within each group, each person's answers are on a single line.
//! For example:
//!
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//!
//! This list represents answers from five groups:
//!
//!     The first group contains one person who answered "yes"
//!     to 3 questions: a, b, and c.
//!
//!     The second group contains three people; combined, they answered "yes"
//!     to 3 questions: a, b, and c.
//!
//!     The third group contains two people; combined, they answered "yes"
//!     to 3 questions: a, b, and c.
//!
//!     The fourth group contains four people; combined, they answered "yes"
//!     to only 1 question, a.
//!
//!     The last group contains one person who answered "yes"
//!     to only 1 question, b.
//!
//! In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
//!
//! For each group, count the number of questions to which anyone answered "yes".
//! What is the sum of those counts?
//!
//!
//!
//! --- Part Two ---
//!
//! As you finish the last group's customs declaration, you notice that
//! you misread one word in the instructions:
//!
//! You don't need to identify the questions to which anyone answered "yes";
//! you need to identify the questions to which everyone answered "yes"!
//!
//! Using the same example as above:
//!
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//!
//! This list represents answers from five groups:
//!
//!     In the first group, everyone (all 1 person) answered "yes"
//!     to 3 questions: a, b, and c.
//!
//!     In the second group, there is no question to which everyone answered "yes".
//!
//!     In the third group, everyone answered yes to only 1 question, a.
//!     Since some people did not answer "yes" to b or c, they don't count.
//!
//!     In the fourth group, everyone answered yes to only 1 question, a.
//!
//!     In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
//!
//! In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
//!
//! For each group, count the number of questions to which everyone answered "yes".
//! What is the sum of those counts?

use std::collections::{HashMap, HashSet};

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day06.txt")?;

    match std::env::args().nth(2).as_deref() {
        Some("1") => println!("Day06 part1: {}", part1(&input)),
        Some("2") => println!("Day06 part2: {}", part2(&input)),

        _ => {
            println!("Day06 part1: {}", part1(&input));
            println!("Day06 part2: {}", part2(&input));
        }
    };

    Ok(())
}

/// Count any occurence of a letter.
fn part1(s: &str) -> usize {
    s.trim()
        .split("\n\n")
        .map(|s| s.trim())
        .map(|grp: &str| {
            let unique = grp
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<char>>();
            unique.len()
        })
        .sum()
}

/// Count only the letters that appears on each row
/// per group.
/// Prev wrong answer 3237
fn part2(s: &str) -> usize {
    s.trim()
        .split("\n\n")
        .map(|s| s.trim())
        .map(|grp: &str| {
            let mut yesses = HashMap::<char, usize>::new();
            let mut no_lines = 0;
            // let mut all_yes = all_questions.clone();
            grp.lines().for_each(|line| {
                no_lines += 1;
                line.chars()
                    .for_each(|c| *yesses.entry(c).or_default() += 1)
            });

            yesses.values().filter(|&n| *n == no_lines).count()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        let s = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
"#
        .trim();
        assert_eq!(11, super::part1(s));
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day06.txt").expect("reading input");
        assert_eq!(6551, super::part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
"#
        .trim();

        assert_eq!(6, super::part2(input));
    }

    #[test]
    fn part2() {
        let input = crate::read_input("day06.txt").expect("reading input");
        assert_eq!(3358, super::part2(&input));
    }
}
