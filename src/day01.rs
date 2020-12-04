//! Day 01
//!
//! ## Problem:
//! From an input file containig unsigned integer values:
//! 1. find the two entries that sum to 2020 and multiply those numbers together;
//! 2. find three entries that sum to 2020 and multiply them together.

/// Parses a string containing the input data and returns a sorted array.
/// input should be a list of values separated by the new line character. If a value is negative or cannot be parsed, a value of zero is assigned.
pub fn parse_input(input: &str) -> Vec<u32> {
    let mut out: Vec<u32> = input
        .lines()
        .map(|l| if let Ok(x) = l.parse::<u32>() { x } else { 0 })
        .collect();
    out.sort();
    out
}

/// Tries to find a pair that sums to target by the two-pointer technique (left and right cursors).
/// If values are found, it returns their multiplication. Otherwise, returns None.
/// ## Assumptions
/// the input array must be sorted.
pub fn puzzle01a(sorted: &[u32], target: u32) -> Option<u32> {
    let mut left = 0;
    let mut right = sorted.len() - 1;

    while right > left {
        let sum = sorted[left] + sorted[right];
        if sum == target {
            return Some(sorted[left] * sorted[right]);
        }
        if sum > target {
            right -= 1;
        }
        if sum < target {
            left += 1;
        }
    }
    return None;
}

/// Tries to find a triplet that sums to target integer value.
/// If values are found, it returns their multiplication. Otherwise, returns None.
/// ## Assumptions
/// the input array must be sorted.
pub fn puzzle01b(sorted: &[u32], target: u32) -> Option<u32> {
    for v in sorted {
        if *v > target {
            break;
        }
        if let Some(x) = puzzle01a(sorted, target - v) {
            return Some(v * x);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_puzzle01a() {
        let list = vec![299, 366, 675, 979, 1456, 1721];
        assert_eq!(puzzle01a(&list, 2020).unwrap(), 514579);
    }

    #[test]
    fn test_puzzle01a_2() {
        let list = vec![299, 366, 675, 979, 1456, 1721];
        assert_eq!(puzzle01a(&list, 1041).unwrap(), 247050);
    }

    #[test]
    fn test_puzzle01b() {
        let list = vec![299, 366, 675, 979, 1456, 1721];
        assert_eq!(puzzle01b(&list, 2020).unwrap(), 241861950);
    }
}
