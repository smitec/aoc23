use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

struct State {
    stones: Vec<(usize, usize)>,
    first_seen: usize,
}

pub fn day14() -> std::io::Result<()> {
    let mut file = File::open("./input/day14.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let total_height = contents.lines().count() - 1;
    println!("Total Height: {}", total_height);
    // Stores the current highest empty space when tilted north
    let mut current_channels: Vec<usize> = Vec::new();
    let mut total_weight = 0;
    let mut blockers: HashSet<(usize, usize)> = HashSet::new();
    let mut stones: HashSet<(usize, usize)> = HashSet::new();
    let mut rows = 0;
    let mut cols = 0;

    for (lv, line) in contents.lines().enumerate() {
        if current_channels.is_empty() {
            current_channels = vec![total_height; line.len()];
            cols = line.len();
            rows = contents.lines().count() - 1;
        }
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    // Change the offset to lv + 1
                    // Do nothing else
                    current_channels[i] = total_height - lv - 1;
                    blockers.insert((lv, i)); // row, column
                }
                '.' => {
                    // Do nothing
                }
                'O' => {
                    // Roll the stote, add to the channel, add to the weight
                    total_weight += current_channels[i];
                    current_channels[i] -= 1;
                    stones.insert((lv, i)); // row, column
                }
                _ => {}
            }
        }
    }

    println!("Total weight when tilted: {}", total_weight);

    let rots = [(-1, 0), (0, -1), (1, 0), (0, 1)]; // Movement for N W S E
                                                   //TODO: Store the lowest index for the loop so I can work out the remainder
    let mut states: HashMap<Vec<(usize, usize)>, State> = HashMap::new();
    let mut first_loop = 0;
    let mut i = 0;
    while i < 1000000000 {
        let mut all_stones: Vec<(usize, usize)> = stones.iter().copied().collect();
        all_stones.sort();

        match states.get(&all_stones) {
            Some(x) => {
                if first_loop == 0 {
                    first_loop = i;
                    let first_seen = x.first_seen;
                    let loop_len = first_loop - first_seen;
                    let remainder = 1000000000 - i;
                    let to_go = remainder % loop_len;
                    println!(
                        "First Seen {:?} First Loop {:?} - Loop Len {:?} - Remainder {:?} - To Go {:?}",
                        first_seen, first_loop, loop_len, remainder, to_go
                    );
                    i = 1000000000 - to_go;
                }
                let mut next_stones: HashSet<(usize, usize)> = HashSet::new();
                for stone in x.stones.iter() {
                    next_stones.insert(*stone);
                }
                stones = next_stones;
                //            break; // Looks like it just loops one state
            }
            None => {
                for rot in rots {
                    let mut next_stones: HashSet<(usize, usize)> = HashSet::new();
                    let mut all_stones: Vec<(usize, usize)> = stones.iter().copied().collect();
                    all_stones.sort_by(|a, b| comp_stones(*a, *b, rot));
                    for stone in all_stones.iter() {
                        // Try find it a new home
                        let mut current = *stone;
                        loop {
                            let test_r = current.0 as i32 + rot.0;
                            let test_c = current.1 as i32 + rot.1;
                            if test_r < 0
                                || test_r >= rows as i32
                                || test_c < 0
                                || test_c >= cols as i32
                                || blockers.contains(&(test_r as usize, test_c as usize))
                                || next_stones.contains(&(test_r as usize, test_c as usize))
                            {
                                // Either out of bounds or something already there
                                break;
                            } else {
                                current = (test_r as usize, test_c as usize);
                            }
                        }
                        next_stones.insert(current);
                    }

                    /*
                    println!("Old State");
                    println!("{:?}", current_state.stones);
                    println!("New State");
                    println!("{:?}", next_state.stones);
                    break;
                    */
                    stones = next_stones;
                }
                let mut next_stones_v: Vec<(usize, usize)> = stones.iter().copied().collect();
                next_stones_v.sort();
                states.insert(
                    all_stones,
                    State {
                        stones: next_stones_v,
                        first_seen: i,
                    },
                );
            }
        }
        i += 1;
    }
    println!("Done");

    let items: Vec<(usize, usize)> = stones.iter().copied().collect();
    let mut total_weight = 0;
    for stone in items {
        total_weight += total_height - stone.0;
    }

    //println!("Stones: {:?}", stones);
    println!("Total Weight Post Cycle: {}", total_weight);

    Ok(())
}

fn comp_stones(a: (usize, usize), b: (usize, usize), rot: (i32, i32)) -> Ordering {
    match rot {
        (1, 0) => {
            if b.0 == a.0 {
                b.1.cmp(&a.1)
            } else {
                b.0.cmp(&a.0)
            }
        }
        (0, -1) => {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        }
        (-1, 0) => {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        }
        (0, 1) => {
            if b.1 == a.1 {
                b.0.cmp(&a.0)
            } else {
                b.1.cmp(&a.1)
            }
        }
        _ => Ordering::Equal,
    }
}
