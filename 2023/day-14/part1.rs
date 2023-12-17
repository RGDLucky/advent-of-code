use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut contents: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let new_line = line.unwrap();
        contents.push(new_line.chars().collect());
    }

    for i in 0..contents.len() {
        for j in 0..contents[i].len() {
            if contents[i][j] == 'O' {
                let mut index = i;
                while index > 0 && contents[index - 1][j] == '.' {
                    let temp = contents[index][j];
                    contents[index][j] = contents[index - 1][j];
                    contents[index - 1][j] = temp;
                    index -= 1;
                }
                result += contents.len() - index;
            }
        }
    }

    println!("{}", result);

    Ok(())
}   
