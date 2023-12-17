use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Ideas/Notes: get all lines into a an array, then check for vertical and if not check for
// horizontal reflection

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut contents: Vec<String> = vec![];

    for line in reader.lines() {
        contents.push(line.unwrap());
    }

    let mut index = 0;
    while index < contents.len() {
        let mut grid: Vec<Vec<char>> = vec![];
        while contents[index] != "" {
            grid.push(contents[index].chars().collect());
            index += 1;
        }
        result += check_vertical(grid()) + check_horizontal(grid.clone());
        index += 1;
    }

    println!("{}", result);

    Ok(())
} 

fn check_vertical(contents: Vec<Vec<char>>) -> i32 {
    for i in 0..contetns[0].len() - 1 {
        let mut left = i;
        let mut right = i + 1;
        let mut valid = true;
        while left >= 0 && right < content.len() {
            for j in 0..contents.len() {
                if contents[j] != contents[i] { valid = false; break; }
            }
            if !valid { break; }
        }
        if valid { return i + 1; }
    }
    return 0;
}

fn check_horizontal(contents: Vec<Vec<char>>) -> i32 {
    
}
