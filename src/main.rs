use adv20::day01;
use adv20::helpers;

fn main() {
    let s1 = helpers::read_to_str("day01").unwrap();
    let in1 = day01::parse_input(&s1);
    let v1 = day01::puzzle01a(&in1, 2020).unwrap();
    println!("{}", v1); // 902451
    let v2 = day01::puzzle01b(&in1, 2020).unwrap();
    println!("{}", v2);
}
