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
        // // Sort cards by value
        // cards.sort_by(|a, b| {
        //     let card_values = [
        //         '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        //     ];
        //     card_values
        //         .iter()
        //         .rev()
        //         .position(|&c| c == *a)
        //         .cmp(&card_values.iter().rev().position(|&c| c == *b))
        // });
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
    fn compare(&self, other: &Hand) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            for i in 0..5 {
                let cmp = Hand::card_value(&self.cards[i]).cmp(&Hand::card_value(&other.cards[i]));
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            return Ordering::Equal;
        }

        // Separate key cards and kickers, and sort them
        let (self_keys, self_kickers) = self.sorted_key_and_kickers();
        let (other_keys, other_kickers) = other.sorted_key_and_kickers();

        // Compare key cards first
        for (self_key, other_key) in self_keys.iter().zip(other_keys.iter()) {
            let cmp = self_key.cmp(other_key);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        // If key cards are equal, compare kickers
        for (self_kicker, other_kicker) in self_kickers.iter().zip(other_kickers.iter()) {
            let cmp = self_kicker.cmp(other_kicker);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        Ordering::Equal
    }

    // Function to sort and separate key cards and kickers
    fn sorted_key_and_kickers(&self) -> (Vec<u8>, Vec<u8>) {
        let mut freq_map = HashMap::new();

        for card in &self.cards {
            let card_val = Hand::card_value(card);
            *freq_map.entry(card_val).or_insert(0) += 1;
        }

        let mut key_cards = Vec::new();
        let mut kickers = Vec::new();

        match self.hand_type {
            HandType::FiveOfAKind => {
                key_cards.extend(freq_map.keys());
            }
            HandType::FourOfAKind | HandType::FullHouse | HandType::ThreeOfAKind => {
                for (&card, &freq) in &freq_map {
                    if freq == 4
                        || (self.hand_type == HandType::FullHouse && freq == 3)
                        || (self.hand_type == HandType::ThreeOfAKind && freq == 3)
                    {
                        key_cards.push(card);
                    } else {
                        kickers.push(card);
                    }
                }
            }
            HandType::TwoPair | HandType::OnePair => {
                let mut pairs = Vec::new();
                for (&card, &freq) in &freq_map {
                    if freq == 2 {
                        pairs.push(card);
                    } else {
                        kickers.push(card);
                    }
                }
                pairs.sort_by(|a, b| b.cmp(a));
                key_cards.extend(pairs);
            }
            HandType::HighCard => {
                kickers.extend(freq_map.keys());
            }
        }

        key_cards.sort_by(|a, b| b.cmp(a));
        kickers.sort_by(|a, b| b.cmp(a));

        (key_cards, kickers)
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
        let counts = counts.values().sorted().rev().collect::<Vec<_>>();
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1] => HandType::OnePair,
            [2, 2, 1] => HandType::TwoPair,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [3, 2] => HandType::FullHouse,
            [4, 1] => HandType::FourOfAKind,
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

    hands.sort_by(|a, b| a.compare(b));

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

fn part_2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    todo!();
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
        assert_eq!(result, 71503);
    }
}
