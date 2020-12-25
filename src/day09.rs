use std::{cmp::Ordering, collections::HashMap, iter::FromIterator};

use crate::helpers::read;

#[derive(Default)]
struct XmasBreaker {
    numbers: Vec<i32>,
    summing_pool: HashMap<i32, u8>, /* hashmap of allowable values to compose next number and
                                     * their frequencies */
    curr: usize,
}

/// failing parsing a number may result in the iterator prematurely ending
fn parse_input(input_str: &str) -> XmasBreaker {
    input_str.lines().map(str::parse::<i32>).flatten().collect()
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    println!("Day 09");
    let input_str = read::read_to_str("day09").unwrap();
    let mut xmas = parse_input(&input_str);
    println!(
        "First number, not in the preambule, that is not a sum of two numbers before it: {}",
        xmas.find_weakness().unwrap()
    );
}

//--------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------

impl XmasBreaker {
    const POOL_LEN: usize = 25; // preambule size

    // Tries to advance the `curr` value and update the summing pool. Returns false
    // if `curr` is already the last value, otherwise returns true indicating the
    // `curr`ent value has been updated.
    fn advance(&mut self) -> bool {
        if self.curr >= self.numbers.len() - 1 {
            false
        } else {
            let last = &self.numbers[self.curr - XmasBreaker::POOL_LEN];
            if let Some(freq) = self.summing_pool.get(last) {
                match freq.cmp(&1) {
                    Ordering::Equal => {
                        self.summing_pool.remove(last);
                    }
                    Ordering::Greater => {
                        self.summing_pool.entry(*last).and_modify(|freq| *freq -= 1);
                    }
                    Ordering::Less => {
                        unreachable!(
                            "Data corruption, frequency value in hashmap is zero or negative."
                        );
                    }
                };
            }
            self.summing_pool
                .entry(self.numbers[self.curr])
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
            self.curr += 1;
            true
        }
    }

    // Returs an optional containing a tuple of the current number and another
    // option for the pair of values that sum to it. If no valid pair has been
    // found, the inner option will be a None value.
    fn get_sum_curr(&self) -> Option<(i32, Option<(i32, i32)>)> {
        if self.curr >= self.numbers.len() {
            return None;
        }

        let curr = self.numbers[self.curr];
        let previous = &self.numbers[self.curr - XmasBreaker::POOL_LEN..self.curr];
        for &n in previous {
            let complement = curr - n;
            if complement != n && self.summing_pool.contains_key(&complement) {
                return Some((curr, Some((n, complement))));
            }
        }
        Some((curr, None))
    }

    // Returns the first value that is not a sum of the previous 25 ones, excluding
    // the preambule.
    fn find_weakness(&mut self) -> Option<i32> {
        while let Some((curr, sum)) = self.get_sum_curr() {
            if sum.is_none() {
                return Some(curr);
            }
            self.advance();
        }
        None
    }
}

impl FromIterator<i32> for XmasBreaker {
    /// panics if the input iterator does not have the minimum preambule lenght.
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut ret = XmasBreaker::default();
        ret.numbers = iter.into_iter().collect();
        if ret.numbers.len() <= XmasBreaker::POOL_LEN {
            panic!("Data is too small for the Xmas encryption");
        }
        let preambule = ret.numbers.iter().take(XmasBreaker::POOL_LEN).copied();
        for p in preambule {
            ret.summing_pool
                .entry(p)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        }
        ret.curr = XmasBreaker::POOL_LEN; // current element is the one after preambule
        ret
    }
}
