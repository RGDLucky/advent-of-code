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

    let result = traverse(0, 0, contents, Direction::Right, 0);
    println!("{}", result);

    Ok(())
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn traverse(row: i32, col: i32, contents: Vec<Vec<char>>, direction: Direction, dir_count: usize) -> usize {
    if row < 0 || col < 0 { return 0; }
    let temp_row: usize = row.try_into().unwrap();
    let temp_col: usize = col.try_into().unwrap();
    if row < 0 || temp_row > contents.len() - 1 || col < 0 || temp_col > contents.len() - 1 { return 0; }
    let value = contents[temp_row][temp_col] as usize - 48;
    println!("{}", value);
    if temp_row == contents.len() - 1 && temp_col == contents[0].len() - 1 { return value; }
    let mut values: Vec<usize> = vec![];
    match direction {
        Direction::Up => {
            if dir_count < 3 { values.push(traverse(row - 1, col, contents.clone(), Direction::Up, dir_count + 1)); }
            values.push(traverse(row, col + 1, contents.clone(), Direction::Right, 0));
            values.push(traverse(row, col - 1, contents.clone(), Direction::Left, 0));
        },
        Direction::Down => {
            if dir_count < 3 { values.push(traverse(row + 1, col, contents.clone(), Direction::Down, dir_count + 1)); }
            values.push(traverse(row, col + 1, contents.clone(), Direction::Right, 0));
            values.push(traverse(row, col - 1, contents.clone(), Direction::Left, 0));
        },
        Direction::Right => {
            if dir_count < 3 { values.push(traverse(row, col + 1, contents.clone(), Direction::Right, dir_count + 1)); }
            values.push(traverse(row - 1, col, contents.clone(), Direction::Up, 0));
            values.push(traverse(row + 1, col, contents.clone(), Direction::Down, 0));
        },
        Direction::Left => {
            if dir_count < 3 { values.push(traverse(row, col - 1, contents.clone(), Direction::Left, dir_count + 1)); }
            values.push(traverse(row + 1, col, contents.clone(), Direction::Down, 0));
            values.push(traverse(row - 1, col, contents.clone(), Direction::Up, 0));
        },
        //_ => panic!(),
    }

    let mut min = values[0];
    for i in 0..values.len() { if values[i] != 0 && values[i] < min { min = values[i];} }
    return value + min;
}
