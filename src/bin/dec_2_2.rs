use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

const COLORS: [&str; 3] = ["red", "green", "blue"];

fn parse_count(s: &str) -> i32 {
    let cube_re = Regex::new(r"([0-9]+).*").unwrap();
    let digit = cube_re.captures(s).unwrap().get(1).unwrap().as_str();
    digit.parse::<i32>().unwrap()
}

fn process_draw(draw: &str) -> (i32, i32, i32) {
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

    (
        current_draw["red"],
        current_draw["green"],
        current_draw["blue"],
    )
}

fn process_game(line: &str) -> i32 {
    let game_re = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    let draws_re = Regex::new(r"(?P<draw>(.*?);)").unwrap();
    let line = format!("{};", line); // Add a trailing semicolon to make the regex work
    let draw = game_re.captures(&line).unwrap().get(2).unwrap().as_str();
    let mut min = (0, 0, 0);
    for caps in draws_re.captures_iter(draw) {
        let draw = caps.name("draw").unwrap().as_str().trim();
        let number_of_cubes = process_draw(draw);
        min.0 = max(min.0, number_of_cubes.0);
        min.1 = max(min.1, number_of_cubes.1);
        min.2 = max(min.2, number_of_cubes.2);
    }
    min.0 * min.1 * min.2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_2_1.txt");

    let mut sum = 0;
    for line in input.lines() {
        sum = sum + process_game(line);
    }
    println!("sum: {}", sum);
    // assert_eq!(sum, 2239);
    Ok(())
}
