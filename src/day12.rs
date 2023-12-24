use anyhow::{Context, Result};
use regex::Regex;
use std::{fmt::format, fs::File, io::Read};

#[derive(Debug)]
struct Pattern {
    original: String,
    checksum: Vec<usize>,
}

pub fn day12() -> Result<()> {
    let mut file = File::open("./input/day12.txt")?;
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
            original: pattern.to_string(),
            checksum: checksum_parsed,
        });
    }

    let mut variants = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        println!("{:?}/{:?} - {:?}", i, patterns.len(), variants);

        let mut possibilities: Vec<String> = Vec::new();
        let p = pattern.original.clone();
        let long_v = format!("{}?{}?{}?{}?{}.", p, p, p, p, p);
        possibilities.push(long_v);
        // Build the regex
        let mut re_loose = "^[.?]*".to_string();
        for _ in 0..5 {
            for bit in pattern.checksum.iter() {
                let next = format!("[#?]{{{}}}[.?]+", bit);
                re_loose = format!("{}{}", re_loose, next);
            }
        }
        re_loose = format!("{}$", re_loose);
        let regex = Regex::new(re_loose.as_str()).context("Malformed Regex")?;
        //println!("\n\n");
        while !possibilities.is_empty() {
            let current = possibilities.pop().context("No head")?;
            //println!("{}", current);
            let r = current.find('?');
            match r {
                Some(x) => {
                    // Determine the index by counting the hashes to the left of x
                    let left = current.split_at(x).0;
                    let mut hashes = left.chars().filter(|x| *x == '#').count() as i32;
                    let mut index = 0;
                    //println!("Hashes {:?} in {:?}, {:?}", hashes, left, pattern.checksum);
                    while hashes > 0 {
                        hashes -= pattern.checksum[index % pattern.checksum.len()] as i32;
                        index += 1;
                        //println!("Hashes {:?} index {:?}", hashes, index);
                    }
                    if hashes < 0 {
                        index -= 1;
                    }
                    // Try put the whole pattern here
                    let mut valid = true;
                    let mut to_replace = 0;
                    let t = current.as_str();
                    if index < pattern.checksum.len() * 5 {
                        index %= pattern.checksum.len();
                        let offset = pattern.checksum[index] as i32 + hashes;
                        //println!("{:?}", offset);
                        for v in x..x + offset as usize {
                            let c = t.chars().nth(v);
                            if c.is_none() {
                                valid = false;
                                break;
                            } else {
                                match c.unwrap() {
                                    '#' => {
                                        continue;
                                    }
                                    '?' => {
                                        to_replace += 1;
                                    }
                                    _ => {
                                        valid = false;
                                        break;
                                    }
                                }
                            }
                        }

                        if valid {
                            let c = current.replacen('?', "#", to_replace);
                            if regex.is_match(format!("{}.", c).as_str()) {
                                possibilities.push(c);
                            }
                        }
                    }

                    // Could still be a .
                    let c = current.replacen('?', ".", 1);
                    if regex.is_match(format!("{}.", c).as_str()) {
                        possibilities.push(c);
                    }
                }
                None => {
                    // Add a trailing . to simplify the match
                    let test = format!("{}.", current);
                    if regex.is_match(&test) {
                        //println!("Match: {:?} {:?}", test, regex);
                        variants += 1;
                    } else {
                        //println!("No Match: {:?} {:?}", test, regex);
                    }
                }
            }
        }
    }

    println!("{:?}", variants);

    Ok(())
}
