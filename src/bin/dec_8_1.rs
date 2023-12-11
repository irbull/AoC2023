use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map_re = Regex::new(r"(?P<name>[A-Z]+) = \((?P<left>[A-Z]+), (?P<right>[A-Z]+)\)").unwrap();

    let mut input = include_str!("../input/dec_8_1.txt").lines();
    let operations = input.next().unwrap();
    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::new();
    input.next(); // skip blank line
    for line in input {
        let current_mapping = map_re.captures(line).unwrap();
        let name = current_mapping.name("name").unwrap().as_str();
        let left = current_mapping.name("left").unwrap().as_str();
        let right = current_mapping.name("right").unwrap().as_str();
        mappings.insert(name, (left, right));
    }
    let mut current = mappings.get("AAA").unwrap();
    let mut found_end = false;
    let mut count = 0;
    while found_end == false {
        for op in operations.chars() {
            count += 1;
            match op {
                'L' => {
                    let left = mappings.get(current.0).unwrap();
                    if current.0 == "ZZZ" {
                        found_end = true;
                        break;
                    }
                    current = left;
                }
                'R' => {
                    let right = mappings.get(current.1).unwrap();
                    if current.1 == "ZZZ" {
                        found_end = true;
                        break;
                    }
                    current = right;
                }
                _ => (),
            }
        }
    }
    println!("count: {}", count);
    Ok(())
}
