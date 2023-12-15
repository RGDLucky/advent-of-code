use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Ideas/Notes: get all lines into a an array, then check for vertical and if not check for
// horizontal reflection

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut contents: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        contents.push(line.chars().collect());
    }

    let vertical = check_vertical(contents.clone());

    println!("{}", result);

    Ok(())
} 

fn check_vertical(contents: Vec<Vec<char>>) -> i32 {
    
}

fn check_horizontal(contents: Vec<Vec<char>>) -> i32 {
    
}
