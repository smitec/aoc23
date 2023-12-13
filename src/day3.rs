use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

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

pub fn day3() -> Result<()> {
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
