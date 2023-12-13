use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

const EXPANSION: i32 = 1000000;

fn has_h_galaxy(h_line: &str) -> bool {
    h_line
        .chars()
        .filter(|c| *c == '#')
        .collect::<Vec<char>>()
        .len()
        > 0
}

struct Distance(Vec<Vec<i32>>);

impl Display for Distance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(f, "{:?} ", col)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_11_1.txt").lines();
    let mut universe: Vec<Vec<i32>> = vec![];
    let mut distances: Distance = Distance(vec![]);

    let mut count = 0;
    for line in input {
        let mut row = vec![];
        if has_h_galaxy(line) {
            distances.0.push(vec![1; line.len()]);
        } else {
            distances.0.push(vec![EXPANSION; line.len()]);
        }
        line.chars().for_each(|s| {
            if s == '#' {
                count += 1;
                row.push(count);
            } else {
                row.push(0);
            }
        });
        universe.push(row);
    }

    for col in 0..universe[0].len() {
        let mut galaxy_found = false;
        for row in 0..universe.len() {
            if universe[row][col] != 0 {
                galaxy_found = true;
                break;
            }
        }
        for row in 0..universe.len() {
            if !galaxy_found {
                distances.0[row][col] = EXPANSION;
            }
        }
    }

    let mut locations: HashMap<i32, (i32, i32)> = HashMap::new();
    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] != 0 {
                locations.insert(universe[row][col], (row as i32, col as i32));
            }
        }
    }

    let mut total_distance = 0u64;
    for i in 1..(count + 1) {
        for j in (i + 1)..(count + 1) {
            let mut current_distance = 0;
            let (x1, y1) = locations.get(&i).unwrap();
            let (x2, y2) = locations.get(&j).unwrap();

            // Negative ranges are not supported in Rust, so we look at ranges
            // in both the positive and negative directions.
            for x in *x1..*x2 {
                current_distance += distances.0[x as usize][*y1 as usize];
            }
            for x in *x2..*x1 {
                current_distance += distances.0[x as usize][*y1 as usize];
            }
            for y in *y1..*y2 {
                current_distance += distances.0[*x2 as usize][y as usize];
            }
            for y in *y2..*y1 {
                current_distance += distances.0[*x2 as usize][y as usize];
            }
            total_distance += current_distance as u64;
        }
    }
    println!("total distance: {:?}", total_distance);
    Ok(())
}
