use regex::Regex;
use std::{cmp::min, collections::HashMap};

fn is_symbol(c: char) -> bool {
    match c {
        '*' => true,
        _ => false,
    }
}

fn check_overlap(
    start: usize,
    end: usize,
    row: usize,
    positions: &Vec<char>,
    operations: &mut HashMap<String, usize>,
    operand: usize,
    total: &mut usize,
) -> bool {
    let start = match start {
        start if start > 0 => start - 1,
        _ => start,
    };
    let range = &positions[start..min(positions.len() - 1, end + 1)];
    for (idx, c) in range.iter().enumerate() {
        if is_symbol(*c) {
            let key = format!("{}:{}", row, idx + start);
            if operations.contains_key(&key) {
                if operations[&key] == 0 {
                    panic!("{} x {} at {}", operations[&key], operand, key);
                }
                println!(
                    "{} x {} = {} at {}",
                    operations[&key],
                    operand,
                    operations[&key] * operand,
                    key
                );
                *total += operations[&key] * operand;
                operations.insert(key, 0);
            } else {
                operations.insert(key, operand);
            }

            return true;
        }
    }
    false
}

fn connected(
    start: usize,
    end: usize,
    row: usize,
    positions: &Vec<Vec<char>>,
    operations: &mut HashMap<String, usize>,
    operand: usize,
    total: &mut usize,
) -> bool {
    if check_overlap(start, end, row, &positions[row], operations, operand, total) {
        return true;
    }
    if row > 0
        && check_overlap(
            start,
            end,
            row - 1,
            &positions[row - 1],
            operations,
            operand,
            total,
        )
    {
        return true;
    }
    if row < positions.len() - 1
        && check_overlap(
            start,
            end,
            row + 1,
            &positions[row + 1],
            operations,
            operand,
            total,
        )
    {
        return true;
    }
    return false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_3_1.txt");
    let digits = Regex::new(r"(?P<digit>[0-9]+)*").unwrap();
    let operations: &mut HashMap<String, usize> = &mut HashMap::new();

    let positions: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total: usize = 0;
    for (row, line) in input.lines().enumerate() {
        for caps in digits.captures_iter(line) {
            match caps.name("digit") {
                Some(digit) => {
                    let operand = digit.as_str().parse::<usize>()?;
                    connected(
                        digit.start(),
                        digit.end(),
                        row,
                        &positions,
                        operations,
                        operand,
                        &mut total,
                    );
                }
                None => {}
            }
        }
    }

    println!("sum: {}", total);

    Ok(())
}
