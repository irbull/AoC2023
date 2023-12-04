fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_4_1.txt");

    let mut total = 0;
    for line in input.lines() {
        let game = line
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .trim();
        let cards = game.split("|").collect::<Vec<&str>>();
        let winning = cards
            .get(0)
            .unwrap()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|&card| card.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let current = cards
            .get(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|&card| card.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut score: i32 = 0;
        for card in current {
            if winning.contains(&card) {
                match score {
                    0 => score = 1,
                    _ => score *= 2,
                }
            }
        }
        total += score;
        println!("{:?}", score);
    }
    println!("total: {}", total);
    Ok(())
}
