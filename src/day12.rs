use std::str::FromStr;

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day12.txt")?;
    println!("Day12 part1: {}", part1(&input));
    Ok(())
}

fn part1(s: &str) -> usize {
    let mut ship = Ship::default();

    for action in parse(s) {
        ship.apply(action);
    }

    ship.distance_from_start()
}

fn parse<'a>(s: &'a str) -> impl IntoIterator<Item = Action> + 'a {
    s.trim()
        .lines()
        .map(|s| s.trim().parse::<Action>().expect("Parsing action"))
}

struct Point {
    x: i32,
    y: i32,
}

struct Ship {
    facing: Direction,
    position: Point,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            facing: Direction::East,
            position: Point { x: 0, y: 0 },
        }
    }
}

impl Ship {
    fn apply(&mut self, action: Action) {
        match action.op {
            Op::MoveNorth => {
                self.position.y += action.n as i32;
            }
            Op::MoveEast => {
                self.position.x += action.n as i32;
            }
            Op::MoveSouth => {
                self.position.y -= action.n as i32;
            }
            Op::MoveWest => {
                self.position.x -= action.n as i32;
            }
            Op::RotateLeft => {
                self.facing = self.facing.rotate_left(action.n);
            }
            Op::RotateRight => {
                self.facing = self.facing.rotate_right(action.n);
            }
            Op::Forward => self.apply(Action {
                op: self.facing.into(),
                n: action.n,
            }),
        }
    }

    fn distance_from_start(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs()) as usize
    }
}

/// A number between 0-4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Into<Op> for Direction {
    fn into(self) -> Op {
        match self {
            Self::North => Op::MoveNorth,
            Self::East => Op::MoveEast,
            Self::South => Op::MoveSouth,
            Self::West => Op::MoveWest,
        }
    }
}

impl From<usize> for Direction {
    fn from(n: usize) -> Self {
        match n {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            n => panic!("Invalid Direction discriminate: `{}`", n),
        }
    }
}

impl Direction {
    fn rotate_left(self, deg: usize) -> Self {
        let deg = 360 - (deg % 360);
        let disc = self as usize;
        let new_deg = (disc + deg / 90) % 4;

        Self::from(new_deg as usize)
    }

    fn rotate_right(self, deg: usize) -> Self {
        let new_deg = (self as usize + deg / 90) % 4;
        Self::from(new_deg)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    RotateLeft,
    RotateRight,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Action {
    op: Op,
    n: usize,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op_s, n_s) = s.split_at(1);

        let op = match op_s {
            "N" => Op::MoveNorth,
            "S" => Op::MoveSouth,
            "E" => Op::MoveEast,
            "W" => Op::MoveWest,
            "L" => Op::RotateLeft,
            "R" => Op::RotateRight,
            "F" => Op::Forward,
            s => return Err(format!("Invalid OP `{}`", s)),
        };

        let n = n_s
            .parse::<usize>()
            .map_err(|_| format!("Invalid Action value `{}`", n_s))?;

        Ok(Action { op, n })
    }
}

#[cfg(test)]
mod tests {

    use super::{Action, Direction, Op, Ship};

    static INPUT: &str = r#"
F10
N3
F7
R90
F11
"#;

    #[test]
    fn part1_example() {
        let mut ship = Ship::default();
        for action in super::parse(INPUT) {
            ship.apply(action);
        }

        assert_eq!(17, ship.position.x);
        assert_eq!(-8, ship.position.y);
        assert_eq!(25, ship.distance_from_start());
    }

    #[test]
    fn parse_example() {
        let actions = super::parse(INPUT).into_iter().collect::<Vec<_>>();
        assert_eq!(
            actions[0],
            Action {
                op: Op::Forward,
                n: 10
            }
        );
        assert_eq!(
            actions[1],
            Action {
                op: Op::MoveNorth,
                n: 3
            }
        );
        assert_eq!(
            actions[2],
            Action {
                op: Op::Forward,
                n: 7
            }
        );
        assert_eq!(
            actions[3],
            Action {
                op: Op::RotateRight,
                n: 90
            }
        );
        assert_eq!(
            actions[4],
            Action {
                op: Op::Forward,
                n: 11
            }
        );
    }

    #[test]
    fn apply_rotations_to_direction() {
        let dir = Direction::East;

        assert_eq!(Direction::South, dir.rotate_right(90));
        assert_eq!(Direction::East, dir.rotate_right(0));
        assert_eq!(Direction::West, dir.rotate_right(180));
        assert_eq!(Direction::North, dir.rotate_right(270));
        assert_eq!(Direction::East, dir.rotate_right(360));

        let dir = Direction::North;
        assert_eq!(Direction::East, dir.rotate_right(90));
        assert_eq!(Direction::South, dir.rotate_right(180));
        assert_eq!(Direction::West, dir.rotate_right(270));
        assert_eq!(Direction::North, dir.rotate_right(360));
        assert_eq!(Direction::East, dir.rotate_right(450));
        assert_eq!(Direction::North, dir.rotate_right(720));
        assert_eq!(Direction::North, dir.rotate_right(1080));

        let dir = Direction::East;
        assert_eq!(Direction::North, dir.rotate_left(90));
        assert_eq!(Direction::West, dir.rotate_left(180));
        assert_eq!(Direction::South, dir.rotate_left(270));
        assert_eq!(Direction::East, dir.rotate_left(360));
        assert_eq!(Direction::East, dir.rotate_left(720));
        assert_eq!(Direction::East, dir.rotate_left(1080));
    }
}
