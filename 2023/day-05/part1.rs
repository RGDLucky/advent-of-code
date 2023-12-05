use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut maps: Vec<Vec<&str>> = vec![];
    let mut new_map: Vec<&str> = vec![];

    for line in reader.lines() {
        let new_line = line.unwrap();
        let temp = new_line.clone();
        if new_line == "" {
            maps.push(new_map);
            new_map = vec![];
            continue;
        }    
        new_map.push(&temp);
    }

    for map in maps {
        for nums in map {
            println!("{nums}");
        }
    }

    println!("{}", result);

    Ok(())
}    
