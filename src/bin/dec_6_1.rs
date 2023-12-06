fn compute_distance(warm_up: i32, total_time: i32) -> i32 {
    let running_time = total_time - warm_up;
    return running_time * warm_up;
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = include_str!("../input/dec_6_1.txt").lines();
    let times = input.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;
    let distances = input.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    let mut total: u64 = 1u64;
    for race in 0..times.len() {
        let mut ways_to_win = 0;
        for i in 1..(times[race] + 1) {
            let distance = compute_distance(i, times[race]);
            if distance > distances[race] {
                ways_to_win += 1;
            }
        }
        total *= ways_to_win as u64;
    }
    println!("total: {}", total);
    Ok(())
}
