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

#[derive(Debug, Clone, Copy)]
enum TileState {
    Loop,
    Left,
    Unvisited,
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

fn lefts(tile: Tile, c: Cardinal) -> Vec<(i32, i32)> {
    match tile {
        Tile::NorthSouth => match c {
            Cardinal::North => vec![(0, -1)],
            Cardinal::South => vec![(0, 1)],
            _ => Vec::new(),
        },
        Tile::EastWest => match c {
            Cardinal::East => vec![(-1, 0)],
            Cardinal::West => vec![(1, 0)],
            _ => Vec::new(),
        },
        Tile::NorthEast => match c {
            Cardinal::West => vec![(1, 0), (0, -1)], //TODO: diagonal?
            _ => Vec::new(),
        },
        Tile::NorthWest => match c {
            Cardinal::South => vec![(1, 0), (0, 1)], //TODO: diagonal?
            _ => Vec::new(),
        },
        Tile::SouthWest => match c {
            Cardinal::East => vec![(-1, 0), (0, 1)], //TODO: diagonal?
            _ => Vec::new(),
        },
        Tile::SouthEast => match c {
            Cardinal::North => vec![(-1, 0), (0, -1)], //TODO: diagonal?
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

pub fn day10() -> Result<()> {
    let mut file = File::open("./input/day10.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut tiles: Vec<Vec<Tile>> = Vec::new();

    // Parse the map, storing the animal position when we find it.
    let mut head: Vec<Position> = Vec::new();
    let mut animal: Position = Position { row: 0, col: 0 };
    let mut row_max = 0;
    let mut col_max = 0;

    tiles.push(Vec::new()); // Padding to be filled afterwards
    for (row, line) in contents.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut current_row: Vec<Tile> = Vec::new();
        // Padding
        current_row.push(Tile::Dirt);
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
        // Padding
        current_row.push(Tile::Dirt);
        row_max = row;
        tiles.push(current_row);
    }
    tiles.push(Vec::new()); // Padding to be filled

    col_max += 3;
    row_max += 3;

    for r in [0, row_max - 1] {
        let line = tiles.get_mut(r).context("no row")?;
        for _ in 0..col_max {
            line.push(Tile::Dirt);
        }
    }

    let mut distances: Vec<Vec<i32>> = Vec::new();
    let mut states: Vec<Vec<TileState>> = Vec::new();
    for _ in 0..row_max {
        let current_row = vec![-1; col_max];
        distances.push(current_row);

        let current_row = vec![TileState::Unvisited; col_max];
        states.push(current_row);
    }

    println!("{:?} {:?}", row_max, col_max);

    distances[animal.row][animal.col] = 0;
    states[animal.row][animal.col] = TileState::Loop;

    // Loop through until we run out of places to go.
    while !head.is_empty() {
        let current_position = head.pop().context("Head empty in loop")?;
        let current_distance = distances[current_position.row][current_position.col];
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let row = current_position.row as i32 + dr;
            let col = current_position.col as i32 + dc;

            if row < 0 || row == row_max as i32 || col < 0 || col == col_max as i32 {
                continue;
            }

            let row = row as usize;
            let col = col as usize;

            let test_position = Position { row, col };
            let diff = position_offset(current_position, test_position).context(format!(
                "Position Difference was not Valid {:?} {:?}",
                current_position, test_position
            ))?;

            let destination_tile = tiles[test_position.row][test_position.col];
            let current_tile = tiles[current_position.row][current_position.col];

            if does_accept(destination_tile, diff) && does_accept(current_tile, diff.opposite()) {
                let test_distance = distances[test_position.row][test_position.col];
                if test_distance == -1 || current_distance + 1 < test_distance {
                    // println!("{:?} {:?} {:?}", current_position, diff, destination_tile);
                    distances[test_position.row][test_position.col] = current_distance + 1;
                    head.push(test_position);

                    // I found a spot to move and a direction, mark my current location as Loop and
                    // any valid Left tiles as Left
                    states[current_position.row][current_position.col] = TileState::Loop;

                    for (lr, lc) in lefts(destination_tile, diff) {
                        todo!()
                    }
                    break;
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

    println!("Maximum Distance: {:?}", max_d / 2); // 7097

    Ok(())
}
