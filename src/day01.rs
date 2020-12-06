//! Day 01
//!
//! ## Problem:
//! From an input file containig unsigned integer values:
//! 1. find two entries that sum to 2020 and multiply those numbers together;
//! 2. find three entries that sum to 2020 and multiply them together.

extern crate test;
use crate::helpers;
use std::cmp::Ordering;

/// Parses a string containing the input data and returns a sorted array.
/// input should be a list of values separated by the new line character. If an
/// entry is negative or cannot be parsed, a value of zero is assigned.
pub fn parse_input(input: &str) -> Vec<u32> {
    let mut out: Vec<u32> = input
        .lines()
        .map(|l| if let Ok(x) = l.parse::<u32>() { x } else { 0 })
        .collect();
    out.sort();
    out
}

/// Tries to find a pair that sums to a target.
///
/// # Arguments
///
/// - `sorted`  - A **sorted** array slice.
/// - `target`  - The sum we are looking for.
///
/// # Assumptions
///
/// The input array slice `sorted` is sorted and does not contain zeros.
///
/// # Implementation Details
///
/// Uses the two pointer technique (low and high cursors) implemented directly
/// on the iterator output values to achive O(n) performance.
///
/// todo: explore alternatives using Dynamic Programming, HashSets and
/// HashMaps.
pub fn two_sum(sorted: &[u32], target: u32) -> Option<(u32, u32)> {
    // filter is nice if target is a small value
    let mut iter = sorted.iter().filter(|&&x| x < target);
    let mut lo = *iter.next()?;
    let mut hi = *iter.next_back()?;

    loop {
        let sum = lo + hi;
        match sum.cmp(&target) {
            Ordering::Less => lo = *iter.next()?,
            Ordering::Greater => hi = *iter.next_back()?,
            Ordering::Equal => return Some((lo, hi)),
        }
    }
}

/// Tries to find a triplet that sums to a target integer value.
/// ## Assumptions
/// input array is sorted.
fn three_sum(sorted: &[u32], target: u32) -> Option<(u32, u32, u32)> {
    for &pivot in sorted {
        if pivot > target {
            break;
        }
        if let Some((a, b)) = two_sum(sorted, target - pivot) {
            return Some((pivot, a, b));
        }
    }
    return None;
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    let str = &helpers::read_to_str("day01").expect("Unable to read file.");
    let sorted = parse_input(str);
    // part 01
    let (a, b) = two_sum(&sorted, 2020).expect("No pair of values found.");
    // part 02
    let (c, d, e) = three_sum(&sorted, 2020).expect("No triplet found.");
    println!("Day 01");
    println!("Pair: {} * {} = {}", a, b, a * b);
    println!("Triplet: {} * {} * {} = {}", c, d, e, c * d * e);
    println!();
}

//--------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Vec<u32> = vec![299, 366, 675, 979, 1456, 1721];
        static ref PARSED: Vec<u32> =
            parse_input(&helpers::read_to_str("day01").expect("Unable to read file."));
    }

    #[test]
    fn test_parser() {
        let input = "1721\n979\n366\n299\n675\n1456\n282";
        assert_eq!(parse_input(input), *INPUT);
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(INPUT.as_ref(), 1041), Some((366, 675)));
        assert_eq!(two_sum(INPUT.as_ref(), 1653), None);
        assert_eq!(two_sum(INPUT.as_ref(), 1654), Some((675, 979)));
        assert_eq!(two_sum(INPUT.as_ref(), 1655), None);
        assert_eq!(two_sum(INPUT.as_ref(), 2020), Some((299, 1721)));
    }

    #[test]
    fn test_three_sum() {
        assert_eq!(three_sum(INPUT.as_ref(), 1041), None);
        assert_eq!(three_sum(INPUT.as_ref(), 2020), Some((366, 675, 979)));
    }
    //--------------------------------------------------------------------
    // Benches
    //--------------------------------------------------------------------

    // For the Advent of Code Target
    #[bench]
    fn bench_two_sum(b: &mut Bencher) {
        b.iter(|| two_sum(PARSED.as_ref(), 2020));
    }

    #[bench]
    fn bench_three_sum(b: &mut Bencher) {
        b.iter(|| three_sum(PARSED.as_ref(), 2020));
    }

    // Large target and no matches
    #[bench]
    fn bench_three_sum2(b: &mut Bencher) {
        b.iter(|| three_sum(PARSED.as_ref(), 1711));
    }
}
