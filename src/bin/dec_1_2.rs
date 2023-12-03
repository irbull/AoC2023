use regex::Regex;

fn parse(s: &str) -> i32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse::<i32>().unwrap(),
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_1_1.txt");
    let text_digits = "|one|two|three|four|five|six|seven|eight|nine";
    let first_digit_re_str = format!(".*?([0-9]{}).*", text_digits); // non-greedy
    let last_digit_re_str = format!(".*([0-9]{}).*", text_digits); // greedy

    let first_digit_re = Regex::new(&first_digit_re_str).unwrap(); // non-greedy
    let last_digit_re = Regex::new(&last_digit_re_str).unwrap(); // greedy
    let mut sum: i32 = 0;
    for input_line in input.lines() {
        let first_digit = parse(
            first_digit_re
                .captures(input_line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str(),
        );
        let last_digit = parse(
            last_digit_re
                .captures(input_line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str(),
        );
        let value = first_digit * 10 + last_digit;
        sum += value;
    }
    println!("sum: {}", sum);
    Ok(())
}
