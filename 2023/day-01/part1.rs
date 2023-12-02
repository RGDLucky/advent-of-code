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
    let mut digit1 = 0;
    for digit in line.chars() {
        if check_if_digit(digit) {
            digit1 += digit as i32;
            break;
        }
    }
    let mut digit2 = 0;
    for digit in line.chars() {
        if check_if_digit(digit) {
            digit2 = digit as i32;
        }
    }

    //println!("{} {}", digit1, digit2);

    (digit1 - 48) * 10 + (digit2 - 48)
}

fn check_if_digit(digit: char) -> bool {
    let is_digit;
    match digit {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => is_digit = true,
        _ => is_digit = false,
    }
    is_digit
}
