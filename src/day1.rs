use std::fs::File;
use std::io::prelude::*;

fn find_earliest_replace(line: String, rev: bool) -> String {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first_v = 10;
    let mut first_pos = line.len();

    for (c, digit) in digits.iter().enumerate() {
        if rev {
            let rdigit: String = digit.chars().rev().collect();
            if let Some(x) = line.chars().rev().collect::<String>().find(rdigit.as_str()) {
                if x < first_pos {
                    first_pos = x;
                    first_v = c;
                }
            }
        } else if let Some(x) = line.find(digit) {
            if x < first_pos {
                first_pos = x;
                first_v = c;
            }
        }
    }
    if first_v == 10 {
        return line;
    }

    if rev {
        let rline: String = line.chars().rev().collect();
        let rdigit: String = digits[first_v].chars().rev().collect();

        rline
            .replacen(rdigit.as_str(), format!("{:?}", first_v + 1).as_str(), 1)
            .chars()
            .rev()
            .collect()
    } else {
        line.replacen(digits[first_v], format!("{:?}", first_v + 1).as_str(), 1)
    }
}

pub fn day1() -> std::io::Result<()> {
    let part_2 = true;
    let debug: bool = true;

    let mut file = File::open("./input/day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total = 0;
    for line in contents.lines() {
        let mut vline = line;
        let vline_a;
        let vline_b;
        if part_2 {
            vline_a = find_earliest_replace(vline.to_string(), false);
            vline = vline_a.as_str();
        }

        let mut current = 0;
        for c in vline.chars() {
            if c.is_numeric() {
                current = c.to_digit(10).unwrap();
                break;
            }
        }

        current *= 10;

        if part_2 {
            vline_b = find_earliest_replace(line.to_string(), true);
            vline = vline_b.as_str();
        }

        for c in vline.chars().rev() {
            if c.is_numeric() {
                current += c.to_digit(10).unwrap();
                break;
            }
        }

        if debug {
            println!("{:?} -> {:?} -> {:?}", line, vline, current);
        }
        total += current;
    }
    println!("{:?}", total);
    Ok(())
}
