use anyhow::{Context, Result};
use std::collections::HashMap;
use std::{fs::File, io::Read};

pub fn day2() -> Result<()> {
    let mut file = File::open("./input/day2.txt")?;
    let mut contents = "".to_string();
    file.read_to_string(&mut contents)
        .context("Couldn't read the file.")?;
    let max_balls = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut id_total = 0;
    let mut power_total = 0;

    for line in contents.as_str().split('\n') {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(": ");
        let first = parts.next().unwrap(); // Game x
        let game_id = first
            .replace("Game ", "")
            .parse::<i32>()
            .context("Couldn't parse game id")?;

        let last = parts.next().unwrap(); // the data
        let mut game_ok = true;
        let mut game_max = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for hand in last.split("; ") {
            for portion in hand.split(", ") {
                let mut portion_parts = portion.split(' ');
                let ball_count = portion_parts
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .context("Couldn't parse ball count")?;
                let color = portion_parts.next().unwrap();
                let comp = *max_balls.get(color).unwrap();
                let comp_game = *game_max.get(color).unwrap();
                if ball_count > comp_game {
                    game_max.insert(color, ball_count);
                }

                if ball_count > comp {
                    game_ok = false;
                }
            }
        }

        let mut game_power = 1;
        for v in game_max.into_values() {
            game_power *= v;
        }
        power_total += game_power;

        if game_ok {
            id_total += game_id;
        }
    }

    println!("{:?}, {:?}", id_total, power_total);

    Ok(())
}
