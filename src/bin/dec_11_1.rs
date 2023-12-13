use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_11_1.txt").lines();
    let mut universe: Vec<Vec<i32>> = vec![];

    let mut count = 0;
    for line in input {
        let mut row = vec![];
        let mut galaxy_found = false;
        line.chars().for_each(|s| {
            if s == '#' {
                count += 1;
                row.push(count);
                galaxy_found = true;
            } else {
                row.push(0);
            }
        });
        universe.push(row.clone());
        if !galaxy_found {
            universe.push(row.clone());
        }
    }

    let mut col = 0;
    loop {
        let mut galaxy_found = false;
        for row in 0..universe.len() {
            if universe[row][col] != 0 {
                galaxy_found = true;
                break;
            }
        }
        if !galaxy_found {
            for row in 0..universe.len() {
                universe[row].insert(col, 0);
            }
            col += 1;
        }
        col += 1;
        if col >= universe[0].len() {
            break;
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

    let mut total_distance = 0;
    for i in 1..(count + 1) {
        for j in (i + 1)..(count + 1) {
            let (x1, y1) = locations.get(&i).unwrap();
            let (x2, y2) = locations.get(&j).unwrap();
            let distance = (*x1 - *x2).abs() + (*y1 - *y2).abs();
            total_distance += distance;
        }
    }

    println!("total distance: {:?}", total_distance);

    Ok(())
}
