use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let candidates = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut total: u64 = 0;
    for line in reader.lines() {
        if let Ok(s) = line {
            let ans = solve(&s, &candidates);
            total += ans as u64;
            // println!("{} -> {}", s, ans);
        }
    }
    println!("Sum of all calibration numbers is: {}", total);

    Ok(())
}

fn solve(s: &String, candidates: &HashMap<&str, u32>) -> u64 {
    let mut calibration_value: u64 = 0;
    let mut result = match_leftmost(&s, &candidates);
    calibration_value += (result * 10) as u64;
    result = match_rightmost(&s, &candidates);
    calibration_value += result as u64;
    calibration_value
}

fn match_leftmost(s: &String, candidates: &HashMap<&str, u32>) -> u32 {
    let mut leftmost_match_index: usize = s.len() - 1; 
    let mut leftmost_match_value: u32 = 0;
    for (key, value) in candidates.iter() {
        if let Some(match_index) = s.find(key) {
            if match_index <= leftmost_match_index {
                leftmost_match_index = match_index;
                leftmost_match_value = *value;
            }
        }
    }
    if leftmost_match_value == 0 {
        panic!("{}: Leftmost match should not be 0", s);
    }
    leftmost_match_value
}

fn match_rightmost(s: &String, candidates: &HashMap<&str, u32>) -> u32 {
    let mut rightmost_match_index: usize = 0; 
    let mut rightmost_match_value: u32 = 0;
    for (key, value) in candidates.iter() {
        if let Some(match_index) = s.rfind(key) {
            if match_index >= rightmost_match_index {
                rightmost_match_index = match_index;
                rightmost_match_value = *value;
            }
        }
    }
    if rightmost_match_value == 0 {
        panic!("{}: Rightmost match should not be 0", s);
    }
    rightmost_match_value
}

