use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Notes/Ideas: Split each line by " " so that you can get the numbers of each spring,
// iterate through row and update position in numbers so you can find ? and then find
// possible arrangements
// find possible arrangements by trying all possible combinations

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let new_line = line.unwrap();
        let contents: Vec<&str> = new_line.split(" ").collect();
        let conditions: Vec<char> = contents[0].chars();
        let sizes: Vec<&str> = contents[1].split(",").collect();
        let index = 0;

        // iterate through conditions and find all possible solutions for ????
        for i in 0..conditions.len() {

        }
    }

    println!("{}", result);

    Ok(())
} 
