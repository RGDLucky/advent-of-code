use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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
        while index < contents.len() && contents[index] != "" {
            grid.push(contents[index].chars().collect());
            index += 1;
        }
        result += check_vertical(grid.clone()) + check_horizontal(grid.clone());
        index += 1;
    }

    println!("{}", result);

    Ok(())
} 

fn check_vertical(contents: Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for i in 0..contents[0].len() - 1 {
        let mut left = i;
        let mut right = i + 1;
        let mut valid = true;
        while right < contents[0].len() {
            for j in 0..contents.len() {
                if contents[j][left] != contents[j][right] { valid = false; break; }
            }
            if !valid { break; }
            if left == 0 { break; }
            left -= 1;
            right += 1;
        }
        if valid { result = i + 1; }
    }
    return result;
}

fn check_horizontal(contents: Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for i in 0..contents.len() - 1 {
        let mut top = i;
        let mut bottom = i + 1;
        let mut valid = true;
        while bottom < contents.len() {
            if contents[top] != contents[bottom] { valid = false; break; }
            if top == 0 { break; }
            top -= 1;
            bottom += 1;
        }
        if valid { result = (i + 1) * 100; }
    }
    return result;
}
