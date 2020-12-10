use std::fs::File;
use std::io;
use std::io::prelude::*;

//--------------------------------------------------------------------
// Read Input
//--------------------------------------------------------------------
pub fn read_to_str(input: &str) -> Result<String, io::Error> {
    let mut file = File::open("inputs/".to_string() + input).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
