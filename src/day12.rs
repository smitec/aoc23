use anyhow::{Context, Result};
use std::{fs::File, io::Read};

enum PatternState {
    Operational, //'.'
    Damaged,     //'#'
    Unknown,     // '?'
}

struct Pattern {
    pattern: Vec<Option<PatternState>>,
    checksum: Vec<u32>,
}

fn char_to_pattern(c: char) -> Option<PatternState> {
    match c {
        '.' => Some(PatternState::Operational),
        '#' => Some(PatternState::Damaged),
        '?' => Some(PatternState::Unknown),
        _ => None,
    }
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
        let checksum_parsed: Vec<u32> = checksum
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let pattern_parsed: Vec<Option<PatternState>> =
            pattern.chars().map(char_to_pattern).collect();
        patterns.push(Pattern {
            pattern: pattern_parsed,
            checksum: checksum_parsed,
        });
    }

    Ok(())
}
