use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Ideas/Thought: add periods to buffer the top bottom left and right. put everything in 2d vector
// of chars and find the S position and then use bfs to find a loop.  Stoping point in bfs is when
// it runs into another digit then get that digit and return it
//
// New Idea: use dfs that when loops back to S prints count / 0, make a dfs structure with methods,
// use an enum for direction, each iteration will have a direction dictating what to look for

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut field: Vec<Vec<char>> = vec![];
    let mut buffer: Vec<char> = vec![];
    for i in 0..7 { buffer.push('.'); }

    let mut sx = 0;
    let mut sy = 0;
    let mut line_index = 0;

    // Create field
    for line in reader.lines() {
        let line_chars: Vec<char> = line?.chars().collect();
        let mut new_vec: Vec<char> = vec![];
        new_vec.push('.');
        for _i in 0..line_chars.len() {
            new_vec.push(line_chars[i]);
            if line_chars[i] == 'S' {
                sx = i;
                sy = line_index;
            }
        }

        new_vec.push('.');
        field.push(new_vec);
        line_index += 1;
    }

    // BFS to find path
    let level = 0;
    let queue: Vec<Vec<usize>> = vec![];
    let start_index: Vec<usize> = vec![sx, sy, level];
    level += 1;
    // check all sides of S for valid directions and add them to the queue
    // north
    if field[sx][sy+1] == '|' || field[sx][sy+1] == '7' || field[sx][sy+1] == 'F' { queue.push(vec![sx, sy+1, level]);
    // south
    if field[sx][sy-1] == '|' || field[sx][sy-1] == 'L' || field[sx][sy-1] == 'J' { queue.push(vec![sx, sy-1, level]);
    // east
    if field[sx-1][sy] == '-' || field[sx-1][sy] == 'F' || field[sx-1][sy] == 'L' { queue.push(vec![sx-1], sy, level]);
    // west
    if field[sx+1][sy] == '-' || field[sx+1][sy] == 'J' || field[sx+1][sy] == '7' { queue.push(vec![sx-1, sy, level]);
    loop {
        let (x, y) = queue[0]; 
        queue.remove(0);
        level += 1;

    }


    println!("S at: {} {}", sx, sy);
    //for line in field { println!("{}", line ); }

    println!("{}", result);

    Ok(())
}    

struct dfs {
    feild: Vec<Vec<char>>,
    sx: usize,
    sy: usize,
}

impl dfs {
    fn new(new_field: Vec<Vec<char>>, new_sx: usize, new_sy: usize) -> Self {
        Self {
            field: new_field
            sx: new_sx,
            sy: new_sy,
        }
    }

    fn search() -> usize {
        
    }

    fn searchHelper() -> usize {

    }
}
