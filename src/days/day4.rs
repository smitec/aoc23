use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::{fs::File, io::Read};

pub fn day4() -> Result<()> {
    let mut file = File::open("./input/day4.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut score_total = 0;
    let mut card_matches: Vec<u32> = Vec::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            break;
        }

        let mut linec = line.split(": ");
        let _card_info = linec.next().unwrap();
        let game_info = linec.next().unwrap();

        let mut game_parts = game_info.split('|');

        let winners = game_parts.next().unwrap();
        let chances = game_parts.next().unwrap();

        let mut winning_numbers: BTreeSet<u32> = BTreeSet::new();
        for winner in winners.split_whitespace() {
            let winner_val = winner.parse::<u32>().unwrap();
            winning_numbers.insert(winner_val);
        }

        let mut matches = 0;
        for chances in chances.split_whitespace() {
            let chances_val = chances.parse::<u32>().unwrap();
            if winning_numbers.contains(&chances_val) {
                matches += 1;
            }
        }

        card_matches.push(matches);

        if matches > 0 {
            score_total += 2_u32.pow(matches - 1);
        }
    }

    // Part 2
    // Reverse the list. Work out how many points the last card gets (doesn't matter, its none) +
    // store it
    // Jump up one card, for N cards after, get the stored value of winners, sum them, store it
    // Repeat until at the top of the list
    let mut running_matches: Vec<u32> = Vec::new();
    for (i, match_count) in card_matches.iter().rev().enumerate() {
        let mut running_v = 0;
        let mut j = i as i32 - 1;
        let mut c = 0;
        while (j >= 0) && c < *match_count {
            running_v += running_matches[j as usize];
            j -= 1;
            c += 1;
        }
        running_matches.push(running_v + match_count);
    }

    let tot = running_matches
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap_or(0);

    println!("{:?} {:?}", score_total, tot + card_matches.len() as u32);

    Ok(())
}
