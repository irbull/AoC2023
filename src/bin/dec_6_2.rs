fn compute_distance(warm_up: u64, total_time: u64) -> u64 {
    let running_time = total_time - warm_up;
    return running_time * warm_up;
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = include_str!("../input/dec_6_1.txt").lines();
    let time = input.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .to_string()
        .replace(" ", "")
        .parse::<i32>()?;

    let distance = input.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .to_string()
        .replace(" ", "")
        .parse::<i64>()?;

    let mut ways_to_win = 0;
    for i in 1..(time + 1) {
        let current_distance = compute_distance(i as u64, time as u64);
        if current_distance as i64 > distance {
            ways_to_win += 1;
        }
    }
    println!("total: {}", ways_to_win);
    assert_eq!(ways_to_win, 24655068);
    Ok(())
}
