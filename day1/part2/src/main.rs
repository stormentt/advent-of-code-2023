#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

lazy_static! {
    static ref RE_FORWARD  : Regex = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    static ref RE_BACKWARD : Regex = Regex::new(r"[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
}


fn reverse(s: String) -> String {
    return s.chars().rev().collect::<String>();
}

fn main() -> io::Result<()> {
    let input = File::open("input")?;
    let reader = BufReader::new(input);

    let mut total = 0;
    for line in reader.lines() {
        total += scan(line?);
    }

    println!("Total: {}", total);

    Ok(())
}


fn scan(s: String) -> i32 {
    let mut first = String::new();
    let mut last = String::new();

    for v in RE_FORWARD.find_iter(s.as_str()).map(|m| String::from(m.as_str())) {
        first = v;
        break
    }

    let reversed = reverse(s);
    for v in RE_BACKWARD.find_iter(reversed.as_str()).map(|m| String::from(m.as_str())) {
        last = reverse(v);
        break
    }

    let first = parse(first);
    let last = parse(last);

    let combined = format!("{}{}", first, last);
    return combined.parse().unwrap();

}

fn parse(s: String) -> i32 {
    if s.len() == 1 {
        return s.parse().expect("expected digit, got something else");
    }

    match s.as_str() {
        "one"   => 1,
        "two"   => 2,
        "three" => 3,
        "four"  => 4,
        "five"  => 5,
        "six"   => 6,
        "seven" => 7,
        "eight" => 8,
        "nine"  => 9,
        _ => panic!("unexpected string in parse(): {}", s),
    }
}
