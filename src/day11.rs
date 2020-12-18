use std::fmt;

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day11.txt")?;

    println!("Day11 part1: {}", part1(&input));

    Ok(())
}

pub fn part1(s: &str) -> usize {
    let mut board = Board::from_str(s);

    while board.tick() {}

    board.no_occupied()
}

struct Board {
    seats: Vec<Seat>,
    width: usize,
}

impl Board {
    fn from_str(s: &str) -> Self {
        let s = s.trim();
        Self {
            width: s.find('\n').unwrap_or_else(|| s.len()),
            seats: s
                .chars()
                .filter(|c| *c != '\n')
                .map(Seat::from_char)
                .collect::<Vec<_>>(),
        }
    }

    fn rows(&self) -> usize {
        self.seats.len() / self.width
    }

    fn cols(&self) -> usize {
        self.width
    }

    fn tick(&mut self) -> bool {
        let mut new_board = vec![Seat::Empty; self.seats.len()];
        let mut changed = false;
        for col in 0..self.cols() {
            for row in 0..self.rows() {
                let idx = col + (row * self.width);

                match self.seat_at(Col(col), Row(row)).expect("getting seat") {
                    Seat::Empty if self.adjecently_occupied(Col(col), Row(row)) == 0 => {
                        changed = true;
                        new_board[idx] = Seat::Occupied;
                    }

                    Seat::Occupied if 4 <= self.adjecently_occupied(Col(col), Row(row)) => {
                        changed = true;
                        new_board[idx] = Seat::Empty;
                    }

                    seat => new_board[idx] = seat,
                }
            }
        }

        self.seats = new_board;
        changed
    }

    fn adjecently_occupied(&self, col: Col, row: Row) -> usize {
        let mut count = 0;

        // Above Left
        if 0 < col.0 && 0 < row.0 && self.is_occupied_at(col - 1, row - 1) {
            count += 1;
        }

        // Above
        if 0 < row.0 && self.is_occupied_at(col, row - 1) {
            count += 1;
        }

        // Above Right
        if 0 < row.0 && self.is_occupied_at(col + 1, row - 1) {
            count += 1;
        }

        // To Right of
        if self.is_occupied_at(col + 1, row) {
            count += 1;
        }

        // Below right
        if self.is_occupied_at(col + 1, row + 1) {
            count += 1;
        }
        // Below
        if self.is_occupied_at(col, row + 1) {
            count += 1;
        }

        // Below left
        if 0 < col.0 && self.is_occupied_at(col - 1, row + 1) {
            count += 1;
        }
        // To left of
        if 0 < col.0 && self.is_occupied_at(col - 1, row) {
            count += 1;
        }

        count
    }

    fn is_occupied_at(&self, col: Col, row: Row) -> bool {
        self.seat_at(col, row)
            .filter(|seat| seat.is_occupied())
            .is_some()
    }

    fn seat_at(&self, col: Col, row: Row) -> Option<Seat> {
        if col.0 < self.cols() && row.0 < self.rows() {
            self.seats.get(col.0 + row.0 * self.width).copied()
        } else {
            None
        }
    }

    fn no_occupied(&self) -> usize {
        self.seats.iter().filter(|s| s.is_occupied()).count()
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board = self
            .seats
            .chunks(self.width)
            .map(|line| line.iter().map(Seat::to_char).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", board)
    }
}

#[derive(Clone, Copy)]
struct Row(usize);

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("R{}", self.0))
    }
}

impl std::ops::Sub<usize> for Row {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl std::ops::Add<usize> for Row {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

#[derive(Clone, Copy)]
struct Col(usize);

impl fmt::Display for Col {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("R{}", self.0))
    }
}

impl std::ops::Sub<usize> for Col {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl std::ops::Add<usize> for Col {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl Seat {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Empty,
            '#' => Self::Occupied,
            '.' => Self::Floor,
            c => panic!("Unknown Tile `{}`", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Empty => 'L',
            Self::Occupied => '#',
            Self::Floor => '.',
        }
    }

    fn is_occupied(&self) -> bool {
        matches!(self, Self::Occupied)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_adjecently() {
        use super::{Board, Col, Row};
        assert_eq!(
            0,
            Board::from_str(r#"LLL"#).adjecently_occupied(Col(1), Row(0))
        );
        assert_eq!(
            1,
            Board::from_str(r#"#LL"#).adjecently_occupied(Col(1), Row(0))
        );
        assert_eq!(
            2,
            Board::from_str(r#"#L#"#).adjecently_occupied(Col(1), Row(0))
        );
        assert_eq!(
            2,
            Board::from_str(
                r#"
#L#
LLL
"#
            )
            .adjecently_occupied(Col(1), Row(0))
        );
        assert_eq!(
            3,
            Board::from_str(
                r#"
#L#
#LL
"#
            )
            .adjecently_occupied(Col(1), Row(0))
        );
        assert_eq!(
            4,
            Board::from_str(
                r#"
#L#
##L
"#
            )
            .adjecently_occupied(Col(1), Row(0))
        );
        assert_eq!(
            5,
            Board::from_str(
                r#"
#L#
###
"#
            )
            .adjecently_occupied(Col(1), Row(0))
        );

        assert_eq!(
            8,
            Board::from_str(
                r#"
###
#L#
###
"#
            )
            .adjecently_occupied(Col(1), Row(1))
        );
    }

    #[test]
    fn part1_example() {
        let mut board = super::Board::from_str(
            r#"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#
            .trim(),
        );

        let versions = vec![
            r#"
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
"#
            .trim(),
            r#"
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
"#
            .trim(),
            r#"
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
"#
            .trim(),
            r#"
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
"#
            .trim(),
            r#"
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
"#
            .trim(),
        ];

        for (n, version) in versions.into_iter().enumerate() {
            board.tick();
            assert_eq!(version, board.to_string(), "Version #{}", n);
        }

        assert_eq!(37, board.no_occupied());
    }
}
