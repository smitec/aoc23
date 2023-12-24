use std::fs::File;
use std::io::prelude::*;

pub fn day15() -> std::io::Result<()> {
    let mut file = File::open("./input/day15small.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total_checksum = 0;
    for item in contents.split(',') {
        let mut current_checksum = 0;
        for c in item.chars() {
            let ascii = c as u8;
            current_checksum = (current_checksum + ascii as i32) * 17 % 256;
        }
        total_checksum += current_checksum;
    }

    println!("Final Checksum: {}", total_checksum);

    Ok(())
}
