//! Day 01
//!
//! ## Problem:
//! From an input file containig unsigned integer values:
//! 1. find two entries that sum to 2020 and multiply those numbers together;
//! 2. find three entries that sum to 2020 and multiply them together.

extern crate test;

use std::cmp::Ordering;

use crate::helpers::read;
///
/// # Arguments
///
///  - `input` - A string slice holding unsigned integer values separated by the
///    new line character.
///
/// # Implementation Details
///
/// Values that are zero, negative or cannot be parsed are not included in the
/// output.
pub fn parse_input(input: &str) -> Vec<u32> {
    let mut out: Vec<u32> = input
        .lines()
        .map(|l| if let Ok(x) = l.parse::<u32>() { x } else { 0 })
        .filter(|&v| v > 0)
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

/// Tries to find a triplet that sums to a target.
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
/// Iterates over the array for pivot values and then uses `two_sum()` to find a
/// suitable pair to complete the sum. Its performance is O(n^2).
///
/// todo: there is an algorithm for three_sum that uses FFT. Check Wikipedia
/// later.
pub fn three_sum(sorted: &[u32], target: u32) -> Option<(u32, u32, u32)> {
    let iter = sorted
        .into_iter()
        .enumerate()
        .take_while(|(_, &pivot)| pivot < target);
    for (i, &pivot) in iter {
        if let Some((a, b)) = two_sum(&sorted[i + 1..], target - pivot) {
            return Some((pivot, a, b));
        }
    }
    return None;
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    let str = read::read_to_str("day01").expect("Unable to read file.");
    let sorted = parse_input(&str);
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
        static ref INPUT: Vec<u32> = vec![282, 299, 366, 675, 979, 1456, 1721];
        static ref PARSED: Vec<u32> =
            parse_input(&read::read_to_str("day01").expect("Unable to read file."));
    }

    #[test]
    fn test_parser() {
        let input = "1721\n979\n0\n366\n299\n675\n1456\n282";
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
        assert_ne!(
            three_sum(INPUT.as_ref(), 2020),
            Some((282, 282, 1456)),
            "Corner case, pivot is being used again"
        );
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
