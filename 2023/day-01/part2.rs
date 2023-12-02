use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input2.txt")?;
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
    let numbers = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
    let mut digit1 = -1;
    let mut digit2 = -1;
    for i in 0..line.len() {
        let mut count = 1;
        if check_digit(line.chars().nth(i).unwrap()) {
            if digit1 == -1 {
                digit1 = line.chars().nth(i).unwrap() as i32 - 48;
            }
            digit2 = line.chars().nth(i).unwrap() as i32 - 48;
            continue;
        }
        for number in numbers {
            if check_number(line.clone(), number.to_string(), i) {
                if digit1 == -1 {
                    digit1 = count;
                }
                digit2 = count;
            }
            count += 1;
        }
    }
    println!("{} {}", digit1, digit2);
    digit1 * 10 + digit2
}

fn check_number(line: String, number: String, start: usize) -> bool {
    let mut line_index = start;
    for character in number.chars() {
        if line_index >= line.len() || line.chars().nth(line_index).unwrap() != character {
            return false;
        }
        line_index += 1;
    }

    true
}

fn check_digit(character: char) -> bool {
    match character {
        '1' | '2' | '3' | '4' | '5' | '6'| '7' | '8' | '9' => return true,
        _ => return false,
    }
}
