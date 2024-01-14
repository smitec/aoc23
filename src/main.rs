mod days;

use crate::days::{
    day1::*, day10::*, day11::*, day13::*, day14::*, day15::*, day16::*, day17::*, day18::*,
    day2::*, day3::*, day4::*, day5::*, day6::*, day7::*, day8::*, day9::*,
};

fn main() {
    let run_old = false;
    if run_old {
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
        println!("Day 13");
        match day13() {
            Ok(_) => {}
            Err(s) => println!("{:?}", s),
        }
        println!("Day 14");
        match day14() {
            Ok(_) => {}
            Err(s) => println!("{:?}", s),
        }
        println!("Day 15");
        match day15() {
            Ok(_) => {}
            Err(s) => println!("{:?}", s),
        }
        println!("Day 16");
        match day16() {
            Ok(_) => {}
            Err(s) => println!("{:?}", s),
        }
        println!("Day 17");
        match day17() {
            Ok(_) => {}
            Err(s) => println!("{:?}", s),
        }
    }
    println!("Day 18");
    match day18() {
        Ok(_) => {}
        Err(s) => println!("{:?}", s),
    }
}
