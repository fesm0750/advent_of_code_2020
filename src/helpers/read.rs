// todo: implement tests

use std::{fs::File, io, io::prelude::*, str::FromStr};

//--------------------------------------------------------------------
// Read Input
//--------------------------------------------------------------------

/// reads the whole file into a String.
pub fn to_str(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open("inputs/".to_string() + filename).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

//--------------------------------------------------------------------
// Commom Parsing
//--------------------------------------------------------------------

//------------------------------
// Entries are single lines
//------------------------------

/// parses an `input` where each line is an entry into a `Vec`.
pub fn lines_into_vec<T: FromStr>(input: &str) -> Vec<T> {
    input.lines().map(str::parse::<T>).flatten().collect()
}

/// parses the `input` and sorts the entries. Each line in string slice is a new
/// entry. Uses `sort_unstable`.
pub fn lines_into_sorted<T>(input: &str) -> Vec<T>
where
    T: FromStr + Ord,
{
    let mut out: Vec<T> = lines_into_vec(input);
    out.sort_unstable();
    out
}

/// returns an iterator over parsed values of an `input` string slice where the
/// entries are separated by a new line.
pub fn parsed_lines_iter<'a, T>(input: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
{
    input.lines().map(str::parse::<T>).flatten()
}

//------------------------------
// Entries are separated by custom characters
//------------------------------

/// parses an `input` into a `Vec<T>`. Entries in the string slice are
/// separated by the `split_at` characters.
pub fn split_into_vec<T>(input: &str, split_at: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .split(split_at)
        .map(str::parse::<T>)
        .flatten()
        .collect()
}

/// parses an `input` into a sorted `Vec<T>`. Entries are separated by the
/// `split_at` characters.
pub fn split_into_sorted<T>(input: &str, split_at: &str) -> Vec<T>
where
    T: FromStr + Ord,
{
    let mut out = split_into_vec(input, split_at);
    out.sort_unstable();
    out
}

/// returns an iterator over parsed values of an `input` string where the
/// entries are separated by the `split_at` characters.
pub fn parsed_split_iter<'a, T>(input: &'a str, split_at: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
{
    input.split(split_at).map(str::parse::<T>).flatten()
}
