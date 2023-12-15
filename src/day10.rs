use anyhow::{Context, Result};
use std::{fs::File, io::Read};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Dirt,       // .
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Animal,     // S
}

#[derive(Debug, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    fn opposite(self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::South,
            Cardinal::South => Cardinal::North,
            Cardinal::East => Cardinal::West,
            Cardinal::West => Cardinal::East,
        }
    }
}

fn parse_tile(c: char) -> Option<Tile> {
    match c {
        '.' => Some(Tile::Dirt),
        '|' => Some(Tile::NorthSouth),
        '-' => Some(Tile::EastWest),
        'L' => Some(Tile::NorthEast),
        'J' => Some(Tile::NorthWest),
        '7' => Some(Tile::SouthWest),
        'F' => Some(Tile::SouthEast),
        'S' => Some(Tile::Animal),
        _ => None,
    }
}

fn position_offset(p1: Position, p2: Position) -> Option<Cardinal> {
    // Returns the direction of p2 relative to p1
    // Assumes the two positions are 1 tile away either up, down, left or right
    // Remembering rows go down and columns go left, so a delta of one row (positive) is a movement
    // down
    let dr = p2.row as i32 - p1.row as i32;
    let dc = p2.col as i32 - p1.col as i32;
    match (dr, dc) {
        (1, 0) => Some(Cardinal::South),
        (-1, 0) => Some(Cardinal::North),
        (0, 1) => Some(Cardinal::East),
        (0, -1) => Some(Cardinal::West),
        _ => None,
    }
}

fn does_accept(t: Tile, c: Cardinal) -> bool {
    // Does the tile t accept an input from direction c?
    match t {
        Tile::NorthSouth => matches!(c, Cardinal::North | Cardinal::South),
        Tile::EastWest => matches!(c, Cardinal::East | Cardinal::West),
        Tile::NorthEast => matches!(c, Cardinal::South | Cardinal::West),
        Tile::NorthWest => matches!(c, Cardinal::South | Cardinal::East),
        Tile::SouthWest => matches!(c, Cardinal::North | Cardinal::East),
        Tile::SouthEast => matches!(c, Cardinal::North | Cardinal::West),
        Tile::Animal => true,
        _ => false,
    }
}

pub fn day10() -> Result<()> {
    let mut file = File::open("./input/day10.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    // Plan:
    // Put the contents in the file of a 2D array of enum values
    // Enum values define directionality
    // During parsing store the start point of the Creature
    // Based on the surrounding tiles, check if they could possibly lead into the current tile at
    // the head.
    // For each of the two tiles that could, add them to the head.
    // Should be pretty straightforward pathfinding.

    // Parse the map, storing the animal position when we find it.
    let mut head: Vec<Position> = Vec::new();
    let mut animal: Position = Position { row: 0, col: 0 };
    let mut row_max = 0;
    let mut col_max = 0;
    for (row, line) in contents.split('\n').enumerate() {
        let mut current_row: Vec<Tile> = Vec::new();
        for (col, c) in line.chars().enumerate() {
            let tile = parse_tile(c);
            if let Some(tile) = tile {
                current_row.push(tile);
                if let Tile::Animal = tile {
                    animal = Position { row, col };
                    head.push(animal);
                }
            }
            col_max = col;
        }
        row_max = row;
        tiles.push(current_row);
    }

    let mut distances: Vec<Vec<i32>> = Vec::new();
    // The blank line means no need for the extra row.
    col_max += 1;
    for _ in 0..row_max {
        let mut current_row = Vec::new();
        for _ in 0..col_max {
            current_row.push(-1);
        }
        distances.push(current_row);
    }

    println!("{:?} {:?}", row_max, col_max);

    distances[animal.row][animal.col] = 0;

    // Loop through until we run out of places to go.
    while !head.is_empty() {
        let current_position = head.pop().context("Head empty in loop")?;
        let current_distance = distances[current_position.row][current_position.col];
        // Check the surrounding positions for potentially adjoining pipes.
        // If the position is either unvisited (-1) or has a distance less than the distanct at
        // head + 1, add the new position to head and set its distance to distance at head + 1
        for dr in [-1, 0, 1] {
            for dc in [-1, 0, 1] {
                if (dr != 0 && dc != 0) || (dr == 0 && dc == 0) {
                    continue;
                }

                let row = current_position.row as i32 + dr;
                let col = current_position.col as i32 + dc;

                if row < 0 || row == row_max as i32 || col < 0 || col == col_max as i32 {
                    continue;
                }

                let row = (current_position.row as i32 + dr) as usize;
                let col = (current_position.col as i32 + dc) as usize;

                let test_position = Position { row, col };
                let diff = position_offset(current_position, test_position).context(format!(
                    "Position Difference was not Valid {:?} {:?}",
                    current_position, test_position
                ))?;

                let destination_tile = tiles[test_position.row][test_position.col];
                let current_tile = tiles[current_position.row][current_position.col];

                if does_accept(destination_tile, diff) && does_accept(current_tile, diff.opposite())
                {
                    let test_distance = distances[test_position.row][test_position.col];
                    if test_distance == -1 || current_distance + 1 < test_distance {
                        // println!("{:?} {:?} {:?}", current_position, diff, destination_tile);
                        distances[test_position.row][test_position.col] = current_distance + 1;
                        head.push(test_position);
                    }
                }
            }
        }
    }

    // Find the highest distance
    let mut max_d = -1;
    for row in &distances {
        // println!("{:?}", row);
        for v in row {
            if *v > max_d {
                max_d = *v;
            }
        }
    }

    println!("Maximum Distance: {:?}", max_d);

    Ok(())
}
