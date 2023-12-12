use anyhow::{Context, Result};
use bio::data_structures::interval_tree::Entry;
use bio::data_structures::interval_tree::IntervalTree;
use bio::utils::Interval;
use num::integer::lcm;
use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::zip;
use std::{fs::File, io::Read};

struct Node {
    left: String,
    right: String,
}

fn check_positions(positions: &[usize]) -> bool {
    for s in positions.iter() {
        if *s == 0 {
            return false;
        }
    }
    true
}

fn day8() -> Result<()> {
    let mut file = File::open("./input/day8.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut nodes: HashMap<String, Node> = HashMap::new();

    // first line is instructions, then blank, then node format
    let mut lines: Vec<String> = contents.split('\n').map(|x| x.to_string()).collect();
    assert!(lines.len() > 1);

    let directions = lines[0].to_string();
    lines = lines[1..].to_vec();

    let re = Regex::new(r"^(?<loc>[A-Z0-9]+) = \((?<left>[A-Z0-9]+), (?<right>[A-Z0-9]+)\)$")
        .context("Couldn't setup regex")?;
    let mut starters: Vec<String> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let matches = re
            .captures(line.as_str())
            .context(format!("Line did not match regex {:?}", line))?;
        let loc = matches["loc"].to_string();
        let left = matches["left"].to_string();
        let right = matches["right"].to_string();

        if loc.ends_with('A') {
            starters.push(loc.clone());
        }

        nodes.insert(loc, Node { left, right });
    }

    let mut c = 0;
    //let mut current_loc = "AAA".to_string();
    println!("{:?} Starting Positions", starters.len());
    println!("{:?}", starters);
    let mut freq: Vec<usize> = vec![0; starters.len()];
    while !check_positions(&freq) {
        let mut next_locations: Vec<String> = Vec::new();
        let i = c % directions.len();
        let d = directions.chars().nth(i).context("Ran out of direcitons")?;
        for (i, s) in starters.iter().enumerate() {
            let node = nodes
                .get(s)
                .context(format!("No node for current position {:?}", s))?;
            let next = if d == 'L' {
                node.left.to_string()
            } else {
                node.right.to_string()
            };
            if next.ends_with('Z') && freq[i] == 0 {
                freq[i] = c + 1;
            }
            next_locations.push(next);
        }
        c += 1;
        starters = next_locations;
    }

    println!("{:?}", freq);
    let mut v = freq[0] as u128;
    for i in 1..starters.len() {
        v = lcm(v, freq[i] as u128);
        println!("{:?}", v);
    }

    println!("{:?}", v);

    Ok(())
}

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

