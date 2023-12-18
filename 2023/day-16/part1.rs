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
    
    let mut traversal = Traversal::new(contents);
    traversal.traverse(0, 0, Direction::Left);
    println!("{}", traversal.get_len());

    Ok(())
}  

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

// new idea make a struct and make traverse a method so you can store visited nodes somewhere
//
struct Traversal<'a> {
    contents: Vec<Vec<char>>,
    visited: Vec<&'a str>,
}

impl<'a> Traversal<'a> {
    fn new(new_contents: Vec<Vec<char>>) -> Self {
        let new_visited: Vec<& str> = vec![];
        Traversal {
            contents: new_contents,
            visited: new_visited,
        }
    }

    fn traverse(&mut self, row: usize, col: usize, direction: Direction) {
        if row < 0 || row >= self.contents.len() || col < 0 || col >= self.contents[row].len() { return; }
        let value::<'a> = (row.to_string() + "," + &col.to_string()).as_str();
        if !self.visited.contains(&value) { self.visited.push(value); }
        
        match (direction, self.contents[row][col]) {
            (Direction::Up, '|' | '.') => self.traverse(row - 1, col, Direction::Up),
            (Direction::Up, '/') => self.traverse(row, col + 1, Direction::Right),
            (Direction::Up, '\\') => self.traverse(row, col - 1, Direction::Left),
            (Direction::Up, '-') => {
                self.traverse(row, col + 1, Direction::Right);
                self.traverse(row, col - 1, Direction::Left);
            },
        
            (Direction::Down, '|' | '.') => self.traverse(row + 1, col, Direction::Down),
            (Direction::Down, '/') => self.traverse(row, col - 1, Direction::Left),
            (Direction::Down, '\\') => self.traverse(row, col + 1, Direction::Right),
            (Direction::Down, '-') => {
                self.traverse(row, col + 1, Direction::Right);
                self.traverse(row, col - 1, Direction::Left);
            },

            (Direction::Right, '-' | '.') => self.traverse(row, col + 1, Direction::Right),
            (Direction::Right, '/') => self.traverse(row - 1, col, Direction::Up),
            (Direction::Right, '\\') => self.traverse(row + 1, col, Direction::Down),
            (Direction::Right, '|') => {
                self.traverse(row + 1, col, Direction::Up);
                self.traverse(row - 1, col, Direction::Down);
            },

            (Direction::Left, '-' | '.') => self.traverse(row, col - 1, Direction::Left),
            (Direction::Left, '/') => self.traverse(row + 1, col, Direction::Down),
            (Direction::Left, '\\') => self.traverse(row - 1, col, Direction::Up),
            (Direction::Left, '|') => {
                self.traverse(row + 1, col, Direction::Up);
                self.traverse(row - 1, col, Direction::Down);
            },
        
            _ => panic!(),
        }
    }

    fn get_len(&self) -> usize {
        return self.visited.len();
    }
}
/*
fn traverse(row: usize, col: usize, direction: Direction, contents: Vec<Vec<char>>) -> Vec<&'static str> {
    let mut visited: Vec<&str> = vec![];
    if row < 0 || row >= contents.len() || col < 0 || col >= contents[row].len() { return visited.clone(); }
    // diferent values: . / \ | -
    match (direction, contents[row][col]) {
        (Direction::Up, '|' | '.') =>  visited = traverse(row - 1, col, Direction::Up, contents.clone()),
        (Direction::Up, '/') => visited = traverse(row, col + 1, Direction::Right, contents.clone()),
        (Direction::Up, '\\') => visited = traverse(row, col - 1, Direction::Left, contents.clone()),
        (Direction::Up, '-') => {
            visited = traverse(row, col + 1, Direction::Right, contents.clone());
            let temp: Vec<&str> = traverse(row, col - 1, Direction::Left, contents.clone());
            for i in 0..temp.len() { if !visited.contains(&temp[i]) { visited.push(temp[i]) }; }
        },
        
        (Direction::Down, '|' | '.') =>  visited = traverse(row + 1, col, Direction::Down, contents.clone()),
        (Direction::Down, '/') => visited = traverse(row, col - 1, Direction::Left, contents.clone()),
        (Direction::Down, '\\') => visited = traverse(row, col + 1, Direction::Right, contents.clone()),
        (Direction::Down, '-') => {
            visited = traverse(row, col + 1, Direction::Right, contents.clone());
            let temp: Vec<&str> = traverse(row, col - 1, Direction::Left, contents.clone());
            for i in 0..temp.len() { if !visited.contains(&temp[i]) { visited.push(temp[i]) }; }
        },

        (Direction::Right, '-' | '.') =>  visited = traverse(row, col + 1, Direction::Right, contents.clone()),
        (Direction::Right, '/') => visited = traverse(row - 1, col, Direction::Up, contents.clone()),
        (Direction::Right, '\\') => visited = traverse(row + 1, col, Direction::Down, contents.clone()),
        (Direction::Right, '|') => {
            visited = traverse(row + 1, col, Direction::Up, contents.clone());
            let temp: Vec<&str> = traverse(row - 1, col, Direction::Down, contents.clone());
            for i in 0..temp.len() { if !visited.contains(&temp[i]) { visited.push(temp[i]) }; }
        },

        (Direction::Left, '-' | '.') =>  visited = traverse(row, col - 1, Direction::Left, contents.clone()),
        (Direction::Left, '/') => visited = traverse(row + 1, col, Direction::Down, contents.clone()),
        (Direction::Left, '\\') => visited = traverse(row - 1, col, Direction::Up, contents.clone()),
        (Direction::Left, '|') => {
            visited = traverse(row + 1, col, Direction::Up, contents.clone());
            let temp: Vec<&str> = traverse(row - 1, col, Direction::Down, contents.clone());
            for i in 0..temp.len() { if !visited.contains(&temp[i]) { visited.push(temp[i]) }; }
        },
        
        _ => panic!(),
    }
    let value = row.to_string() + "," + &col.to_string();
    let new_value = value.as_str();
    visited.push(new_value.clone());
    return visited.clone();
} */
