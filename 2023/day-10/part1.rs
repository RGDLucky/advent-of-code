use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut field: Vec<Vec<char>> = vec![];
    let mut buffer: Vec<char> = vec![];
    for _i in 0..7 { buffer.push('.'); }
    field.push(buffer.clone());

    let mut sx = 0;
    let mut sy = 0;
    let mut line_index = 0;

    // Create field
    for line in reader.lines() {
        let line_chars: Vec<char> = line?.chars().collect();
        let mut new_vec: Vec<char> = vec![];
        new_vec.push('.');
        for i in 0..line_chars.len() {
            new_vec.push(line_chars[i]);
            if line_chars[i] == 'S' {
                sx = line_index + 1;
                sy = i + 1;
            }
        }

        new_vec.push('.');
        field.push(new_vec);
        line_index += 1;
    }
    field.push(buffer);

    let dfs = DFS::new(field, sx, sy);
    let result = dfs.search();

    println!("{}", result / 4);

    Ok(())
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    DeadEnd,
}

struct DFS {
    field: Vec<Vec<char>>,
    sr: usize,
    sc: usize,
}

impl DFS {

    fn new(new_field: Vec<Vec<char>>, new_sr: usize, new_sc: usize) -> Self {
        Self {
            field: new_field,
            sr: new_sr,
            sc: new_sc,
        }
    }

    fn search(&self) -> usize {
        let up = Self::check_up(self.field[self.sr - 1][self.sc]);
        let down = Self::check_down(self.field[self.sr + 1][self.sc]);
        let right = Self::check_right(self.field[self.sr][self.sc + 1]);
        let left = Self::check_left(self.field[self.sr][self.sc - 1]);
        
        return self.search_helper(self.sr - 1, self.sc, up, 1) + self.search_helper(self.sr + 1, self.sc, down, 1) + self.search_helper(self.sr, self.sc + 1, right, 1) + self.search_helper(self.sr, self.sc - 1, left, 1);
    }

    fn search_helper(&self, row: usize, col: usize, direction: Direction, count: usize) -> usize {
        if self.field[row][col] == 'S' { return count; }
        if direction == Direction::DeadEnd { return 0; }
        let mut new_row = row;
        let mut new_col = col;
        let new_direction;
        match direction {
            Direction::Up => { new_row = row - 1; new_direction = Self::check_up(self.field[new_row][new_col]); },
            Direction::Down => { new_row = row + 1; new_direction = Self::check_down(self.field[new_row][new_col]); },
            Direction::Right => { new_col = col + 1; new_direction = Self::check_right(self.field[new_row][new_col]); },
            Direction::Left => { new_col = col - 1; new_direction = Self::check_left(self.field[new_row][new_col]); },
            Direction::DeadEnd => panic!(),
        }
        return self.search_helper(new_row, new_col, new_direction, count + 1); 
    }

    fn check_up(pipe: char) -> Direction {
        match pipe {
            '|' => return Direction::Up,
            '7' => return Direction::Left,
            'F' => return Direction::Right,
            _ => return Direction::DeadEnd,
        }
    }
    
    fn check_down(pipe: char) -> Direction {
        match pipe {
            '|' => return Direction::Down,
            'L' => return Direction::Right,
            'J' => return Direction::Left,
            _ => return Direction::DeadEnd,
        }
    }

    fn check_right(pipe: char) -> Direction {
        match pipe {
            '-' => return Direction::Right,
            '7' => return Direction::Down,
            'J' => return Direction::Up,
            _ => return Direction::DeadEnd,
        }
    }

    fn check_left(pipe: char) -> Direction {
        match pipe {
            '-' => return Direction::Left,
            'F' => return Direction::Down,
            'L' => return Direction::Up,
            _ => return Direction::DeadEnd,
        }
    }
}
