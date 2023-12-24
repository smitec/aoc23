mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use crate::day1::day1;
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::day9;

fn main() {
    println!("Day 1");
    match day1() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 2");
    match day2() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 3");
    match day3() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 4");
    match day4() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 5");
    match day5() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 6");
    match day6() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 7");
    match day7() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 8");
    match day8() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 9");
    match day9() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 10");
    match day10() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Day 11");
    match day11() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    println!("Skipping Day 12");
    /*
    match day12() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
    */
}
