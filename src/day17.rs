use anyhow::{Context, Result};
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

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Head {
    pos: (usize, usize),
    direction: Direction,
    chain: usize,
    heat: i32,
}

impl Ord for Head {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat.cmp(&other.heat)
    }
}

impl PartialOrd for Head {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat.partial_cmp(&other.heat)
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Key {
    pos: (usize, usize),
    direction: Direction,
    chain: usize,
}

struct Map {
    tiles: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

fn left(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

fn right(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn valid_directions(head: &Head) -> Vec<Direction> {
    let mut valid: Vec<Direction> = Vec::new();
    if head.chain < 10 {
        // use 3 for part 1
        valid.push(head.direction);
    }
    if head.chain >= 4 {
        valid.push(left(&head.direction));
        valid.push(right(&head.direction));
    }

    valid
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

fn run_sim(initial_pos: (usize, usize), initial_d: Direction, map: &Map) -> Result<i32> {
    let mut head: BinaryHeap<Head> = BinaryHeap::new();
    let mut distances: HashMap<Key, i32> = HashMap::new();
    let mut visited: HashSet<Key> = HashSet::new();

    let h = Head {
        pos: initial_pos,
        direction: initial_d,
        chain: 0,
        heat: 0,
    };
    head.push(h);

    distances.insert(
        Key {
            pos: h.pos,
            direction: h.direction,
            chain: h.chain,
        },
        0,
    );

    while !head.is_empty() {
        let current = head.pop().context("Empty Head")?;
        let key = Key {
            pos: current.pos,
            direction: current.direction,
            chain: current.chain,
        };
        if current.pos == (map.rows - 1, map.cols - 1) {
            println!("{:?}", distances.get(&key));
        }
        if visited.contains(&key) {
            continue;
        }
        let current_heat = current.heat;
        // Find Valid Directions.
        let next_directions = valid_directions(&current);
        // Attempt to move in them.
        for direction in next_directions.iter() {
            let new_pos = move_in_direction(current.pos, direction, (map.rows, map.cols));
            if let Some(x) = new_pos {
                let new_heat = map.tiles[x.0][x.1];
                let chain = if *direction == current.direction {
                    current.chain + 1
                } else {
                    1
                };
                let t = Head {
                    pos: x,
                    direction: *direction,
                    chain,
                    heat: current_heat - new_heat,
                };
                let t_key = Key {
                    pos: x,
                    direction: *direction,
                    chain,
                };
                let current_best = distances.get(&t_key);
                if !visited.contains(&t_key)
                    && (current_best.is_none()
                        || current_best.is_some_and(|x| *x < current_heat - new_heat))
                {
                    head.push(t);
                    if chain >= 4 {
                        distances.insert(t_key, current_heat - new_heat);
                    }
                }
            }
        }
        visited.insert(key);
    }

    Ok(0)
}

pub fn day17() -> Result<()> {
    let mut file = File::open("./input/day17.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rows = contents.lines().count();
    let mut cols = 0;

    let mut tiles: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        if !line.is_empty() {
            cols = line.len();
            tiles.push(
                line.chars()
                    .map(|x| x.to_string().parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }
    let map = Map { tiles, rows, cols };
    println!("Final Heat: {:?}", run_sim((0, 0), Direction::Right, &map));
    Ok(())
}
