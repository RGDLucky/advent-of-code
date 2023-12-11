use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let new_line = line.unwrap();
        let first_values_strings: Vec<&str> = new_line.split(" ").collect();
        let mut first_values: Vec<i32> = vec![];
        for i in 0..first_values_strings.len() { first_values.push(first_values_strings[i].parse::<i32>().unwrap()); }
        let mut values: Vec<Vec<i32>> = vec![];
        values.push(first_values);

        loop {
            let mut new_values: Vec<i32> = vec![];
            for i in 1..values[values.len() - 1].len() {
                new_values.push(values[values.len()-1][i] - values[values.len()-1][i-1]);
            }

            values.push(new_values.clone());
            let mut zeros = true;
            for num in new_values { if num != 0 { zeros = false; break; }}
            if zeros { break; }
        }

        for i in 0..values.len() {
            result += values[i][values[i].len()-1];
        }

    }

    println!("{}", result);

    Ok(())
}    
