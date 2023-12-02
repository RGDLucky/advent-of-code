use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let number = get_number(line.unwrap());
        //println!("{}", number);
        result += number;
    }

    println!("{}", result);

    Ok(())
} 

fn get_number(line: String) -> i32 {
    let numbers = { "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" }
    // check each char in line to see if it is a number and the first one store it
    // store the last one until you run out of characters
    for i in 0..line.len {

    }
}

fn check_number(line: String, number: String, start: i32) {
    
}
