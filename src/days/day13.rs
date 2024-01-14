use std::fs::File;
use std::io::prelude::*;

struct Map {
    row_major: Vec<Vec<char>>,
    col_major: Vec<Vec<char>>,
}

fn check_reflections(lines: Vec<Vec<char>>) -> i32 {
    for i in 1..lines.len() {
        // Move down, check if all rows below match those going up
        let mut match_count = 0;
        let mut used_smudge = false;
        for j in 1..lines.len() - i + 1 {
            if (i as i32 - j as i32) < 0 {
                break;
            }
            // println!("Comparing {} and {}", i - j, i + j - 1);
            let one = &lines[i - j];
            let other = &lines[i + j - 1];
            if *one == *other {
                // println!("{:?} equals {:?}", one, other);
                match_count += 1;
            } else {
                // println!("{:?} NE {:?}", one, other);
                // Part 2, is the difference 1 char?
                let mut diff = 0;
                for (a, b) in std::iter::zip(one, other) {
                    if a != b {
                        diff += 1;
                    }
                }
                if diff == 1 && !used_smudge {
                    match_count += 1;
                    used_smudge = true;
                    continue;
                }
                break;
            }
        }
        if (match_count == i || i + match_count == lines.len()) && used_smudge {
            return i as i32;
        }
    }
    return 0;
}

pub fn day13() -> std::io::Result<()> {
    let mut file = File::open("./input/day13.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut maps: Vec<Map> = Vec::new();
    let mut current_rows: Vec<Vec<char>> = Vec::new();
    let mut current_cols: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            maps.push(Map {
                row_major: current_rows,
                col_major: current_cols,
            });
            current_cols = Vec::new();
            current_rows = Vec::new();
            continue;
        }
        let row: Vec<char> = line.chars().collect();
        if current_cols.is_empty() {
            for _ in 0..row.len() {
                current_cols.push(Vec::new());
            }
        }
        current_rows.push(line.chars().collect());
        for (i, v) in row.iter().enumerate() {
            current_cols[i].push(*v);
        }
    }

    println!("Got {} Maps", maps.len());

    // Try to find reflections
    let mut reflections = 0;
    for map in maps {
        let c = check_reflections(map.row_major);
        if c == 0 {
            reflections += check_reflections(map.col_major);
        } else {
            reflections += 100 * c;
        }
    }

    println!("Total Reflection Points: {}", reflections);
    Ok(())
}
