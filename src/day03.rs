//! Day 03
//!
//! # Problem
//!
//! Given an input representing a map of trees in an area:
//!
//! 1. Count how many collisions would happen if following the slope (3, 1);
//!
//! 2. What is the product betwwen the number of collision for the slopes
//! [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].

use std::convert::TryInto;

use crate::helpers::{base2d::Base2d, grid::Grid, read};

/// Parses a string containing the input data and returns a `Grid` of bool
/// values representing trees (`true`) and free paths (`false`).
///
/// `input` - a string whose characters represent open paths and trees. The
/// character `#` represents trees whereas `.` stands for free paths.
///
/// # Assumptions
///
/// - Input data is well behaved.
pub fn parse_input(input: &str) -> Grid<bool> {
    let len_x = input.find('\n').unwrap();
    let flat: Vec<bool> = input
        .lines()
        .map(|line| line.chars())
        .flatten()
        .map(|c| c == '#')
        .collect();
    Grid::new_with_vec(len_x, flat.len() / len_x, flat)
}

/// Returns the number of collisions given a forest and a slope.
///
/// - `forest` - a Grid of booleans holding positions of trees and free paths.
///
/// - `slope` - the movement of the tobogan in the forest for the `x` and `y`
///   directions.
pub fn count_collisions(forest: &Grid<bool>, slope: Base2d<usize>) -> u32 {
    let mut collisions = 0;
    let (mut x, mut y) = (0, 0);
    let (dx, dy) = slope.tuple();
    while y < forest.len_y {
        if *forest.wrap_x(x, y) {
            collisions += 1;
        }
        x += dx;
        y += dy;
    }
    collisions
}

/// Runs `count_collisions()` for differents `slopes` and returns a `Vec` with
/// the number of collision for them.
///
/// - `forest` - a Grid of booleans holding positions of trees and free paths.
///
/// - `slopes` - an array containing slopes to run.
pub fn count_for_many(forest: &Grid<bool>, slopes: &[Base2d<usize>]) -> Vec<u32> {
    slopes
        .iter()
        .map(|&slope| count_collisions(forest, slope))
        .collect()
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    let str = read::read_to_str("day03").unwrap();
    let forest = parse_input(&str);
    // Parte 01
    println!("Day 03");
    println!(
        "Number of collisions: {}",
        count_collisions(&forest, (3, 1).try_into().unwrap())
    );
    // Parte 02
    // using tuples for creating the `Base2d` slopes
    let slopes: Vec<Base2d<usize>> = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&v| v.try_into().unwrap())
        .collect();
    println!(
        "Product: {}",
        count_for_many(&forest, &slopes)
            .into_iter()
            .product::<u32>()
    );
    println!();
}

//--------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    lazy_static! {
        static ref PARSED: Grid<bool> = parse_input(INPUT);
    }

    #[test]
    fn test_parser() {
        let s = ".#....\n#..#..";
        assert_eq!(
            parse_input(s),
            Grid::new_with_vec(
                6,
                2,
                vec![
                    false, true, false, false, false, false, true, false, false, true, false, false
                ]
            )
        );
    }

    #[test]
    fn test_count_collisions() {
        let slope = (3, 1).try_into().unwrap();
        assert_eq!(count_collisions(&PARSED, slope), 7);
    }

    #[test]
    fn test_count_for_many() {
        let slopes: Vec<Base2d<usize>> = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&v| v.try_into().unwrap())
            .collect();
        let out = count_for_many(&PARSED, &slopes);
        assert_eq!(&out[0..], &[2, 7, 3, 4, 2]);
    }
}
