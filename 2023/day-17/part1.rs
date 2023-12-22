use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::convert::TryInto;

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut contents: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let new_line = line.unwrap();
        contents.push(new_line.chars().collect());
    }

    let visited: Vec<Vec<i32>> = vec![];
    let result = traverse(0, 0, contents, Direction::Right, 0, visited); 
    println!("{}", result);

    Ok(())
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn traverse(row: i32, col: i32, contents: Vec<Vec<char>>, direction: Direction, dir_count: usize, mut visited: Vec<Vec<i32>>) -> usize {
    if row < 0 || col < 0 || visited.iter().any(|v| *v == vec![row, col]) {
        return 0;
    }

    //println!("row: {} col: {}", row, col);

    let temp_row: usize = row.try_into().unwrap();
    let temp_col: usize = col.try_into().unwrap();

    if row < 0 || temp_row >= contents.len() as usize || col < 0 || temp_col >= contents[0].len() as usize {
        return 0;
    }

    let value = contents[temp_row][temp_col] as usize - 48;

    if temp_row == contents.len() - 1 && temp_col == contents[0].len() - 1 {
        println!("{}", value);
        return value;
    }

    visited.push(vec![row, col]);

    let mut values: Vec<usize> = vec![];

    match direction {
        Direction::Up => {
            if dir_count < 3 {
                values.push(traverse(row - 1, col, contents.clone(), Direction::Up, dir_count + 1, visited.clone()));
            }
            values.push(traverse(row, col + 1, contents.clone(), Direction::Right, 0, visited.clone()));
            values.push(traverse(row, col - 1, contents.clone(), Direction::Left, 0, visited.clone()));
        }
        Direction::Down => {
            if dir_count < 3 {
                values.push(traverse(row + 1, col, contents.clone(), Direction::Down, dir_count + 1, visited.clone()));
            }
            values.push(traverse(row, col + 1, contents.clone(), Direction::Right, 0, visited.clone()));
            values.push(traverse(row, col - 1, contents.clone(), Direction::Left, 0, visited.clone()));
        }
        Direction::Right => {
            if dir_count < 3 {
                values.push(traverse(row, col + 1, contents.clone(), Direction::Right, dir_count + 1, visited.clone()));
            }
            values.push(traverse(row - 1, col, contents.clone(), Direction::Up, 0, visited.clone()));
            values.push(traverse(row + 1, col, contents.clone(), Direction::Down, 0, visited.clone()));
        }
        Direction::Left => {
            if dir_count < 3 {
                values.push(traverse(row, col - 1, contents.clone(), Direction::Left, dir_count + 1, visited.clone()));
            }
            values.push(traverse(row + 1, col, contents.clone(), Direction::Down, 0, visited.clone()));
            values.push(traverse(row - 1, col, contents.clone(), Direction::Up, 0, visited.clone()));
        }
    }

    let mut min = values[0];
    for i in 0..values.len() {
        if values[i] != 0 && values[i] < min {
            min = values[i];
        }
    }

    return value + min;
}




/*
fn traverse(row: i32, col: i32, contents: Vec<Vec<char>>, direction: Direction, dir_count: usize, visited:Vec<Vec<i32>>) -> usize {
    if row < 0 || col < 0 || visited.contains(&vec![row, col]) { return 0; }
    println!("row: {} col: {}", row, col);
    let temp_row: usize = row.try_into().unwrap();
    let temp_col: usize = col.try_into().unwrap();
    if row < 0 || temp_row > contents.len() - 1 || col < 0 || temp_col > contents.len() - 1 { return 0; }
    let value = contents[temp_row][temp_col] as usize - 48;
    // println!("{}", value);
    if temp_row == contents.len() - 1 && temp_col == contents[0].len() - 1 { println!("{}", value); return value; }

    let mut new_visited: Vec<Vec<i32>> = vec![vec![row, col]];
    for i in 0..visited.len() { new_visited.push(vec![visited[i][0], visited[i][1]]); }

    let mut values: Vec<usize> = vec![];
    match direction {
        Direction::Up => {
            if dir_count < 3 { values.push(traverse(row - 1, col, contents.clone(), Direction::Up, dir_count + 1, new_visited.clone())); }
            values.push(traverse(row, col + 1, contents.clone(), Direction::Right, 0, new_visited.clone()));
            values.push(traverse(row, col - 1, contents.clone(), Direction::Left, 0, new_visited.clone()));
        },
        Direction::Down => {
            if dir_count < 3 { values.push(traverse(row + 1, col, contents.clone(), Direction::Down, dir_count + 1, new_visited.clone())); }
            values.push(traverse(row, col + 1, contents.clone(), Direction::Right, 0, new_visited.clone()));
            values.push(traverse(row, col - 1, contents.clone(), Direction::Left, 0, new_visited.clone()));
        },
        Direction::Right => {
            if dir_count < 3 { values.push(traverse(row, col + 1, contents.clone(), Direction::Right, dir_count + 1, new_visited.clone())); }
            values.push(traverse(row - 1, col, contents.clone(), Direction::Up, 0, new_visited.clone()));
            values.push(traverse(row + 1, col, contents.clone(), Direction::Down, 0, new_visited.clone()));
        },
        Direction::Left => {
            if dir_count < 3 { values.push(traverse(row, col - 1, contents.clone(), Direction::Left, dir_count + 1, new_visited.clone())); }
            values.push(traverse(row + 1, col, contents.clone(), Direction::Down, 0, new_visited.clone()));
            values.push(traverse(row - 1, col, contents.clone(), Direction::Up, 0, new_visited.clone()));
        },
        //_ => panic!(),
    }

    let mut min = values[0];
    for i in 0..values.len() { if values[i] != 0 && values[i] < min { min = values[i];} }
    // println!("{}", value + min);
    return value + min;
} */
