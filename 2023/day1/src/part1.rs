use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                total += get_first_digit(&ip) * 10 + get_last_digit(&ip);
            }
        }
    }
    println!("Sum of all calibration numbers: {}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_first_digit(s: &str) -> u32 {
    for c in s.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    0
}

fn get_last_digit(s: &str) -> u32 {
    for c in s.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    0
}
