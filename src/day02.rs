//! Day 02
//!
//! # Problem:
//!
//! From an input containing a list of passwords and their validation
//! parameters:
//!
//! 1. Count how many valid password there are according to the old policy;
//!
//! 2. Count the number of valid passwords according to the new policy.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::helpers;

//--------------------------------------------------------------------
// Password Data Structure
//--------------------------------------------------------------------

/// A struct to store a password and their validation parameters.
///
/// # Assumptions
///
/// - the pass `String` does not contain more than 255 repetitions of the same
///   character.
#[derive(Debug, PartialEq, Eq)]
pub struct PasswordRecord {
    ch: char,
    lower: u8,
    upper: u8,
    pass: String,
}

impl PasswordRecord {
    /// Checks whether or not a password is valid according to the old policy.
    pub fn is_valid_old(&self) -> bool {
        let count = self.pass.chars().filter(|&c| c == self.ch).count() as u8;
        self.lower <= count && count <= self.upper
    }

    /// Checks whether or not a password is valid according to the new policy.
    ///
    /// # Panics
    ///
    /// panics if `min` or `max` does not correlate to valid indexes for the
    /// string pass.
    pub fn is_valid_new(&self) -> bool {
        let ch = self.ch;
        let mut chars = self.pass.chars();
        let pos1 = (self.lower - 1) as usize;
        let pos2 = (self.upper - self.lower - 1) as usize;
        let c1 = chars.nth(pos1).expect("Invalid index.");
        let c2 = chars.nth(pos2).expect("Invalid index.");
        (c1 == ch) ^ (c2 == ch)
    }
}

impl FromStr for PasswordRecord {
    type Err = Box<dyn Error>;

    /// Converts a string slice into a struct of the type `PasswordRecord`.
    ///
    /// # Arguments
    ///
    /// `s` is of the pattern "min-max ch: pass", where the fields min and
    /// max are `u8`, ch is a `char` and pass is a substring.
    ///
    /// - An example is: "2-9 c: ccccccccc"
    ///
    /// # Assumptions
    ///
    /// String slice `s` holds well structured data.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let mut bounds = iter.next().ok_or(ParseDay02Error)?.split('-');
        let min = bounds.next().ok_or(ParseDay02Error)?.parse::<u8>()?;
        let max = bounds.next().ok_or(ParseDay02Error)?.parse::<u8>()?;
        let letter = iter
            .next()
            .ok_or(ParseDay02Error)?
            .chars()
            .next()
            .ok_or(ParseDay02Error)?;
        let pass = iter.next().ok_or(ParseDay02Error)?.to_owned();
        Ok(PasswordRecord {
            ch: letter,
            lower: min,
            upper: max,
            pass,
        })
    }
}

#[derive(Debug)]
struct ParseDay02Error;

impl Error for ParseDay02Error {}

impl fmt::Display for ParseDay02Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while parsing Day02 input. Malformed input data.")
    }
}

//--------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------

/// Parses a string containing the input data and returns a `vec` of
/// `Password`s.
/// `input` - a string slice holding password records with entries separated by
/// the new line character.
pub fn parse_input(input: &str) -> Vec<PasswordRecord> {
    input
        .lines()
        .map(|line| {
            line.parse::<PasswordRecord>()
                .expect("Invalid input record.")
        })
        .collect()
}

/// returns the total of valid passwords inside the array slice `passwords`,
/// according to the old policy.
pub fn count_valid_old(passwords: &[PasswordRecord]) -> usize {
    passwords.iter().filter(|p| p.is_valid_old()).count()
}

/// returns the total of valid passwords inside the array slice `passwords`,
/// according to the new policy.
pub fn count_valid_new(passwords: &[PasswordRecord]) -> usize {
    passwords.iter().filter(|p| p.is_valid_new()).count()
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    let str = helpers::read_to_str("day02").unwrap();
    let passwords = parse_input(&str);
    println!("Day 02");
    println!(
        "Valid password count on old policy: {}",
        count_valid_old(&passwords)
    );
    println!(
        "Valid password count on new policy: {}",
        count_valid_new(&passwords)
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

    lazy_static! {
        static ref P1: PasswordRecord = PasswordRecord {
            lower: 1,
            upper: 3,
            ch: 'a',
            pass: "abcde".to_owned(),
        };
        static ref P2: PasswordRecord = PasswordRecord {
            lower: 1,
            upper: 3,
            ch: 'b',
            pass: "cdefg".to_owned(),
        };
        static ref P3: PasswordRecord = PasswordRecord {
            lower: 2,
            upper: 9,
            ch: 'c',
            pass: "ccccccccc".to_owned(),
        };
    }

    #[test]
    fn test_passport_from_str() {
        let s1 = "1-3 a: abcde";
        let s2 = "1-3 b: cdefg";
        let s3 = "2-9 c: ccccccccc";
        assert_eq!(s1.parse::<PasswordRecord>().unwrap(), *P1);
        assert_eq!(s2.parse::<PasswordRecord>().unwrap(), *P2);
        assert_eq!(s3.parse::<PasswordRecord>().unwrap(), *P3);
    }

    #[test]
    fn test_is_valid_old() {
        assert!(P1.is_valid_old());
        assert!(!P2.is_valid_old());
        assert!(P3.is_valid_old());
    }

    #[test]
    fn test_is_valid_new() {
        assert!(P1.is_valid_new());
        assert!(!P2.is_valid_new());
        assert!(!P3.is_valid_new());
    }
}
