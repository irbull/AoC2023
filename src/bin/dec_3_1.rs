use regex::Regex;
use std::cmp::min;

fn is_symbol(c: char) -> bool {
    match c {
        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => false,
        _ => true,
    }
}

fn check_overlap(start: usize, end: usize, positions: &Vec<char>) -> bool {
    let start = match start {
        start if start > 0 => start - 1,
        _ => start,
    };
    let range = &positions[start..min(positions.len() - 1, end + 1)];
    for c in range {
        if is_symbol(*c) {
            return true;
        }
    }
    false
}

fn connected(start: usize, end: usize, row: usize, positions: &Vec<Vec<char>>) -> bool {
    if check_overlap(start, end, &positions[row]) {
        return true;
    }
    if row > 0 && check_overlap(start, end, &positions[row - 1]) {
        return true;
    }
    if row < positions.len() - 1 && check_overlap(start, end, &positions[row + 1]) {
        return true;
    }
    return false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_3_1.txt");
    let digits = Regex::new(r"(?P<digit>[0-9]+)*").unwrap();

    let positions: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;
    for (row, line) in input.lines().enumerate() {
        for caps in digits.captures_iter(line) {
            match caps.name("digit") {
                Some(digit) => {
                    if connected(digit.start(), digit.end(), row, &positions) {
                        println!("{}{:?}{}", row, digit, "connected horizontally");
                        sum += digit.as_str().parse::<i32>()?;
                    }
                }
                None => {}
            }
        }
    }

    println!("sum: {}", sum);

    Ok(())
}
