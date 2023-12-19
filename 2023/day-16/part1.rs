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
    
    let mut traversal = Traversal::new(contents);
    traversal.traverse(0, 0, Direction::Right);
    println!("{}", traversal.get_len());

    Ok(())
}  

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

// Find a way to detect cycles
//
struct Traversal {
    contents: Vec<Vec<char>>,
    visited: Vec<Vec<usize>>,
    cycle: Vec<Vec<usize>>,
}

impl Traversal {
    fn new(new_contents: Vec<Vec<char>>) -> Self {
        let new_visited: Vec<Vec<usize>> = vec![];
        let new_cycle: Vec<Vec<usize>> = vec![];
        Traversal {
            contents: new_contents,
            visited: new_visited,
            cycle: new_cycle,
        }
    }

    fn traverse(&mut self, row: i32, col: i32, direction: Direction) {
        //println!("{} {}", row, col);
        if row < 0 || col < 0 { return; }
        let temp_row: usize = row.try_into().unwrap();
        let temp_col: usize = col.try_into().unwrap();
        if row < 0 || row >= self.contents.len().try_into().unwrap() || col < 0 || temp_col >= self.contents[0].len() { return; }
        // let value: &'a str = (row.to_string() + "," + &col.to_string()).as_str();
        let dir;
        match direction {
            Direction::Up => dir = 1,
            Direction::Down => dir = 2,
            Direction::Right => dir = 3,
            Direction::Left => dir = 4,
            //_ => panic!(),
        }
        if self.cycle.contains(&vec![temp_row, temp_col, dir]) { return; }
        self.cycle.push(vec![temp_row, temp_col, dir]);
        if !self.visited.contains(&vec![row.try_into().unwrap(), col.try_into().unwrap()]) {  self.visited.push(vec![row.try_into().unwrap(), col.try_into().unwrap()]); }
        
        match (direction, self.contents[temp_row][temp_col]) {
            (Direction::Up, '|') => self.traverse(row - 1, col, Direction::Up),
            (Direction::Up, '.') => self.traverse(row - 1, col, Direction::Up),
            (Direction::Up, '/') => self.traverse(row, col + 1, Direction::Right),
            (Direction::Up, '\\') => self.traverse(row, col - 1, Direction::Left),
            (Direction::Up, '-') => {
                self.traverse(row, col + 1, Direction::Right);
                self.traverse(row, col - 1, Direction::Left);
            },
        
            (Direction::Down, '|') => self.traverse(row + 1, col, Direction::Down),
            (Direction::Down, '.') => self.traverse(row + 1, col, Direction::Down),
            (Direction::Down, '/') => self.traverse(row, col - 1, Direction::Left),
            (Direction::Down, '\\') => self.traverse(row, col + 1, Direction::Right),
            (Direction::Down, '-') => {
                self.traverse(row, col + 1, Direction::Right);
                self.traverse(row, col - 1, Direction::Left);
            },

            (Direction::Right, '-') => self.traverse(row, col + 1, Direction::Right),
            (Direction::Right, '.') => self.traverse(row, col + 1, Direction::Right),
            (Direction::Right, '/') => self.traverse(row - 1, col, Direction::Up),
            (Direction::Right, '\\') => self.traverse(row + 1, col, Direction::Down),
            (Direction::Right, '|') => {
                self.traverse(row + 1, col, Direction::Down);
                self.traverse(row - 1, col, Direction::Up);
            },

            (Direction::Left, '.') => self.traverse(row, col - 1, Direction::Left),
            (Direction::Left, '-') => self.traverse(row, col - 1, Direction::Left),
            (Direction::Left, '/') => self.traverse(row + 1, col, Direction::Down),
            (Direction::Left, '\\') => self.traverse(row - 1, col, Direction::Up),
            (Direction::Left, '|') => {
                self.traverse(row + 1, col, Direction::Down);
                self.traverse(row - 1, col, Direction::Up);
            },
        
            _ => panic!(),
        }
    }

    fn get_len(&self) -> usize {
        //for i in 0..self.visited.len() {
            //println!("row: {} col: {}", self.visited[i][0], self.visited[i][1]);
        //}
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
