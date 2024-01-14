use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Lens {
    id: String,
    focal: i32,
}

fn checksum(s: String) -> i32 {
    let mut checksum = 0;
    for c in s.chars() {
        let ascii = c as u8;
        checksum = (checksum + ascii as i32) * 17 % 256;
    }
    checksum
}

pub fn day15() -> Result<()> {
    let mut file = File::open("./input/day15.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    let mut total_checksum = 0;
    contents = contents.replace('\n', "");
    for item in contents.split(',') {
        let current_checksum = checksum(item.to_string());
        // Place the Lens in the Boxes
        if item.contains('=') {
            // Add or Replace the Lens to Box X
            let (id, f_string) = item.split_once('=').context("No Equal")?;
            let box_id = checksum(id.to_string()) as usize;
            let curent_box = boxes.get_mut(box_id).context("No Box")?;
            let focal = f_string
                .parse::<i32>()
                .context("Couldn't Parse Focal Length")?;
            //println!("Adding {} to {} with focal {}", id, box_id, focal);
            let mut found = false;
            for v in curent_box.iter_mut() {
                if v.id == id {
                    v.focal = focal;
                    found = true;
                    break;
                }
            }
            if !found {
                curent_box.push(Lens {
                    id: id.to_string(),
                    focal,
                });
            }
        } else if item.contains('-') {
            // Remove the Lens from Box X
            let id = item.split_once('-').context("No Dash")?.0;
            let box_id = checksum(id.to_string()) as usize;
            let curent_box = boxes.get_mut(box_id).context("No Box")?;
            //println!("Removing {} from {}", id, box_id);
            for (i, v) in curent_box.iter().enumerate() {
                if v.id == id {
                    curent_box.remove(i);
                    break;
                }
            }
        }
        total_checksum += current_checksum;
        //println!("{} -> {}", item, current_checksum);
    }

    println!("Final Checksum: {}", total_checksum);

    // Add the focal lengths
    let mut total_focal = 0;
    for (i, v) in boxes.iter().enumerate() {
        for (j, lens) in (*v).iter().enumerate() {
            total_focal += (i as i32 + 1) * (j as i32 + 1) * lens.focal;
        }
    }

    println!("Final Focal: {}", total_focal);

    Ok(())
}
