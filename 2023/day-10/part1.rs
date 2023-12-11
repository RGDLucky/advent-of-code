use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Ideas/Thought: add periods to buffer the top bottom left and right. put everything in 2d vector
// of chars and find the S position and then use bfs to find a loop.  Stoping point in bfs is when
// it runs into another digit then get that digit and return it

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
    let queue: Vec<Vec<usize>> = vec![];
    let start_index: Vec<usize> = vec![sx, sy];
    // check all sides of S for valid directions
    loop {
        let (x, y) = queue[0]; 
        queue.remove(0);

    }


    println!("S at: {} {}", sx, sy);
    //for line in field { println!("{}", line ); }

    println!("{}", result);

    Ok(())
}    