fn day7() -> Result<()> {
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

fn day6() -> Result<()> {
    let mut file = File::open("./input/day6b.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let lines: Vec<String> = contents.split('\n').map(|x| x.to_string()).collect();
    assert!(lines.len() == 3);

    let times = lines[0].replace("Time:", "");
    let distances = lines[1].replace("Distance:", "");

    let times_vals: Vec<f64> = times
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let distance_vals: Vec<f64> = distances
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let mut tot = 1_f64;
    for (time, distance) in zip(times_vals, distance_vals) {
        // Find the first time in which (time - x)*x > distance
        // Speed = x
        // distance = (time - x)*x = time*x - x*x
        // solving for distance > 0 is time*x - x*x - distance > 0
        // quad formual is x = -b +/- sqrt(b**2 - 4*a*c) / 2*a
        // where a = -1, b = time, c = -distance
        // giving x = -time +/- sqrt(time*time - 4*-1*distance) / 2*-1
        let a = -1_f64;
        let b = time;
        let c = -distance;

        let root1 = (-b + (b.powi(2) - 4_f64 * a * c).sqrt()) / (2_f64 * a) + 1e-6;
        let root2 = (-b - (b.powi(2) - 4_f64 * a * c).sqrt()) / (2_f64 * a) - 1e-6;

        let r1_c = root1.ceil();
        let r2_f = root2.floor();

        let delta = r2_f - r1_c + 1.0;

        tot *= delta;
    }

    println!("{:?}", tot);

    Ok(())
}

#[derive(Clone, Debug)]
struct RangeMap {
    lower_source: i64,
    lower_dest: i64,
    range: i64,
}

struct TreeDest {
    dest: String,
    tree: IntervalTree<i64, RangeMap>,
}

fn day5() -> Result<()> {
    // Step 1, parse the input file into useful data types.
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: HashMap<String, TreeDest> = HashMap::new();

    let mut file = File::open("./input/day5.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut map_mode = false;
    let mut source = "".to_string();
    let mut dest = "".to_string();
    let mut mappings: IntervalTree<i64, RangeMap> = IntervalTree::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            if map_mode {
                // Clean up the map
                maps.insert(
                    source.clone(),
                    TreeDest {
                        dest: dest.clone(),
                        tree: mappings.clone(),
                    },
                );
                map_mode = false;
            }
            continue;
        }

        if line.starts_with("seeds:") {
            //parse the seeds
            let seed_numbers = line.replace("seeds: ", "");
            for seed_number in seed_numbers.split_whitespace() {
                let seed_as_int = seed_number.parse::<i64>().context("Could not parse int")?;
                seeds.push(seed_as_int);
            }
        } else if line.contains("map:") {
            // Start parsing in a new map
            map_mode = true;
            let re = Regex::new(r"^(?<source>[a-z]+)-to-(?<dest>[a-z]+) map:$")
                .context("Couldn't setup regex")?;
            let captures = re.captures(line).context("Map line did not match regex")?;
            source = captures["source"].to_string();
            dest = captures["dest"].to_string();
            mappings = IntervalTree::new();
        } else if map_mode {
            let re = Regex::new(r"^(?<deststart>[0-9]+) (?<sourcestart>[0-9]+) (?<range>[0-9]+)$")
                .context("Couldn't setup range parse regex")?;
            let captures = re
                .captures(line)
                .context(format!("Map Data line did not match regex: {:?}", line))?;
            let lower_source = captures["sourcestart"]
                .parse::<i64>()
                .context("Couldn't parse lower source")?;
            let range = captures["range"]
                .parse::<i64>()
                .context("Couldn't parse lower dest")?;
            let this_map = RangeMap {
                lower_source,
                lower_dest: captures["deststart"]
                    .parse::<i64>()
                    .context("Couldn't parse lower dest")?,
                range,
            };
            mappings.insert(lower_source..lower_source + range, this_map.clone());
        }
    }

    // For part 2, map the seeds into the new range format.
    let mut first = 0;
    let mut seed_ranges: Vec<Interval<i64>> = Vec::new();
    for (i, s) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            first = *s;
        } else {
            seed_ranges.push(
                Interval::new(first..first + *s).context("Couldn't form interval from seeds")?,
            );
        }
    }
    // Loop through the seed locations
    let mut current_stage = "seed".to_string();
    let mut current_seeds = seed_ranges.clone();
    while current_stage != *"location" {
        let mut next_seeds: Vec<Interval<i64>> = Vec::new();
        let mapper = maps
            .get(current_stage.as_str())
            .context(format!("No maps for stage {:?}", current_stage))?;
        current_stage = mapper.dest.clone();
        for seed in current_seeds.iter() {
            let mut current_pt = seed.start;
            let mut matches: Vec<Entry<'_, i64, RangeMap>> =
                mapper.tree.find(seed.clone()).collect();
            matches.sort_by_key(|x| x.interval().start);
            for matched_range in matches.iter() {
                if matched_range.interval().start > current_pt {
                    // Add a range from current -> new start with no adjustment
                    next_seeds.push(
                        Interval::new(current_pt..matched_range.interval().start)
                            .context("Couldn't form new range from start to current_pt")?,
                    );
                    current_pt = matched_range.interval().start;
                }
                // Map the overlapping portion to a new range
                if matched_range.interval().end < seed.end {
                    // More of the range exists beyond the end of the matched range
                    let offset = current_pt - matched_range.data().lower_source;
                    let dest_start = matched_range.data().lower_dest;
                    let dest_length = matched_range.data().range;
                    next_seeds.push(
                        Interval::new(dest_start + offset..dest_start + dest_length).context(
                            "Couldn't form new range from dest_start to dest_start + length",
                        )?,
                    );
                    current_pt = matched_range.interval().end;
                    // TODO: possible need to +1 here because its exclusive
                } else {
                    // The seed ends within this matched range
                    let offset = current_pt - matched_range.data().lower_source;
                    let dest_start = matched_range.data().lower_dest;
                    let dest_length = seed.end - current_pt;
                    next_seeds.push(
                        Interval::new(dest_start + offset..dest_start + offset + dest_length)
                            .context("Couldn't form new range from dest_start to end of seed")?,
                    );
                    current_pt = seed.end;
                }
            }

            if seed.end > current_pt {
                // Add a range from current -> end with no adjustment
                next_seeds.push(
                    Interval::new(current_pt..seed.end)
                        .context("Couldn't form new range with remaining")?,
                );
            }
        }
        current_seeds = next_seeds;
    }
    let mut min_match = -1;
    for seed in current_seeds {
        if min_match == -1 || seed.start < min_match {
            min_match = seed.start;
        }
    }
    println!("Part B min: {:?}", min_match);

    // Find a path for each seed to its location
    let mut min_loc = -1;
    for seed in seeds.iter() {
        let mut current_stage = "seed".to_string();
        let mut current_value = *seed;
        while current_stage != *"location" {
            let mapper = maps
                .get(current_stage.as_str())
                .context(format!("No maps for stage {:?}", current_stage))?;
            let dest_stage = mapper.dest.clone();
            for map in mapper.tree.find(current_value..current_value + 1) {
                let data = map.data();
                if current_value >= data.lower_source
                    && current_value < data.lower_source + data.range
                {
                    current_value = data.lower_dest + (current_value - data.lower_source);
                    break;
                }
            }

            current_stage = dest_stage;
        }
        if min_loc == -1 || current_value < min_loc {
            min_loc = current_value;
        }
    }
    println!("Lowest location: {:?}", min_loc); // Part 1 should be 535088217

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
    println!("Day 5");
    match day5() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 6");
    match day6() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 7");
    match day7() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 8");
    match day8() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
}
