//! Day 05
//!
//! # Problem
//!
//! From an input file containing containing the encoded identification numbers
//! of boarding passes:
//!
//! 1. find the largest id;
//!
//! 2. find an empty seat. An empty seat is a seat not present in the input list
//! and should be in between two occupied ones.

// TODO: the input string is actually a binary number where the 0's and 1's are
// encoded by letters. A bitwise shifting approach on a accumulator may be a
// faster and better way to implement the decoding.

// Todo: compute the id while parsing and save it inside the BoardingPass struct

use std::{error::Error, str::FromStr};

use crate::helpers::read;

/// Returns the highest id from an array slice of `BoardingPass`es
///
/// # Assumptions
///
/// `tickets` is not empty.
fn highest_id(tickets: &[BoardingPass]) -> u32 {
    tickets.iter().map(|bp| bp.id()).max().unwrap()
}

/// Returns the id of an empty seat in between two occupied ones.
fn find_seat(tickets: &[BoardingPass]) -> Option<u32> {
    let mut seats: Vec<u32> = tickets.iter().map(BoardingPass::id).collect();
    seats.sort_unstable();
    let mut seats = seats.into_iter().peekable();
    while let Some(seat) = seats.next() {
        let peek = seats.peek().copied();
        if peek == Some(seat + 2) {
            return Some(seat + 1);
        }
    }
    None
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    let input = read::to_str("day05").unwrap();
    let tickets = read::lines_into_vec(&input);
    println!("Day 05");
    println!("Highest id: {}", highest_id(&tickets));
    println!("My seat: {}", find_seat(&tickets).unwrap());
}

//--------------------------------------------------------------------
// BoardingPass struct
//--------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
struct BoardingPass {
    pub row: u32,
    pub col: u32,
}

impl BoardingPass {
    // using usize for convinience, those values should be pretty small
    const N_CHAR_ROWS: usize = 7;
    const N_CHAR_COLS: usize = 3;

    fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }

    fn id(&self) -> u32 {
        8 * self.row + self.col
    }

    fn validate_string(ticket: &str) -> Option<(&str, &str)> {
        if ticket.len() != BoardingPass::N_CHAR_ROWS + BoardingPass::N_CHAR_COLS {
            return None;
        }

        let row = &ticket[0..BoardingPass::N_CHAR_ROWS];
        let col = &ticket[BoardingPass::N_CHAR_ROWS..];

        let is_valid_row = row.chars().all(|c| c == 'B' || c == 'F');
        let is_valid_col = col.chars().all(|c| c == 'L' || c == 'R');

        if is_valid_row && is_valid_col {
            Some((row, col))
        } else {
            None
        }
    }

    fn decode_ticket(code: (&str, &str)) -> BoardingPass {
        BoardingPass::new(decode_bsp(code.0, 'F', 'B'), decode_bsp(code.1, 'L', 'R'))
    }
}

impl FromStr for BoardingPass {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bp =
            BoardingPass::validate_string(s).ok_or("Unable to parse string. Invalid input.")?;
        Ok(BoardingPass::decode_ticket(bp))
    }
}

//--------------------------------------------------------------------
// Helper Functions
//--------------------------------------------------------------------

/// Returns the integer value encoded in the string `code` using a search like
/// algorithm for the binary partitioning.
///
/// # Assumptions
///
/// `code` is a ASCII string.
///
/// # Panics
///
/// - if lenght of the string `code` is longer than 32 characteres;
/// - if `code` is an invalid string.
fn decode_bsp(code: &str, lower_char: char, upper_char: char) -> u32 {
    const MAX_CHARS: usize = 32;
    assert!(
        code.len() <= MAX_CHARS,
        "Input argument `code` is too long."
    );
    let lower_char = lower_char as u8;
    let upper_char = upper_char as u8;
    let mut lo = 0u32;
    let mut hi = 2u32.pow(code.len() as u32) - 1;
    let chars = code.as_bytes().iter();
    for &c in chars {
        if c == lower_char {
            hi = (hi - lo) / 2 + lo;
        } else if c == upper_char {
            lo = (hi - lo + 1) / 2 + lo;
        } else {
            panic!("Invalid character in input argument `code`.");
        }
    }
    assert_eq!(lo, hi, "Unable to decode.");
    lo
}

//--------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";

    #[test]
    fn test_validate_ticket() {
        // valid inputs
        assert_eq!(
            BoardingPass::validate_string("BFFFBBFRRR"),
            Some(("BFFFBBF", "RRR"))
        );
        assert_eq!(
            BoardingPass::validate_string("FFFBBBFRRR"),
            Some(("FFFBBBF", "RRR"))
        );
        assert_eq!(
            BoardingPass::validate_string("BBFFBBFRLL"),
            Some(("BBFFBBF", "RLL"))
        );

        // invalid
        assert_eq!(BoardingPass::validate_string("BFFFBBFRR"), None); // len = 9
        assert_eq!(BoardingPass::validate_string("BFFFBBFRRRR"), None); // len = 11
        assert_eq!(BoardingPass::validate_string("BFFFBBFFRR"), None); // wrong column char ('F')
        assert_eq!(BoardingPass::validate_string("BFFFBBFRBR"), None); // wrong column char ('B')
        assert_eq!(BoardingPass::validate_string("BFFFBBRRRR"), None); // wrong row char ('R')
        assert_eq!(BoardingPass::validate_string("BFFLBBFRRR"), None); // wrong row char ('L')
        assert_eq!(BoardingPass::validate_string("BFFABBFRRR"), None); // wrong char ('A')
        assert_eq!(BoardingPass::validate_string("BFFFBBFRCR"), None); // wrong char ('C')
    }

    #[test]
    fn test_decode_bsp() {
        assert_eq!(decode_bsp("FBFBBFF", 'F', 'B'), 44);
        assert_eq!(decode_bsp("RLR", 'L', 'R'), 5);
    }

    #[test]
    fn test_parse_input() {
        let out = vec![
            BoardingPass::new(70, 7),
            BoardingPass::new(14, 7),
            BoardingPass::new(102, 4),
        ];
        assert_eq!(read::lines_into_vec::<BoardingPass>(INPUT_STR), out);
    }

    #[test]
    fn test_highest_id() {
        let tickets = read::lines_into_vec::<BoardingPass>(INPUT_STR);
        assert_eq!(highest_id(&tickets), 820);
    }

    // todo: add a test for find_seat()
}
