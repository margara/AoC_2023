use std::{fs::File, io::{self, BufRead}};
use itertools::Itertools;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Kind {
    FIVE, FOUR, FULL, THREE, TWO, ONE, HIGH
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
    second_part: bool,
}

impl Hand {
    fn new(cards: &str, bid: usize, second_part: bool) -> Self {
        Self { 
            cards: cards.chars().map(|i| {
                if i.is_ascii_digit() {
                    i.to_string().parse::<u8>().unwrap()
                } else {
                    match i {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("unknown card")
                    }
                }
            }).collect(),
            bid,
            second_part,
        }
    }

    fn kind(&self) -> Kind {
        let counts = self.cards.iter()
            .counts()
            .values()
            .sorted()
            .rev()
            .cloned()
            .collect::<Vec<_>>();

        let first = counts[0];
        match first {
            5 => Kind::FIVE,
            4 => Kind::FOUR,
            3 => {
                let second = counts[1];
                match second {
                    2 => Kind::FULL,
                    1 => Kind::THREE,
                    _ => panic!("Invalid hand"),
                }
            },
            2 => {
                let second = counts[1];
                match second {
                    2 => Kind::TWO,
                    1 => Kind::ONE,
                    _ => panic!("Invalid hand")
                }
            },
            1 => Kind::HIGH,
            _ => panic!("Invalid hand")
        }
    }

    fn kind_with_joker(&self) -> Kind {
        let mut counts = self.cards.iter().counts();
        let max_no_j = self.cards.iter().filter(|c| **c != 11).counts().values().cloned().max();
        if counts.contains_key(&11) && max_no_j.is_some() {
            let j_val = *counts.get(&11).unwrap();
            let keys = counts.keys().cloned().collect_vec();
            for k in keys.into_iter() {
                if *k != 11 && *counts.get(k).unwrap() == max_no_j.unwrap() {
                    *counts.get_mut(k).unwrap() += j_val;
                    break;
                }
            }
            *counts.get_mut(&11).unwrap() = 0;
        }
        
        let counts = counts
            .values()
            .sorted()
            .rev()
            .cloned()
            .collect::<Vec<_>>();

        let first = counts[0];
        match first {
            5 => Kind::FIVE,
            4 => Kind::FOUR,
            3 => {
                let second = counts[1];
                match second {
                    2 => Kind::FULL,
                    1 => Kind::THREE,
                    _ => panic!("Invalid hand"),
                }
            },
            2 => {
                let second = counts[1];
                match second {
                    2 => Kind::TWO,
                    1 => Kind::ONE,
                    _ => panic!("Invalid hand")
                }
            },
            1 => Kind::HIGH,
            _ => panic!("Invalid hand")
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_kind = if self.second_part { self.kind_with_joker() } else { self.kind() };
        let other_kind = if other.second_part { other.kind_with_joker() } else { other.kind() };

        let cards = if self.second_part {
            self.cards.iter().map(|a| {
                if *a == 11 { 1 } else { *a }
            }).collect::<Vec<_>>()
        } else {
            self.cards.clone()
        };

        let other_cards = if self.second_part {
            other.cards.iter().map(|a| {
                if *a == 11 { 1 } else { *a }
            }).collect::<Vec<_>>()
        } else {
            other.cards.clone()
        };

        if self_kind > other_kind {
            std::cmp::Ordering::Less
        } else if self_kind < other_kind {
            std::cmp::Ordering::Greater
        } else if cards[0] < other_cards[0] {
            std::cmp::Ordering::Less
        } else if cards[0] > other_cards[0] {
            std::cmp::Ordering::Greater
        } else if cards[1] < other_cards[1] {
            std::cmp::Ordering::Less
        } else if cards[1] > other_cards[1] {
            std::cmp::Ordering::Greater
        } else if cards[2] < other_cards[2] {
            std::cmp::Ordering::Less
        } else if cards[2] > other_cards[2] {
            std::cmp::Ordering::Greater
        } else if cards[3] < other_cards[3] {
            std::cmp::Ordering::Less
        } else if cards[3] > other_cards[3] {
            std::cmp::Ordering::Greater
        } else if cards[4] < other_cards[4] {
            std::cmp::Ordering::Less
        } else if cards[4] > other_cards[4] {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

fn parse_input(second_part: bool) -> Vec<Hand> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let cards = line.next().unwrap();
        let bid = line.next().unwrap().parse::<usize>().unwrap();
        Hand::new(cards, bid, second_part)
    }).collect_vec()
}

fn main() {
    // First part
    let input = parse_input(false);
    let hands = input.iter().sorted().collect_vec();
    let mut res = 0;
    for rank in 0..hands.len() {
        res = res + (rank+1) * hands[rank].bid
    }
    println!("Result: {}", res);

    // Second part
    let input = parse_input(true);
    let hands = input.iter().sorted().collect_vec();
    let mut res = 0;
    for rank in 0..hands.len() {
        res = res + (rank+1) * hands[rank].bid
    }
    println!("Result: {}", res);
}
