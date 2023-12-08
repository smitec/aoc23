use anyhow::{Context, Result};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::{fs::File, io::Read};

struct RangeMap {
    source: String,
    dest: String,
    lower_source: i32,
    lower_dest: i32,
    range: i32,
}

fn day5() -> Result<()> {
    // Step 1, parse the input file into useful data types.
    let seeds: Vec<i32> = Vec::new();
    let maps: HashMap<String, RangeMap> = HashMap::new();

    let mut file = File::open("./input/day5small.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    Ok(())
}

fn day4() -> Result<()> {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, Debug)]
struct PartNumber {
    start: Coordinate,
    end: Coordinate,
    value: u32,
}

fn day3() -> Result<()> {
    let mut file = File::open("./input/day3.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    // Rough Plan
    // Create a list of part numbers that store the row and column of the first and last digit
    // Create a HashMap of Symbols based on their row and column
    // Define a function has neighbor which takes a start and end coordinate pair and the HashMap
    // of symbols, returning true of any symbols are in the 8 way neighborhood

    let mut parts: HashMap<Coordinate, PartNumber> = HashMap::new();
    let mut symbols: HashSet<Coordinate> = HashSet::new();
    let mut gears: HashSet<Coordinate> = HashSet::new();

    // Step 1 : Parse
    let mut digit_placeholder = 0;
    let mut start_location = Coordinate { row: 0, col: 0 };
    for (row, line) in contents.split('\n').enumerate() {
        for (col, current) in line.chars().enumerate() {
            if current.is_ascii_digit() {
                if digit_placeholder == 0 {
                    start_location = Coordinate { row, col };
                }
                digit_placeholder *= 10;
                digit_placeholder += current.to_digit(10).unwrap();
            } else {
                if digit_placeholder > 0 {
                    // Create a new Part Number
                    parts.insert(
                        start_location,
                        PartNumber {
                            start: start_location,
                            end: Coordinate { row, col: col - 1 },
                            value: digit_placeholder,
                        },
                    );
                    digit_placeholder = 0;
                }

                if current == '.' {
                    continue;
                }

                if current == '*' {
                    gears.insert(Coordinate { row, col });
                }

                symbols.insert(Coordinate { row, col });
            }
        }

        if digit_placeholder > 0 {
            // Create a new Part Number
            parts.insert(
                start_location,
                PartNumber {
                    start: start_location,
                    end: Coordinate {
                        row,
                        col: line.len() - 1,
                    },
                    value: digit_placeholder,
                },
            );
            digit_placeholder = 0;
        }
    }

    // Step 2 : Check
    let mut part_id_count = 0;
    let mut gear_sum = 0;
    let mut gear_parts: HashMap<Coordinate, Vec<PartNumber>> = HashMap::new();
    for part in parts.values() {
        let mut skip = false;

        let row_min = if part.start.row > 0 {
            part.start.row - 1
        } else {
            0
        };

        let row_max = part.end.row + 1;

        for row in row_min..=row_max {
            if skip {
                break;
            }

            let col_min = if part.start.col > 0 {
                part.start.col - 1
            } else {
                0
            };

            let col_max = part.end.col + 1;

            for col in col_min..=col_max {
                if symbols.get(&Coordinate { row, col }).is_some() {
                    if gears.get(&Coordinate { row, col }).is_some() {
                        if gear_parts.get(&Coordinate { row, col }).is_none() {
                            gear_parts.insert(Coordinate { row, col }, Vec::new());
                        }

                        let current_parts = gear_parts.get_mut(&Coordinate { row, col }).unwrap();
                        current_parts.push(*part);
                    }
                    part_id_count += part.value;
                    skip = true;
                    break;
                }
            }
        }
    }

    for gp in gear_parts.values() {
        if gp.len() == 2 {
            let first = gp.first().unwrap().value;
            let last = gp.last().unwrap().value;
            gear_sum += first * last;
        }
    }

    println!("{:?}, {:?}", part_id_count, gear_sum);

    Ok(())
}

fn day2() -> Result<()> {
    let mut file = File::open("./input/day2.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;
    let max_balls = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut id_total = 0;
    let mut power_total = 0;

    for line in contents.as_str().split('\n') {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(": ");
        let first = parts.next().unwrap(); // Game x
        let game_id = first
            .replace("Game ", "")
            .parse::<i32>()
            .context("Couldn't parse game id")?;

        let last = parts.next().unwrap(); // the data
        let mut game_ok = true;
        let mut game_max = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for hand in last.split("; ") {
            for portion in hand.split(", ") {
                let mut portion_parts = portion.split(' ');
                let ball_count = portion_parts
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .context("Couldn't parse ball count")?;
                let color = portion_parts.next().unwrap();
                let comp = *max_balls.get(color).unwrap();
                let comp_game = *game_max.get(color).unwrap();
                if ball_count > comp_game {
                    game_max.insert(color, ball_count);
                }

                if ball_count > comp {
                    game_ok = false;
                }
            }
        }

        let mut game_power = 1;
        for v in game_max.into_values() {
            game_power *= v;
        }
        power_total += game_power;

        if game_ok {
            id_total += game_id;
        }
    }

    println!("{:?}, {:?}", id_total, power_total);

    Ok(())
}

fn main() {
    println!("Day 2");
    match day2() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 3");
    match day3() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 4");
    match day4() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
}
