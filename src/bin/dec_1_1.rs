use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_1_1.txt");
    let first_digit_re = Regex::new(".*?([0-9]).*").unwrap(); // non-greedy
    let last_digit_re = Regex::new(".*([0-9]).*").unwrap(); // greedy
    let mut sum = 0;
    for input_line in input.lines() {
        let first_digit = first_digit_re
            .captures(input_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()?;
        let last_digit = last_digit_re
            .captures(input_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()?;
        println!("{}:{}", first_digit, last_digit);
        let value = first_digit * 10 + last_digit;
        sum += value;
    }
    println!("sum: {}", sum);
    Ok(())
}
