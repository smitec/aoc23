use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
enum TileType {
    Empty,              // .
    MirrorForward,      // /
    MirrorBackward,     // \
    SplitterVertical,   // |
    SplitterHorizontal, // -
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Head {
    pos: (usize, usize),
    direction: Direction,
}

struct Map {
    tiles: Vec<Vec<TileType>>,
    rows: usize,
    cols: usize,
}

fn char_to_tile(c: char) -> Option<TileType> {
    match c {
        '.' => Some(TileType::Empty),
        '/' => Some(TileType::MirrorForward),
        '\\' => Some(TileType::MirrorBackward),
        '|' => Some(TileType::SplitterVertical),
        '-' => Some(TileType::SplitterHorizontal),
        _ => None,
    }
}

fn move_in_direction(
    pos: (usize, usize),
    d: &Direction,
    bounds: (usize, usize),
) -> Option<(usize, usize)> {
    // Attempt to move one square, None if OOB
    let mut test_r = pos.0 as i32;
    let mut test_c = pos.1 as i32;

    match d {
        Direction::Up => test_r -= 1,
        Direction::Down => test_r += 1,
        Direction::Left => test_c -= 1,
        Direction::Right => test_c += 1,
    }

    if test_c >= 0 && test_c < bounds.0 as i32 && test_r >= 0 && test_r < bounds.1 as i32 {
        Some((test_r as usize, test_c as usize))
    } else {
        None
    }
}

fn update_direction(d: Direction, t: TileType) -> (Direction, Option<Direction>) {
    // Return either one or two directions based on the tile and current direction
    match t {
        TileType::Empty => (d, None),
        // /
        TileType::MirrorForward => match d {
            Direction::Up => (Direction::Right, None),
            Direction::Down => (Direction::Left, None),
            Direction::Left => (Direction::Down, None),
            Direction::Right => (Direction::Up, None),
        },
        // \
        TileType::MirrorBackward => match d {
            Direction::Up => (Direction::Left, None),
            Direction::Down => (Direction::Right, None),
            Direction::Left => (Direction::Up, None),
            Direction::Right => (Direction::Down, None),
        },
        TileType::SplitterVertical => match d {
            Direction::Up => (d, None),
            Direction::Down => (d, None),
            Direction::Left => (Direction::Up, Some(Direction::Down)),
            Direction::Right => (Direction::Up, Some(Direction::Down)),
        },
        TileType::SplitterHorizontal => match d {
            Direction::Up => (Direction::Left, Some(Direction::Right)),
            Direction::Down => (Direction::Left, Some(Direction::Right)),
            Direction::Left => (d, None),
            Direction::Right => (d, None),
        },
    }
}

fn run_sim(initial_pos: (usize, usize), initial_d: Direction, map: &Map) -> Result<usize> {
    let mut head: Vec<Head> = Vec::new();
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<Head> = HashSet::new();

    head.push(Head {
        pos: initial_pos,
        direction: initial_d,
    });

    while !head.is_empty() {
        let current = head.pop().context("Empty Head")?;
        // Energize the current location.
        energized.insert(current.pos);

        let tile = map.tiles[current.pos.0][current.pos.1];
        // Assess the new direction(s)
        let (d1, d2) = update_direction(current.direction, tile);

        // Move one or both new beams
        let new_pos = move_in_direction(current.pos, &d1, (map.rows, map.cols));
        if let Some(p) = new_pos {
            let new_head = Head {
                pos: p,
                direction: d1,
            };
            if !visited.contains(&new_head) {
                visited.insert(new_head);
                head.push(new_head);
            }
        }

        if let Some(dir) = d2 {
            let new_pos = move_in_direction(current.pos, &dir, (map.rows, map.cols));
            if let Some(p) = new_pos {
                let new_head = Head {
                    pos: p,
                    direction: dir,
                };
                if !visited.contains(&new_head) {
                    visited.insert(new_head);
                    head.push(new_head);
                }
            }
        }
    }

    Ok(energized.len())
}

pub fn day16() -> Result<()> {
    let mut file = File::open("./input/day16.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut map: Vec<Vec<TileType>> = Vec::new();
    let rows = contents.lines().count();
    let mut cols = 0;
    for line in contents.lines() {
        let current: Vec<TileType> = line.chars().map(|x| char_to_tile(x).unwrap()).collect();
        cols = current.len();
        map.push(current);
    }
    let map = Map {
        tiles: map,
        rows,
        cols,
    };

    println!("Got a map of size {} r {} c", rows, cols);

    // Count the energized tiles
    println!(
        "Total Energized Tiles Part 1: {}",
        run_sim((0, 0), Direction::Right, &map).context("Sim Filed")?
    );

    // Part 2
    let mut max_energized = 0;
    // Top Down
    for i in 0..map.cols {
        let t = run_sim((0, i), Direction::Down, &map).context("Sim Filed")?;
        if t > max_energized {
            max_energized = t;
        }
    }

    // Bottom Up
    for i in 0..map.cols {
        let t = run_sim((map.rows - 1, i), Direction::Up, &map).context("Sim Filed")?;
        if t > max_energized {
            max_energized = t;
        }
    }

    // Left Right
    for i in 0..map.rows {
        let t = run_sim((i, 0), Direction::Right, &map).context("Sim Filed")?;
        if t > max_energized {
            max_energized = t;
        }
    }

    // Right Left
    for i in 0..map.rows {
        let t = run_sim((i, map.cols - 1), Direction::Left, &map).context("Sim Filed")?;
        if t > max_energized {
            max_energized = t;
        }
    }

    println!("Max Overall Energized: {}", max_energized);

    Ok(())
}
