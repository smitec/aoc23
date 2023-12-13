use anyhow::{Context, Result};
use std::collections::HashMap;
use std::iter::zip;
use std::{fs::File, io::Read};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandCategory {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: String,
    category: HandCategory,
    bid: i32,
}

fn hand_to_category(hand: String) -> HandCategory {
    let mut bit_count: HashMap<char, i32> = HashMap::new();
    for c in hand.chars() {
        if bit_count.contains_key(&c) {
            let new_count = bit_count.get(&c).unwrap() + 1;
            bit_count.insert(c, new_count);
        } else {
            bit_count.insert(c, 1);
        }
    }
    let j_count: i32 = *bit_count.get(&'J').unwrap_or(&0);
    bit_count.insert('J', 0);
    let mut vals: Vec<i32> = bit_count.values().copied().collect();
    vals.sort();
    vals.reverse();

    if vals[0] + j_count == 5 {
        return HandCategory::FiveOfAKind;
    } else if vals[0] + j_count == 4 {
        return HandCategory::FourOfAKind;
    } else if vals[0] + j_count == 3 {
        if vals[1] == 2 {
            return HandCategory::FullHouse;
        } else {
            return HandCategory::ThreeOfAKind;
        }
    } else if vals[0] + j_count == 2 {
        if vals[1] == 2 {
            return HandCategory::TwoPair;
        } else {
            return HandCategory::OnePair;
        }
    }

    HandCategory::HighCard
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn card_rank(a: char) -> usize {
    // Part b J is lowest
    let ranked = "J23456789TQKA";
    ranked.find(a).unwrap_or(99)
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.category == other.category {
            // Compare Card
            let left_cards: Vec<char> = self.cards.chars().collect();
            let right_cards: Vec<char> = other.cards.chars().collect();
            for (a, b) in zip(left_cards, right_cards) {
                if a == b {
                    continue;
                }
                return card_rank(b).cmp(&card_rank(a));
            }
            std::cmp::Ordering::Equal
        } else {
            self.category.cmp(&other.category)
        }
    }
}

pub fn day7() -> Result<()> {
    let mut file = File::open("./input/day7.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut hands: Vec<Hand> = Vec::new();

    for line in contents.split('\n') {
        if line.is_empty() {
            break;
        }

        let parts: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        assert!(parts.len() == 2);
        let hand = parts[0].clone();
        let bid = parts[1].parse::<i32>().context("Couldn't parse bid")?;
        let hand = Hand {
            cards: hand.clone(),
            category: hand_to_category(hand),
            bid,
        };
        hands.push(hand);
    }

    hands.sort();

    let mut points = 0;

    for (i, v) in hands.iter().rev().enumerate() {
        points += (i as i32 + 1) * v.bid;
    }

    println!("total ranked points: {:?}", points);

    Ok(())
}
