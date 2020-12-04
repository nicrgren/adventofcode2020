//! Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input);
//! apparently, something isn't quite adding up.
//!
//! Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//!
//! In this list, the two entries that sum to 2020 are 1721 and 299.
//! Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
//!
//! Of course, your expense report is much larger.
//! Find the two entries that sum to 2020; what do you get if you multiply them together?

use std::cmp::max;

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day01.txt")?;
    println!("Day01 part1: {}", part1(&input));
    println!("Day01 part2: {}", part2(&input));

    Ok(())
}

fn parse_input(s: &str) -> Vec<i64> {
    s.trim()
        .lines()
        .map(|s| s.trim())
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Day01: parsing")
}

fn part1(input: &str) -> i64 {
    let nums = parse_input(input);
    let (n1, n2) = nums
        .iter()
        .enumerate()
        .filter_map(|(i, n1)| {
            nums.iter()
                .skip(i)
                .find(|&n2| n2 + n1 == 2020)
                .map(|n2| (n1, n2))
        })
        .next()
        .expect("Day01: solving");

    n1 * n2
}

fn part2(input: &str) -> i64 {
    let nums = parse_input(input);
    let (n1, n2, n3) = nums
        .iter()
        .enumerate()
        .filter_map(|(i1, n1)| {
            nums.iter()
                .skip(i1)
                .enumerate()
                .filter_map(|(i2, n2)| {
                    nums.iter()
                        .skip(max(i1, i2))
                        .find(|&n3| n1 + n2 + n3 == 2020)
                        .map(|n3| (n1, n2, n3))
                })
                .next()
        })
        .next()
        .expect("Day01: solving");

    n1 * n2 * n3
}

#[cfg(test)]
mod tests {

    static EXAMPLE_INPUT: &str = r#"
1721
979
366
299
675
1456"#;

    #[test]
    fn part2_example() {
        assert_eq!(super::part2(EXAMPLE_INPUT.trim()), 241861950);
    }

    #[test]
    fn part1_example() {
        assert_eq!(super::part1(EXAMPLE_INPUT.trim()), 514579);
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day01.txt").expect("input");
        assert_eq!(super::part1(&input), 719796);
    }

    #[test]
    fn part2() {
        let input = crate::read_input("day01.txt").expect("input");
        assert_eq!(super::part2(&input), 144554112);
    }
}
