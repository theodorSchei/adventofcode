use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Formatter},
    fs,
};

fn main() {
    dbg!(part_1("input.txt"));
    dbg!(part_2("input.txt"));
}
struct Hand {
    cards: Vec<char>,
    bid: u32,
    hand_type: HandType,
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} : {: >4} - {:?}",
            self.cards.iter().join(""),
            self.bid,
            self.hand_type
        )
    }
}

impl Hand {
    fn from_str(input: &str) -> Hand {
        let mut parts = input.split_whitespace();
        let cards: Vec<char> = parts.next().unwrap().chars().collect();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();
        let hand_type = HandType::from_cards(&cards.clone());
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
    // Function to map a card character to its value
    fn card_value(card: &char) -> u8 {
        match card {
            'X' => 0, // 'X' is a joker
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        }
    }

    // Compare two hands
    fn cmp(&self, other: &Hand) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            for i in 0..5 {
                let cmp = Hand::card_value(&self.cards[i]).cmp(&Hand::card_value(&other.cards[i]));
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &Vec<char>) -> HandType {
        let mut counts = HashMap::new();
        for card in cards {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }

        // Handle jokers
        let jokers = counts.remove(&'X').unwrap_or(0);

        // Clone the counts into a new Vec<u32> and then sort
        let mut counts: Vec<u32> = counts.values().cloned().collect();
        counts.sort_by(|a, b| b.cmp(a));

        // Add jokers to the most frequent card
        if !counts.is_empty() {
            counts[0] += jokers;
        } else {
            counts.push(jokers);
        }

        dbg!(&counts);
        match counts.as_slice() {
            [1, ..] => HandType::HighCard,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            [3, 2] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }

    fn cmp(&self, hand_type: &HandType) -> std::cmp::Ordering {
        match (self, hand_type) {
            (HandType::HighCard, HandType::HighCard) => std::cmp::Ordering::Equal,
            (HandType::HighCard, _) => std::cmp::Ordering::Less,
            (_, HandType::HighCard) => std::cmp::Ordering::Greater,
            (HandType::OnePair, HandType::OnePair) => std::cmp::Ordering::Equal,
            (HandType::OnePair, _) => std::cmp::Ordering::Less,
            (_, HandType::OnePair) => std::cmp::Ordering::Greater,
            (HandType::TwoPair, HandType::TwoPair) => std::cmp::Ordering::Equal,
            (HandType::TwoPair, _) => std::cmp::Ordering::Less,
            (_, HandType::TwoPair) => std::cmp::Ordering::Greater,
            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => std::cmp::Ordering::Equal,
            (HandType::ThreeOfAKind, _) => std::cmp::Ordering::Less,
            (_, HandType::ThreeOfAKind) => std::cmp::Ordering::Greater,
            (HandType::FullHouse, HandType::FullHouse) => std::cmp::Ordering::Equal,
            (HandType::FullHouse, _) => std::cmp::Ordering::Less,
            (_, HandType::FullHouse) => std::cmp::Ordering::Greater,
            (HandType::FourOfAKind, HandType::FourOfAKind) => std::cmp::Ordering::Equal,
            (HandType::FourOfAKind, _) => std::cmp::Ordering::Less,
            (_, HandType::FourOfAKind) => std::cmp::Ordering::Greater,
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => std::cmp::Ordering::Equal,
        }
    }
}

fn part_1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut hands = input.lines().map(Hand::from_str).collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.cmp(b));

    dbg!(&hands);

    // Multiply the bid with the hand rank
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            println!("{} * {:?}", i + 1, hand.bid);
            hand.bid * (i as u32 + 1)
        })
        .sum::<u32>()
}

fn part_2(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut hands = input
        .lines()
        .map(|line| {
            let replaced = line.replace('J', "X");
            Hand::from_str(replaced.as_str())
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.cmp(b));

    dbg!(&hands);

    // Multiply the bid with the hand rank
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            println!("{} * {:?}", i + 1, hand.bid);
            hand.bid * (i as u32 + 1)
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("example.txt");
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("example.txt");
        assert_eq!(result, 5905);
    }
}
