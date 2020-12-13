//! --- Day 9: Encoding Error ---
//!
//! With your neighbor happily enjoying their video game, you turn your attention
//! to an open data port on the little screen in the seat in front of you.
//!
//! Though the port is non-standard, you manage to connect it to your computer through
//! the clever use of several paperclips. Upon connection, the port outputs a series
//! of numbers (your puzzle input).
//!
//! The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which,
//! conveniently for you, is an old cypher with an important weakness.
//!
//! XMAS starts by transmitting a preamble of 25 numbers. After that, each number
//! you receive should be the sum of any two of the 25 immediately previous numbers.
//! The two numbers will have different values, and there might be more than one such pair.
//!
//! For example, suppose your preamble consists of the numbers 1 through 25 in a random order.
//! To be valid, the next number must be the sum of two of those numbers:
//!
//!     * 26 would be a valid next number, as it could be 1 plus 25
//!       (or many other pairs, like 2 and 24).
//!     * 49 would be a valid next number, as it is the sum of 24 and 25.
//!     * 100 would not be valid; no two of the previous 25 numbers sum to 100.
//!     * 50 would also not be valid; although 25 appears in the previous 25 numbers,
//!       the two numbers in the pair must be different.
//!
//! Suppose the 26th number is 45, and the first number (no longer an option, as it
//! is more than 25 numbers ago) was 20. Now, for the next number to be valid,
//! there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:
//!
//!     * 26 would still be a valid next number, as 1 and 25 are still within
//!       the previous 25 numbers.
//!     * 65 would not be valid, as no two of the available numbers sum to it.
//!     * 64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
//!
//! Here is a larger example which only considers the previous 5 numbers
//! (and has a preamble of length 5):
//!
//! ```
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this example, after the 5-number preamble, almost every number is
//! the sum of two of the previous 5 numbers; the only number that does
//! not follow this rule is 127.
//!
//! The first step of attacking the weakness in the XMAS data is tofind the
//! first number in the list (after the preamble) which is not the sum of two
//! of the 25 numbers before it.
//! What is the first number that does not have this property?
pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day09.txt")?;
    println!("Day09 part1: {}", part1(25, &input));
    Ok(())
}

fn part1(preamble_size: usize, s: &str) -> u64 {
    verify(preamble_size, s)
        .err()
        .expect("Wanted error")
        .element
}

fn verify(preamble_size: usize, s: &str) -> Result<(), XmasDecodeError> {
    let mut it = s
        .trim()
        .lines()
        .map(|s| s.trim().parse::<u64>().expect("parsing input line"));

    let mut preamble = Vec::with_capacity(preamble_size);

    // Read first _preamble_size_ numbers.
    for _ in 0..preamble_size {
        preamble.push(it.next().expect("Incomplete Preamble"));
    }

    // Verify remaining
    for (index, element) in it.enumerate() {
        if !preamble
            .iter()
            .enumerate()
            .any(|(i, n)| preamble.iter().skip(i).any(|n2| n + n2 == element))
        {
            return Err(XmasDecodeError { index, element });
        }

        preamble.remove(0);
        preamble.push(element);
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct XmasDecodeError {
    index: usize,
    element: u64,
}
#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        let input = r#"
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#
        .trim();

        assert_eq!(127, super::part1(5, &input));
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day09.txt").expect("reading input");
        assert_eq!(530627549, super::part1(25, &input));
    }
}
