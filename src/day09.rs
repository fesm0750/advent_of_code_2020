//! All values are supposed to be positive. Using i32 instead of u32 to avoid
//! dealing with subtraction with overflow.

// todo: test this solution using a HashMap in the iterator against a
// brute-force one. The input may be not be bigger enough to justify the
// hashmap.

use std::{cmp::Ordering, collections::HashMap};

use crate::helpers::read;

/// An iterator over xmas data yielding the current xmas number and the values
/// that sum to it.
struct XmasIterator<'a> {
    curr: usize,
    xmas: &'a [i32],
    previous: HashMap<i32, u8>, /* hashmap of allowable values to compose next number and
                                 * their frequencies, values may repeat */
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    println!("Day 09");
    let input = read::to_str("day09").unwrap();
    let xmas = read::lines_into_vec(&input);
    let invalid = find_invalid(&xmas).unwrap();
    println!(
        "First number, not in the preambule, that is not a sum of two numbers before it: {}",
        invalid
    );
    println!("Encryption Weakness: {}", find_weakness(&xmas, invalid));
}

//--------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------

/// Returs the first number in `xmas` that is not a sum of the previous
/// `POOL_LEN` (25) values.
fn find_invalid(xmas: &[i32]) -> Option<i32> {
    let iter = XmasIterator::new(xmas);
    for (curr, sum) in iter {
        if sum.is_none() {
            return Some(curr);
        }
    }
    None
}

/// Finds a contiguous slice inside the `xmas` input that sums to the
/// target value. Uses a sliding and resizable window.
///
/// # Assumptions
/// xmas contains only positive and zero values.
fn find_contiguous_set(xmas: &[i32], target: i32) -> Option<&[i32]> {
    let mut start_idx = 0;
    let mut sum = 0;

    for (idx, &x) in xmas.iter().enumerate() {
        sum += x;

        while sum > target {
            sum -= xmas[start_idx];
            start_idx += 1;
        }

        if sum == target {
            return Some(&xmas[start_idx..=idx]);
        }
    }
    None
}

/// Helper method to find the minimum and maximum value in an slice. Returns a
/// tuple (min, max). Uses the two cursors approach.
fn min_max(slice: &[i32]) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for &x in slice {
        if x > max {
            max = x
        }
        if x < min {
            min = x
        }
    }
    (min, max)
}

/// find the weakness of the xmas encryption
fn find_weakness(xmas: &[i32], invalid: i32) -> i32 {
    let set = find_contiguous_set(xmas, invalid).unwrap();
    let (min, max) = min_max(set);
    min + max
}

//--------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------

impl<'a> XmasIterator<'a> {
    const POOL_LEN: usize = 25; // preambule size

    /// Creates a new XmasIterator from a slice of xmas values.
    fn new(xmas: &'a [i32]) -> Self {
        if xmas.len() <= XmasIterator::POOL_LEN {
            panic!("Data is too small for the Xmas encryption");
        }

        let mut preambule = HashMap::<i32, u8>::new();
        for &p in xmas.iter().take(XmasIterator::POOL_LEN) {
            preambule
                .entry(p)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        }

        Self {
            curr: XmasIterator::POOL_LEN,
            previous: preambule,
            xmas,
        }
    }
}

impl<'a> Iterator for XmasIterator<'a> {
    type Item = (i32, Option<(i32, i32)>);

    /// Returs a tuple containig the current number and an option for the pair
    /// of values that sums to it. If no valid pair has been found, the inner
    /// option will be None.
    ///
    /// A pair of values is a valid sum if:
    /// - both values are in the set of the immediate 25 numbers before the
    ///   current one;
    /// - they must not be equal.
    fn next(&mut self) -> Option<Self::Item> {
        // check termination
        if self.curr >= self.xmas.len() {
            return None;
        }

        // find sum
        let mut sum = None;
        let curr = self.xmas[self.curr];
        let previous = &self.xmas[self.curr - XmasIterator::POOL_LEN..self.curr];
        for &n in previous {
            let complement = curr - n;
            if complement != n && self.previous.contains_key(&complement) {
                sum = Some((n, complement));
            }
        }

        // update iterator
        let last = &self.xmas[self.curr - XmasIterator::POOL_LEN];
        if let Some(freq) = self.previous.get(last) {
            match freq.cmp(&1) {
                Ordering::Equal => {
                    self.previous.remove(last);
                }
                Ordering::Greater => {
                    self.previous.entry(*last).and_modify(|freq| *freq -= 1);
                }
                Ordering::Less => {
                    unreachable!(
                        "Data corruption, frequency value in hashmap is zero or negative."
                    );
                }
            };
        }
        self.previous
            .entry(self.xmas[self.curr])
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
        self.curr += 1;

        // returns the current number and its summing elements
        Some((curr, sum))
    }
}
