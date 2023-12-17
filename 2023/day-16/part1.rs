use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut contents: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let new_line = line.unwrap();
        contents.push(new_line.chars().collect());
    }
    
    let visited = traverse(0, 0, Direction::Right, contents);
    println!("{}", visited.len());

    Ok(())
}  

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn traverse(row: usize, col: usize, direction: Direction, contents: Vec<Vec<char>>) -> Vec<&'static str> {
    let mut visited: Vec<&str> = vec![];
    if row < 0 || row >= contents.len() || col < 0 || col >= contents[row].len() { return visited; }
    // diferent values: . / \ | -
    match (direction, contents[row][col]) {
        (Up, '|' | '.') =>  visited = traverse(row - 1, col, Direction::Up, contents.clone()),
        (Up, '/') => visited = traverse(row, col + 1, Direction::Right, contents.clone()),
        (Up, '\\') => visited = traverse(row, col - 1, Direction::Left, contents.clone()),
        (Up, '-') => {
            visited = traverse(row, col + 1, Direction::Right, contents.clone());
            let temp: Vec<&str> = traverse(row, col - 1, Direction::Left, contents.clone());
            for i in 0..temp.len() { if !visited.contains(&temp[i]) { visited.push(temp[i]) }; }
        },
        
        (Down, '|' | '.') =>  visited = traverse(row + 1, col, Direction::Down, contents.clone()),
        (Down, '/') => visited = traverse(row, col - 1, Direction::Left, contents.clone()),
        (Down, '\\') => visited = traverse(row, col + 1, Direction::Right, contents.clone()),
        (Down, '-') => {
            visited = traverse(row, col + 1, Direction::Right, contents.clone());
            let temp: Vec<&str> = traverse(row, col - 1, Direction::Left, contents.clone());
            for i in 0..temp.len() { if !visited.contains(&temp[i]) { visited.push(temp[i]) }; }
        },

        (Right, '-' | '.') =>  visited = traverse(row, col + 1, Direction::Right, contents.clone()),
        (Right, '/') => visited = traverse(row - 1, col, Direction::Up, contents.clone()),
        (Right, '\\') => visited = traverse(row + 1, col, Direction::Down, contents.clone()),
        (Right, '|') => {
            visited = traverse(row + 1, col, Direction::Up, contents.clone());
            let temp: Vec<&str> = traverse(row - 1, col, Direction::Down, contents.clone());
            for i in 0..temp.len() { if !visited.contains(&temp[i]) { visited.push(temp[i]) }; }
        },

        (Left, '-' | '.') =>  visited = traverse(row, col - 1, Direction::Left, contents.clone()),
        (Left, '/') => visited = traverse(row + 1, col, Direction::Down, contents.clone()),
        (Left, '\\') => visited = traverse(row - 1, col, Direction::Up, contents.clone()),
        (Left, '|') => {
            visited = traverse(row + 1, col, Direction::Up, contents.clone());
            let temp: Vec<&str> = traverse(row - 1, col, Direction::Down, contents.clone());
            for i in 0..temp.len() { if !visited.contains(&temp[i]) { visited.push(temp[i]) }; }
        },
        
        _ => panic!(),
    }
    let value = row.to_string() + "," + &col.to_string();
    visited.push(&value);
    return visited;
}
