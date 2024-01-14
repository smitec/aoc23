use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct DigPlan {
    direction: Direction,
    size: usize,
    color: String,
}

fn parse_direction(c: char) -> Option<Direction> {
    match c {
        'R' => Some(Direction::Right),
        'L' => Some(Direction::Left),
        'U' => Some(Direction::Up),
        'D' => Some(Direction::Down),
        _ => None,
    }
}

pub fn day18() -> Result<()> {
    let mut file = File::open("./input/day18.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let line_regex = Regex::new(r"^(?<dir>[RDLU]) (?<size>[0-9]+) \(#(?<color>[0-9abcdef]{6})\)$")
        .context("Bad Regex")?;

    let mut steps: Vec<DigPlan> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        let re_match = line_regex.captures(line).context("No Match")?;

        let dir = parse_direction(re_match["dir"].chars().next().context("No Direction")?)
            .context("Bad Direction")?;
        let size = re_match["size"].parse::<usize>().context("Bad Size")?;
        let color = re_match["color"].to_string();

        let plan = DigPlan {
            direction: dir,
            size,
            color,
        };
        steps.push(plan);
    }

    let mut movers: HashMap<Direction, (i32, i32)> = HashMap::new();
    movers.insert(Direction::Up, (-1, 0));
    movers.insert(Direction::Down, (1, 0));
    movers.insert(Direction::Left, (0, -1));
    movers.insert(Direction::Right, (0, 1));

    let mut dug: HashMap<(i32, i32), String> = HashMap::new();
    let mut current = (0, 0);
    let mut minr = i32::MAX;
    let mut minc = i32::MAX;
    let mut maxr = i32::MIN;
    let mut maxc = i32::MIN;
    for step in steps.iter() {
        let offset = movers.get(&step.direction).context("Invalid Direction")?;
        for _ in 0..step.size {
            let (dx, dy) = offset;
            let next = (current.0 + dx, current.1 + dy);
            dug.insert(next, step.color.clone());
            current = next;
            if next.0 > maxr {
                maxr = next.0;
            }
            if next.1 > maxc {
                maxc = next.1;
            }
            if next.0 < minr {
                minr = next.0;
            }
            if next.1 < minc {
                minc = next.1;
            }
        }
    }

    println!("Bounds {}-{}, {}-{}", minr, maxr, minc, maxc);

    // flood fill.
    let mut head: Vec<(i32, i32)> = Vec::new();
    // Add additional row / column to ensure we start outside th trench (2 extra of each plus the
    // +1 to compensate for the full range)
    let w = maxc - minc + 3;
    let h = maxr - minr + 3;
    let mut tiles = vec![vec![1; w as usize]; h as usize];
    head.push((0, 0));

    while let Some((r, c)) = head.pop() {
        let true_r = r + minr - 1;
        let true_c = c + minc - 1;
        let is_dug = dug.get(&(true_r, true_c)).is_some();
        if is_dug {
            continue;
        } else {
            for d in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let offset = movers.get(&d).context("No Direction")?;
                let r_new = r + offset.0;
                let c_new = c + offset.1;
                if r_new >= 0 && r_new < h && c_new >= 0 && c_new < w {
                    // In Bounds
                    let true_r_new = r_new + minr - 1;
                    let true_c_new = c_new + minc - 1;
                    if dug.get(&(true_r_new, true_c_new)).is_none()
                        && tiles[r as usize][c as usize] == 1
                    {
                        head.push((r_new, c_new));
                    }
                }
            }
        }
        tiles[r as usize][c as usize] = 0;
    }

    let mut trenches = 0;
    for r in 0..h {
        //println!("{:?}", tiles[r as usize]);
        for c in 0..w {
            trenches += tiles[r as usize][c as usize];
        }
    }

    println!("Total Size: {}", trenches);

    Ok(())
}
