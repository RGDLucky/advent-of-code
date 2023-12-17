use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    
    let mut result = 0;

    for line in reader.lines() {
        let new_line = line.unwrap();
        let contents: Vec<&str> = new_line.split(",").collect(); 
        for i in 0..contents.len() { result += hash(contents[i]); }
    }

    println!("{}", result);
    
    Ok(())
} 

fn hash(string: &str) -> u32 {
    let mut result = 0;
    for curr in string.chars() {
        result += curr as u32;
        result *= 17;
        result %= 256;
    }
    return result
}
