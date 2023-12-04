fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_4_1.txt");

    let mut number_of_cards = vec![1; input.lines().count()];

    for (idx, line) in input.lines().enumerate() {
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
        let mut score: usize = 0;

        for card in current.clone() {
            if winning.contains(&card) {
                score += 1;
            }
        }

        for _x in 1..(number_of_cards[idx] + 1) {
            for x in 1..(score + 1) {
                number_of_cards[idx + x] += 1;
            }
        }
    }

    let sum = number_of_cards.into_iter().reduce(|a, b| a + b).unwrap();
    println!("total: {:?}", sum);

    Ok(())
}
