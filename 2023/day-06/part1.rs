use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 1;
    let distance_and_time: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line"))
        .collect();
    let time_line: Vec<&str> = distance_and_time[0].split(":").collect();
    let distance_line: Vec<&str> = distance_and_time[1].split(":").collect();
    let times: Vec<&str> = time_line[1].split("  ").collect();
    let distances: Vec<&str> = distance_line[1].split("  ").collect();

    for i in 0..times.len() {
        let time = times[i].parse::<i64>().unwrap();
        println!("{}", time);
        let distance = distances[i].parse::<i64>().unwrap();
        let mut wins = 0;
        for j in 1..time {
            if j * (time - j) > distance { wins += 1; }
        }
        result *= wins;
    }
    println!("{}", result);

    Ok(())
}   
