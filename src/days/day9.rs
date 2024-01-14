use anyhow::{Context, Result};
use std::{fs::File, io::Read};

pub fn day9() -> Result<()> {
    let mut file = File::open("./input/day9.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let mut sum = 0;
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let items: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .rev() // Day 2, work backwards
            .collect();

        let mut final_derivatives: Vec<i32> = Vec::new();
        final_derivatives.push(*items.last().unwrap());
        sum += final_derivatives.last().unwrap();

        let mut current_dx = items.clone();
        let mut done = false;

        while !done {
            done = true;
            let mut dx: Vec<i32> = Vec::new();
            for i in 0..current_dx.len() - 1 {
                let value = current_dx[i + 1] - current_dx[i];
                if value != 0 {
                    done = false;
                }
                dx.push(value);
            }

            final_derivatives.push(*dx.last().unwrap());
            current_dx = dx;
            sum += final_derivatives.last().unwrap();
        }
    }

    println!("Final Sum: {:?}", sum);
    Ok(())
}
