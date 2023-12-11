use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialOrd, PartialEq, Eq, Debug, Ord)]
enum HandType {
    HighCard(),
    OnePair(),
    TwoPair(),
    ThreeOfAKind(),
    FullHouse(),
    FourOfAKind(),
    FiveOfAKind(),
}

#[derive(Debug, Eq)]
struct Hand {
    hand_type: HandType,
    cards: [char; 5],
    bid: i32,
}

fn map_to_order(c: char) -> char {
    match c {
        'J' => '0',
        '1' => '1',
        '2' => '2',
        '3' => '3',
        '4' => '4',
        '5' => '5',
        '6' => '6',
        '7' => '7',
        '8' => '8',
        '9' => '9',
        'T' => 'A',
        'Q' => 'C',
        'K' => 'D',
        'A' => 'E',
        _ => panic!("Invalid card rank: {}", c),
    }
}

fn compare_cards(c1: char, c2: char) -> Ordering {
    return map_to_order(c1).cmp(&map_to_order(c2));
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type.cmp(&other.hand_type).is_eq() {
            for idx in 0..5 {
                if compare_cards(self.cards[idx], other.cards[idx]).is_eq() {
                    continue;
                }
                return compare_cards(self.cards[idx], other.cards[idx]);
            }
            return Ordering::Equal;
        }
        return self.hand_type.cmp(&other.hand_type);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (&self.hand_type, &self.cards) == (&other.hand_type, &other.cards)
    }
}

fn count_jokers(cards: [char; 5]) -> u8 {
    let mut count = 0;
    for card in cards {
        if card == 'J' {
            count += 1;
        }
    }
    count
}

fn hand_type(cards: [char; 5]) -> HandType {
    let mut unique: HashMap<char, u8> = HashMap::new();
    for card in cards {
        match unique.get(&card) {
            Some(rank) => unique.insert(card, rank + 1),
            None => unique.insert(card, 1),
        };
    }

    match unique.len() {
        5 => match count_jokers(cards) {
            1 => return HandType::OnePair(), // High card + joker
            0 => HandType::HighCard(),
            _ => panic!("Invalid number of jokers [5]"),
        },
        4 => match count_jokers(cards) {
            //AJJTT
            1 => return HandType::ThreeOfAKind(),
            2 => return HandType::ThreeOfAKind(),
            0 => return HandType::OnePair(),
            _ => panic!("Invalid number of jokers [4]"),
        },
        3 => {
            match count_jokers(cards) {
                1 => match unique.values().max() {
                    Some(3) => return HandType::FourOfAKind(),
                    Some(2) => return HandType::FullHouse(),
                    _ => panic!("Invalid number hand [3.1]"),
                },
                2 => return HandType::FourOfAKind(),
                0 => match unique.values().max() {
                    Some(3) => return HandType::ThreeOfAKind(),
                    _ => return HandType::TwoPair(),
                },
                _ => panic!("Invalid number of jokers [3], {}", count_jokers(cards)),
            };
        }
        2 => {
            match count_jokers(cards) {
                1 => return HandType::FiveOfAKind(),
                2 => return HandType::FiveOfAKind(),
                3 => return HandType::FiveOfAKind(),
                4 => return HandType::FiveOfAKind(),
                0 => match unique.values().max() {
                    Some(4) => return HandType::FourOfAKind(),
                    _ => return HandType::FullHouse(),
                },
                _ => panic!("Invalid number of jokers [2] {}", count_jokers(cards)),
            };
        }
        1 => HandType::FiveOfAKind(),
        _ => panic!("Invalid hand type"),
    }
}

fn convert_to_char_array(v: Vec<char>) -> [char; 5] {
    v.try_into().unwrap_or_else(|v: Vec<char>| {
        panic!("Expected a Vec of length {} but it was {}", 5, v.len())
    })
}

impl Hand {
    fn new(cards: &str, bid: i32) -> Hand {
        let cards = convert_to_char_array(cards.chars().collect::<Vec<char>>());
        Hand {
            hand_type: hand_type(cards),
            cards: cards,
            bid,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input/dec_7_1.txt").lines();
    let mut hands: Vec<Hand> = vec![];
    for line in input {
        let (hand, bid) = line.split_whitespace().next_tuple().unwrap();
        hands.push(Hand::new(hand, bid.parse::<i32>()?));
    }
    hands.sort();
    let mut total = 0;
    for i in 0..hands.len() {
        total += (i + 1) * hands[i].bid as usize;
    }
    println!("total {}", total);
    Ok(())
}
