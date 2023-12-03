use regex::Regex;
use std::collections::HashMap;

const COLORS: [&str; 3] = ["red", "green", "blue"];

fn parse_count(s: &str) -> i32 {
    let cube_re = Regex::new(r"([0-9]+).*").unwrap();
    let digit = cube_re.captures(s).unwrap().get(1).unwrap().as_str();
    digit.parse::<i32>().unwrap()
}

fn process_draw(draw: &str) -> bool {
    let cubes_re =
        Regex::new(r"(?P<red>[0-9]+ red)|(?P<green>[0-9]+ green)|(?P<blue>[0-9]+ blue),?").unwrap();

    let cubes = cubes_re.captures_iter(draw);
    let mut current_draw: HashMap<String, i32> = HashMap::new();
    for &color in COLORS.iter() {
        current_draw.insert(color.into(), 0);
    }
    for cube in cubes {
        for &color in COLORS.iter() {
            if cube.name(color).is_some() {
                current_draw.insert(
                    color.into(),
                    parse_count(cube.name(color).map_or("0", |m| m.as_str()))
                        + current_draw[color.into()],
                );
            }
        }
    }

    if current_draw["red"] > 12 || current_draw["green"] > 13 || current_draw["blue"] > 14 {
        return false;
    }
    true
}

fn process_game(line: &str) -> bool {
    let game_re = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    let draws_re = Regex::new(r"(?P<draw>(.*?);)").unwrap();
    let line = format!("{};", line); // Add a trailing semicolon to make the regex work
    let draw = game_re.captures(&line).unwrap().get(2).unwrap().as_str();
    for caps in draws_re.captures_iter(draw) {
        let draw = caps.name("draw").unwrap().as_str().trim();
        match process_draw(draw) {
            false => return false,
            _ => (),
        }
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_2_1.txt");

    let mut i = 0;
    let mut sum = 0;
    for line in input.lines() {
        i += 1;
        match process_game(line) {
            true => sum += i as i32,
            _ => (),
        }
    }
    println!("sum: {}", sum);
    assert_eq!(sum, 2239);
    Ok(())
}
