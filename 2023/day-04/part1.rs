use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let new_line = line.unwrap();
        let game: Vec<&str> = new_line.split(": ").collect();
        let numbers: Vec<&str> = game[1].split(" | ").collect();
        let winning_numbers: Vec<&str> = numbers[0].split(" ").collect();
        let game_numbers: Vec<&str> = numbers[1].split(" ").collect();
        let mut increment = 0;
        
        for i in 0..game_numbers.len() {
            // println!("{}", game_numbers[i]);
            for j in 0..winning_numbers.len() {
                // println!("{} {}", game_numbers[i], winning_numbers[j]);
                if game_numbers[i] == winning_numbers[j] {
                    //println!("{}", game_numbers[i]);
                    //println!("yes");
                    if increment == 0 {
                        increment += 1;
                    } else {
                        increment *= 2;
                    }
                } //else { println!("no"); }
            }
        }
        result += increment;
        println!("{}", result);
    }

    println!("{}", result);

    Ok(())
}    
