//! --- Day 3: Toboggan Trajectory ---
//!
//! With the toboggan login problems resolved, you set off toward the airport.
//! While travel by toboggan might be easy, it's certainly not safe:
//! there's very minimal steering and the area is covered in trees.
//! You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact
//! integer coordinates in a grid.
//! You make a map (your puzzle input) of the open squares (.) and trees (#) you can see.
//! For example:
//!
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//!
//! These aren't the only trees, though; due to something you read about
//! once involving arboreal genetics and biome stability, the same pattern
//! repeats to the right many times:
//!
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//!
//! You start on the open square (.) in the top-left corner and need to reach
//! the bottom (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes
//! (you opted for a cheaper model that prefers rational numbers);
//! start by counting all the trees you would encounter for the slope right 3, down 1:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1.
//! Then, check the position that is right 3 and down 1 from there, and so on until
//! you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with O where
//! there was an open square and X where there was a tree:
//!
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//!
//! In this example, traversing the map using this slope would cause you to encounter 7 trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1,
//! how many trees would you encounter?
//!
//!
//!
//! --- Part Two ---
//!
//! Time to check the rest of the slopes - you need to minimize the probability of
//! a sudden arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the following slopes,
//! you start at the top-left corner and traverse the map all the way to the bottom:
//!
//!     Right 1, down 1.
//!     Right 3, down 1. (This is the slope you already checked.)
//!     Right 5, down 1.
//!     Right 7, down 1.
//!     Right 1, down 2.
//!
//! In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively;
//! multiplied together, these produce the answer 336.
//!
//! What do you get if you multiply together the number of trees encountered
//! on each of the listed slopes?

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day03.txt")?;
    println!("Day03 part1: {}", solve_part1(&input));
    println!("Day03 part1: {}", solve_part2(&input));

    Ok(())
}
/// Returns the number of trees encountered.
fn solve_part1(input: &str) -> usize {
    solver(input, 3, 1)
}

/// Returns the number of trees encountered.
fn solve_part2(input: &str) -> usize {
    solver(input, 1, 1)
        * solver(input, 3, 1)
        * solver(input, 5, 1)
        * solver(input, 7, 1)
        * solver(input, 1, 2)
}

/// Returns the number of trees encountered.
///
/// # Arguments
///
/// * `input` - The input string
/// * `x_step` - The steps to move on each line per iteration.
/// * `y_step` - The number of lines to jump after each iteration.
fn solver(input: &str, x_step: usize, y_step: usize) -> usize {
    let mut tree_count = 0;
    let mut col_index = 0;

    for line in input.trim().lines().map(|s| s.trim()).step_by(y_step) {
        let c = line.chars().nth(col_index % line.len()).unwrap();
        if c == '#' {
            tree_count += 1;
        }
        col_index += x_step;
    }

    tree_count
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXAMPLE_INPUT: &str = r#"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(EXAMPLE_INPUT.trim()), 7);
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day03.txt").expect("Reading input");
        assert_eq!(solve_part1(&input), 184);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(EXAMPLE_INPUT.trim()), 336);
    }

    #[test]
    fn part2() {
        let input = crate::read_input("day03.txt").expect("Reading input");
        assert_eq!(solve_part2(&input), 2431272960);
    }
}
