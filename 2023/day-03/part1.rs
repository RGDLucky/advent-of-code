use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);

    // construct array
    let mut schematic: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let schematic_line: Vec<char> = line.unwrap().chars().collect();
        // add shecmatic_line to schematic
        schematic.push(schematic_line);
    }

    // call function and get result
    let result = adjacent_parts(schematic);
    println!("{}", result);

    Ok(())
}    

// get all lines into an array
// find numbers in array then look for adjacent symbol
fn adjacent_parts(schematic: Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    for i in 0..schematic.len() {
        let mut j = 0;
        while j < schematic[i].len() {
            if is_digit(schematic[i][j]) {
                let mut found = false;
                let mut number = String::from("");
                let mut k = j;
                while k < schematic[i].len() && is_digit(schematic[i][k]) {
                    number.push(schematic[i][k]);
                    // top left
                    if i > 0 && k > 0 && schematic[i - 1][k - 1] != '.' && !is_digit(schematic[i - 1][k - 1]) {
                        found = true;
                    }
                    // bottom left
                    if i < schematic.len() - 1 && k > 0 && schematic[i + 1][k - 1] != '.' && !is_digit(schematic[i + 1][k - 1]) {
                        found = true;
                    }
                    // top right
                    if i > 0 && k < schematic[i].len() - 1 && schematic[i - 1][k + 1] != '.' && !is_digit(schematic[i - 1][k + 1]) {
                        found = true;
                    }
                    // bottom right
                    if i < schematic.len() - 1 && k < schematic[i].len() - 1 && schematic[i + 1][k + 1] != '.' && !is_digit(schematic[i + 1][k + 1]) {
                        found = true;
                    }
                    // top
                    if i > 0 && schematic[i - 1][k] != '.' && !is_digit(schematic[i - 1][k]) {
                        found = true;
                    }
                    // bottom
                    if i < schematic.len() - 1 && schematic[i + 1][k] != '.' && !is_digit(schematic[i + 1][k]) {
                        found = true;
                    }
                    // left
                    if k > 0 && schematic[i][k - 1] != '.' && !is_digit(schematic[i][k - 1]) {
                        found = true;
                    }
                    // right
                    if k < schematic[i].len() - 1 && schematic[i][k + 1] != '.' && !is_digit(schematic[i][k + 1]) {
                        found = true;
                    }

                    k += 1;
                }

                // adding number to result if found is true and update j
                if found { 
                    result += number.parse::<i32>().unwrap();
                }
                j = k;
            }
            j += 1;
        }
    }
    result
}

fn is_digit(digit: char) -> bool {
    match digit {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => return true,
        _ => return false,
    }
}
