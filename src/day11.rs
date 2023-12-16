use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

pub fn day11() -> Result<()> {
    let mut file = File::open("./input/day11.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut filled_rows: HashSet<usize> = HashSet::new();
    let mut filled_cols: HashSet<usize> = HashSet::new();
    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in contents.split('\n').enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                filled_rows.insert(row);
                filled_cols.insert(col);
                galaxies.insert((row, col));
            }
        }
    }

    println!(
        "Filled Rows {:?} Filled Cols {:?} Galaxies {:?}",
        filled_rows.len(),
        filled_cols.len(),
        galaxies.len()
    );

    let mut total_distance = 0_f64;
    for (i, item) in galaxies.iter().enumerate() {
        for comp in galaxies.iter().skip(i) {
            //println!("{:?} to {:?}", item, comp);
            let minr = item.0.min(comp.0);
            let maxr = item.0.max(comp.0);
            for r in minr..maxr {
                total_distance += match filled_rows.get(&r) {
                    Some(_) => 1.,
                    None => 1000000.,
                }
            }
            let minc = item.1.min(comp.1);
            let maxc = item.1.max(comp.1);
            for c in minc..maxc {
                total_distance += match filled_cols.get(&c) {
                    Some(_) => 1.,
                    None => 1000000.,
                }
            }
        }
    }

    println!("{:?}", total_distance);

    Ok(())
}
