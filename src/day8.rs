use anyhow::{Context, Result};
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
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

pub fn day8() -> Result<()> {
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
