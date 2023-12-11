fn difference_vec(seq: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..seq.len() - 1 {
        result.push(seq[i + 1] - seq[i]);
    }
    result
}

fn is_zeros(seq: &Vec<i32>) -> bool {
    for i in seq {
        if *i != 0 {
            return false;
        }
    }
    true
}

fn solve_seq(seq: Vec<i32>) -> i32 {
    let mut sol_space: Vec<Vec<i32>> = vec![];
    let mut last: Vec<i32> = seq.clone();
    while !is_zeros(&last) {
        sol_space.insert(0, last.clone());
        last = difference_vec(last.clone());
    }
    sol_space.insert(0, last.clone());
    sol_space[0].insert(0, 0);
    for i in 1..sol_space.len() {
        let last_prev_row = sol_space[i - 1].first().unwrap().to_owned();
        let cur_row = &mut sol_space[i];
        cur_row.insert(0, cur_row.first().unwrap() - last_prev_row);
    }
    sol_space.last().unwrap().first().unwrap().to_owned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_9_1.txt").lines();

    let mut total = 0;
    for line in input {
        let seq = line
            .split_whitespace()
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let next_num = solve_seq(seq);
        total += next_num;
    }
    println!("total: {}", total);

    Ok(())
}
