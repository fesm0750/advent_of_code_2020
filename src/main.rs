use adv20::day01;
use adv20::day02;
use std::env;

fn main() {
    let input = env::args().nth(1);
    if input.is_none() {
        println!("No input argument.")
    }
    match input.unwrap().parse().unwrap() {
        1 => day01::run(),
        2 => day02::run(),
        _ => println!("Invalid input argument."),
    }
}
