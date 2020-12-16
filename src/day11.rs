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
            width: s.find('\n').expect("Cant find first linefeed"),
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
        for col in 0..self.cols() {
            for row in 0..self.rows() {
                match self.seat_at(col, row).expect("getting seat") {
                    Seat::Empty if self.adjencently_occupied(col, row) == 0 => {
                        self.seats[col + (row * self.width)] = Seat::Occupied;
                    }

                    Seat::Occupied if 4 < self.adjencently_occupied(col, row) => {
                        self.seats[col + (row * self.width)] = Seat::Empty;
                    }

                    _ => (),
                }
            }
        }

        false
    }

    fn adjencently_occupied(&self, col: usize, row: usize) -> usize {
        let mut count = 0;
        // Above Left
        if 0 < col && row < 0 && self.is_occupied_at(col - 1, row - 1) {
            count += 1;
        }

        // Above
        if 0 < row && self.is_occupied_at(col, row - 1) {
            count += 1;
        }

        // Above Right
        if 0 < row && self.is_occupied_at(col + 1, row - 1) {
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
        if 0 < col && self.is_occupied_at(col - 1, row + 1) {
            count += 1;
        }
        // To left of
        if 0 < col && self.is_occupied_at(col - 1, row) {
            count += 1;
        }

        count
    }

    fn is_occupied_at(&self, col: usize, row: usize) -> bool {
        self.seat_at(col, row)
            .filter(|seat| seat.is_occupied())
            .is_some()
    }

    fn seat_at(&self, col: usize, row: usize) -> Option<Seat> {
        if 0 <= col && col < self.cols() && 0 <= row && row < self.rows() {
            self.seats.get(col + row * self.width).copied()
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
        for slice in self.seats.chunks(self.width) {
            let row = slice.iter().map(Seat::to_char).collect::<String>();
            f.write_str(&row)?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
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
    fn board() {
        let s = r#"
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
"#;

        let board = super::Board::from_str(s);

        assert_eq!(10, board.cols());
        assert_eq!(10, board.rows());

        assert_eq!(37, super::part1(s));
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
    }
}
