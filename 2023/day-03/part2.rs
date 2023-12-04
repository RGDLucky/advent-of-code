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
    let result = gears(schematic);
    println!("{}", result);

    Ok(())
}

fn gears(schematic: Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    for i in 0..schematic.len() {
        for j in 0..schematic[i].len() {
            if schematic[i][j] = '*' {
                let mut found = 0;
                let mut new_number = 1
                // check adjacency and if so then retrieve number and if it
                // is different than already in the vec then add it
            }
        }
    }
}

fn retrieve_number(schematic: Vec<Vec<char>>, indexi: usize, indexj: usize) -> i32 {
 
}

fn is_digit(digit: char) -> bool {
    match digit {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => return true,
        _ => return false,
    }
}
