extern crate test;

use crate::helpers::read;

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

/// Parses the input file and adds the output and device ports.
pub fn parse_input(input: &str) -> Vec<u8> {
    let mut out = Vec::<u8>::new();
    out.push(0); // charging outlet
    out.extend(read::parsed_lines_iter::<u8>(input));
    out.sort_unstable();
    out.push(out.last().unwrap() + 3); //device adapter
    out
}

pub fn run() {
    let input = read::to_str("day10").unwrap();
    let joltages: Vec<u8> = parse_input(&input);
    if !is_valid(&joltages) {
        panic!("Failed to connect all adaptors");
    }
    println!("Day 10");
    println!(
        "Number of 1 jolt differences times 3 jolts differences = {}",
        checksum_diffs(&joltages)
    );
    println!("Number of distinct ways: {}", count_arrangements(&joltages));
}

//--------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------

/// Checks whether the input is valid or not. The difference between two
/// neighbour elements must be at most 3.
///
/// # Assumptions
///
/// `joltages` is sorted.
fn is_valid(joltages: &[u8]) -> bool {
    joltages.array_windows().all(|&[a, b]| b - a <= 3)
}

/// from a list of adapters joltages checks the joltages differences and returns
/// a check sum consisting in the number of diferences of one times the number
/// of differences of three.
///
/// # Assumptions
///
/// `joltages` is sorted.
fn checksum_diffs(adapters: &[u8]) -> usize {
    let (ones, threes) = adapters
        .array_windows()
        .fold((0, 0), |(ones, threes), &[a, b]| match b - a {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => unreachable!(),
        });
    ones * threes
}

/// Counts the total number of distinct ways to arrange the adapters.
///
/// # Assumptions
///
/// - `adapters` is sorted;
///
/// # Implementation Details
///
/// - Walks the `adapters` array slice in reversed order computing the number of
/// ways to get from any adapter to the final one using dynamic programming;
///
/// - The last adapter has one way to get to itself;
///
/// - One adapter can only connect to another one if the joltage difference is
///   at most `MAX_DELTA` (3 jolts).
fn count_arrangements(adapters: &[u8]) -> u64 {
    if adapters.is_empty() {
        return 0;
    }

    const MAX_DELTA: u8 = 3; // maximum allowable joltage variation

    let mut cache: Vec<u64> = vec![0; adapters.len()];
    *cache.last_mut().unwrap() = 1;

    for (i, &jolt) in adapters.iter().enumerate().rev().skip(1) {
        (i + 1..adapters.len())
            .take_while(|&j| adapters[j] <= jolt + MAX_DELTA)
            .for_each(|j| cache[i] += cache[j]);
    }

    cache[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use test::Bencher;

    const INVALID_STR: &str = "16\n10\n15\n5\n1\n11\n10\n19\n6\n12\n4"; // 4 diff between 6 and 10
    const INPUT_STR1: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
    const INPUT_STR2: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

    lazy_static! {
        static ref PARSED: Vec<u8> =
            parse_input(&read::to_str("day10").expect("Unable to read file."));
        static ref INPUT_INVALID: Vec<u8> = parse_input(INVALID_STR);
        static ref INPUT1: Vec<u8> = parse_input(INPUT_STR1);
        static ref INPUT2: Vec<u8> = parse_input(INPUT_STR2);
    }

    #[test]
    fn test_is_valid() {
        assert!(!is_valid(&INPUT_INVALID));
        assert!(is_valid(&INPUT1));
        assert!(is_valid(&INPUT2));
    }

    #[test]
    fn test_checksum_diffs() {
        assert_eq!(checksum_diffs(&INPUT1), 35);
        assert_eq!(checksum_diffs(&INPUT2), 220);
    }

    #[test]
    fn test_count_arrangements() {
        assert_eq!(count_arrangements(&INPUT1), 8);
        assert_eq!(count_arrangements(&INPUT2), 19208);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(INPUT_STR2));
    }

    #[bench]
    fn bench_parse_input2(b: &mut Bencher) {
        b.iter(|| parse_input(&read::to_str("day10").unwrap()));
    }

    #[bench]
    fn bench_checksum_diffs(b: &mut Bencher) {
        b.iter(|| checksum_diffs(&PARSED));
    }

    #[bench]
    fn bench_count_paths(b: &mut Bencher) {
        b.iter(|| count_arrangements(&PARSED));
    }
}
