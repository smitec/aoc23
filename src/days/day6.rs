use anyhow::{Context, Result};
use std::iter::zip;
use std::{fs::File, io::Read};

pub fn day6() -> Result<()> {
    let mut file = File::open("./input/day6b.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;

    let lines: Vec<String> = contents.split('\n').map(|x| x.to_string()).collect();
    assert!(lines.len() == 3);

    let times = lines[0].replace("Time:", "");
    let distances = lines[1].replace("Distance:", "");

    let times_vals: Vec<f64> = times
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let distance_vals: Vec<f64> = distances
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let mut tot = 1_f64;
    for (time, distance) in zip(times_vals, distance_vals) {
        // Find the first time in which (time - x)*x > distance
        // Speed = x
        // distance = (time - x)*x = time*x - x*x
        // solving for distance > 0 is time*x - x*x - distance > 0
        // quad formual is x = -b +/- sqrt(b**2 - 4*a*c) / 2*a
        // where a = -1, b = time, c = -distance
        // giving x = -time +/- sqrt(time*time - 4*-1*distance) / 2*-1
        let a = -1_f64;
        let b = time;
        let c = -distance;

        let root1 = (-b + (b.powi(2) - 4_f64 * a * c).sqrt()) / (2_f64 * a) + 1e-6;
        let root2 = (-b - (b.powi(2) - 4_f64 * a * c).sqrt()) / (2_f64 * a) - 1e-6;

        let r1_c = root1.ceil();
        let r2_f = root2.floor();

        let delta = r2_f - r1_c + 1.0;

        tot *= delta;
    }

    println!("{:?}", tot);

    Ok(())
}
