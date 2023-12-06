use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    //let mut result = 0;
    //let mut maps: Vec<Vec<&str>> = vec![];
    //let mut new_map: Vec<&str> = vec![];
    //let mut map: Vec<Vec<&str>> = vec![]; 

    let map: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line"))
        .collect();
    let seed_line: Vec<&str> = map[0].split(": ").collect();
    let seeds: Vec<&str> = seed_line[1].split(" ").collect();
    let mut index = 3;

    // use hash map to get value seed to soil
    let mut seed_to_soil = HashMap::new();
    while index < map.len() {
        if map[index] == "" { break; }
        let convertion: Vec<&str> = map[index].split(" ").collect();
        let mut seed = convertion[1].parse::<i32>().unwrap();
        let mut soil = convertion[0].parse::<i32>().unwrap();

        for i in 0..convertion[2].parse::<i32>().unwrap() {
            seed += i;
            soil += i;
            seed_to_soil.insert(seed, soil);
        }
        index += 1;
    }

    let mut soils: Vec<i32> = vec![];

    for i in 0..seeds.len() {
        let seed = seeds[i].parse::<i32>().unwrap(); 
        if seed_to_soil.contains_key(&seed) {
            soils.push(seed_to_soil[&seed]);
        } else {
            soils.push(seed);
        }
    }

    // use hash map to get value seed to soil
    //let mut soil_to_fertilizer = HashMap::new();
    while index < map.len() {

        index += 1;
    }

    // use hash map to get value seed to soil
    while index < map.len() {

        index += 1;
    }

    // use hash map to get value seed to soil
    while index < map.len() {

        index += 1;
    }

    // use hash map to get value seed to soil
    while index < map.len() {

        index += 1;
    }

    // use hash map to get value seed to soil
    while index < map.len() {

        index += 1;
    }

    // use hash map to get value seed to soil
    while index < map.len() {

        index += 1;
    }
    //println!("{}", result);

    Ok(())
}    
