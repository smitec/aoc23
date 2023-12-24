use anyhow::{Context, Result};

use std::{fs::File, io::Read};

#[derive(Debug)]
struct Pattern {
    original: String,
    checksum: Vec<usize>,
}

pub fn day12() -> Result<()> {
    let mut file = File::open("./input/day12small.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut patterns: Vec<Pattern> = Vec::new();

    // Parse
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        // Line is pattern_checksum
        let (pattern, checksum) = line.split_once(' ').context("Bad line")?;
        let checksum_parsed: Vec<usize> = checksum
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        patterns.push(Pattern {
            original: format!("{}.", pattern),
            checksum: checksum_parsed,
        });
    }

    // Store a 2D array representing, the number of ways we can fill up to string index X when
    // constrained by checksup up to index Y
    let variants = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        println!("{:?}/{:?} - {:?}", i, patterns.len(), variants);
        let rows = pattern.checksum.len();
        let cols = pattern.original.len();

        let mut storage: Vec<Vec<usize>> = Vec::new();
        for _ in 0..rows {
            storage.push(vec![0; cols]);
        }

        let mut checksum_count: usize = 0;

        let mut offset = 0;

        while checksum_count < pattern.checksum.len() {
            let check_digit = pattern.checksum[checksum_count];
            println!("Check Digit {:?}", check_digit);
            let mut next_offset = offset;
            let mut found = false;
            let mut escaped = false;
            println!("{:?}, {:?}", check_digit + offset, cols);
            for char_count in offset + check_digit..cols {
                // If there are . in this chunk, there's no space.
                let chunk = pattern.original[char_count - check_digit..char_count].to_string();

                let left = if char_count > 0 {
                    storage[checksum_count][char_count - 1]
                } else {
                    0
                };
                let down = if checksum_count > 0 {
                    storage[checksum_count - 1][char_count]
                } else {
                    0
                };

                let previous = left.max(down);
                if escaped {
                    println!("Escapred, using previous {} {} {}", previous, left, down);
                    storage[checksum_count][char_count] = previous;
                    continue;
                }

                println!(
                    "chunk {:?} previous {:?} checksum {:?} at offset {}",
                    chunk, previous, check_digit, char_count
                );
                if chunk.contains('.') {
                    storage[checksum_count][char_count] = previous;
                    if !found {
                        // Haven't found a solution yet. Grab from below (or 0 if on first digit)
                        println!("Found a leading .");
                    } else {
                        // Found a dot so time to move on
                        println!("Found a . matched before so move on.");
                        escaped = true;
                        continue;
                    }
                } else {
                    // Ok if the next spot is a . or a ?
                    let next = pattern.original.chars().nth(char_count).unwrap();
                    if next == '.' || next == '?' {
                        if !found {
                            println!("First Match for this Checksum {}", char_count);
                            next_offset = char_count - check_digit + 2;
                        }
                        println!("Space after, valid option");
                        storage[checksum_count][char_count] = previous + 1;
                        found = true;
                        if !chunk.contains('?') {
                            println!("Perfect Fit, stop here.");
                            // TODO: maybe hard set this to just use down + 1
                            next_offset = char_count + 1;
                            escaped = true;
                            continue;
                        }
                    } else {
                        println!("Nope, no space after. {}", next);
                        storage[checksum_count][char_count] = previous;
                    }
                }
            }
            offset = next_offset;
            checksum_count += 1;
        }
        for v in storage {
            println!("{:?}", v);
        }
    }

    println!("{:?}", variants);

    Ok(())
}
