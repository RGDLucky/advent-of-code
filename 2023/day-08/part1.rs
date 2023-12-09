use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let number = get_number(line.unwrap());
        //println!("{}", number);
        result += number;
    }

    println!("{}", result);

    Ok(())
}   
