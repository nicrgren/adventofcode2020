//! -- Day 5: Binary Boarding ---
//!
//! You board your plane only to discover a new problem: you dropped your boarding pass!
//! You aren't sure which seat is yours, and all of the flight attendants are busy with
//! the flood of people that suddenly made it through passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the nearby
//! boarding passes (your puzzle input); perhaps you can find your seat through
//! process of elimination.
//!
//! Instead of zones or groups, this airline uses binary space partitioning to
//! seat people. A seat might be specified like FBFBBFFRLR, where F means "front",
//! B means "back", L means "left", and R means "right".
//!
//! The first 7 characters will either be F or B; these specify exactly one
//! of the 128 rows on the plane (numbered 0 through 127). Each letter tells
//! you which half of a region the given seat is in. Start with the whole list of rows;
//! the first letter indicates whether the seat is in the front (0 through 63) or the back
//! (64 through 127). The next letter indicates which half of that region the seat
//! is in, and so on until you're left with exactly one row.
//!
//! For example, consider just the first seven characters of FBFBBFFRLR:
//!
//!     Start by considering the whole range, rows 0 through 127.
//!     F means to take the lower half, keeping rows 0 through 63.
//!     B means to take the upper half, keeping rows 32 through 63.
//!     F means to take the lower half, keeping rows 32 through 47.
//!     B means to take the upper half, keeping rows 40 through 47.
//!     B keeps rows 44 through 47.
//!     F keeps rows 44 through 45.
//!     The final F keeps the lower of the two, row 44.
//!
//! The last three characters will be either L or R; these specify exactly one of
//! the 8 columns of seats on the plane (numbered 0 through 7).
//! The same process as above proceeds again, this time with only three steps.
//! L means to keep the lower half, while R means to keep the upper half.
//!
//! For example, consider just the last 3 characters of FBFBBFFRLR:
//!
//!     Start by considering the whole range, columns 0 through 7.
//!     R means to take the upper half, keeping columns 4 through 7.
//!     L means to take the lower half, keeping columns 4 through 5.
//!     The final R keeps the upper of the two, column 5.
//!
//! So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
//!
//! Every seat also has a unique seat ID: multiply the row by 8, then add the column.
//! In this example, the seat has ID 44 * 8 + 5 = 357.
//!
//! Here are some other boarding passes:
//!
//!     BFFFBBFRRR: row 70, column 7, seat ID 567.
//!     FFFBBBFRRR: row 14, column 7, seat ID 119.
//!     BBFFBBFRLL: row 102, column 4, seat ID 820.
//!
//! As a sanity check, look through your list of boarding passes. What is the
//! highest seat ID on a boarding pass?
//!
//!
//!
//! --- Part Two ---
//!
//! Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding
//! pass in your list. However, there's a catch: some of the seats at the very front
//! and back of the plane don't exist on this aircraft,
//! so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats
//! with IDs +1 and -1 from yours will be in your list.
//!
//! What is the ID of your seat?
//!

use std::str::FromStr;

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day05.txt")?;
    println!("Day05 part1: {}", part1(&input));
    println!("Day05 part2: {}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|s| Seat::from_str(s).expect("Invalid seat"))
        .map(|seat| seat.id())
        .max()
        .expect("Could not find highest seat")
}

fn part2(input: &str) -> usize {
    let mut seats = input
        .trim()
        .lines()
        .map(|s| Seat::from_str(s).expect("Invalid seat"))
        .collect::<Vec<_>>();

    seats.sort_by(|s1, s2| s1.id().cmp(&s2.id()));

    // Iterate over all seats viewing 2 at each iter.
    // 1: [a b] c d e
    // 2: a [b c] d e
    // 3: a b [c d] e
    for slice in seats.windows(2) {
        let s1 = &slice[0];
        let s2 = &slice[1];

        // If the right id is not one smaller than the prev.
        // It must be the seat.
        if s2.id() != s1.id() + 1 {
            return s1.id() + 1;
        }
    }

    panic!("Could not find the seat!");
}

fn part2_fst(input: &str) -> usize {
    let mut seats = input
        .trim()
        .lines()
        .map(|s| Seat::from_str(s).expect("Invalid seat"))
        .collect::<Vec<_>>();

    seats.sort_by(|s1, s2| s1.id().cmp(&s2.id()).reverse());

    // Iterate over all seats viewing 2 at each iter.
    // 1: [a b] c d e
    // 2: a [b c] d e
    // 3: a b [c d] e
    for slice in seats.windows(2) {
        let s1 = &slice[0];
        let s2 = &slice[1];

        // If the right id is not one smaller than the prev.
        // It must be the seat.
        if s2.id() != s1.id() - 1 {
            return s1.id() - 1;
        }
    }

    panic!("Could not find the seat!");
}

impl FromStr for Seat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = 0..128;
        let mut cols = 0..8;

        for c in s.chars() {
            match c {
                'B' => rows.start += rows.len() / 2,
                'F' => rows.end -= rows.len() / 2,
                'R' => cols.start += cols.len() / 2,
                'L' => cols.end -= cols.len() / 2,
                c => return Err(format!("`{}`. Invalid char `{}`", s, c)),
            }
        }

        assert_eq!(cols.len(), 1);
        assert_eq!(rows.len(), 1);

        Ok(Seat {
            row: rows.start,
            col: cols.start,
        })
    }
}

struct Seat {
    row: usize,
    col: usize,
}
impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }

    #[cfg(test)]
    fn from_id(id: usize) -> Self {
        Self {
            row: id / 8,
            col: id % 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Seat;
    #[test]
    fn part1_example() {
        let tests = vec![
            ("FBFBBFFRLR", 44, 5, 357),
            ("BFFFBBFRRR", 70, 7, 567),
            ("FFFBBBFRRR", 14, 7, 119),
            ("BBFFBBFRLL", 102, 4, 820),
        ];

        for (s, row, col, id) in tests {
            let seat: Seat = s.parse().expect("Parsing seat");
            assert_eq!(row, seat.row, "Invalid row for {}", s);
            assert_eq!(col, seat.col, "Invalid col for {}", s);
            assert_eq!(id, seat.id(), "Invalid id for {}", s);
        }
    }

    #[test]
    fn seat_from_id() {
        let input = crate::read_input("day05.txt").expect("reading input");
        input
            .trim()
            .lines()
            .map(|s| s.parse::<Seat>().expect("Parsing seat"))
            .for_each(|seat| assert_eq!(seat.id(), Seat::from_id(seat.id()).id()));
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day05.txt").expect("reading input");
        assert_eq!(801, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = crate::read_input("day05.txt").expect("reading input");
        assert_eq!(597, super::part2(&input));
    }
}
