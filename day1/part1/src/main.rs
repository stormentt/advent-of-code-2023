use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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
    let mut first = ' ';
    let mut last = ' ';

    for c in s.chars() {
        if !c.is_digit(10) {continue}
        if first == ' ' {
            first = c;
        }
        last = c;
    }

    let combined = format!("{}{}", first, last);
    return combined.parse::<i32>().unwrap();
}
