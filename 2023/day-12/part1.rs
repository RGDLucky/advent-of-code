use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let new_line = line.unwrap();
        let contents: Vec<&str> = new_line.split(" ").collect();
        let conditions: Vec<char> = contents[0].chars().collect();
        let sizes: Vec<&str> = contents[1].split(",").collect();
        
        let combos: Vec<String> = get_combinations(0, conditions);
        result += check_combinations(combos, sizes);
    }

    println!("{}", result);

    Ok(())
} 

fn get_combinations(index: usize, line: Vec<char>) -> Vec<String> {
    if index >= line.len() { return vec![String::from("")]; }
    let combos: Vec<String> = get_combinations(index + 1, line.clone());

    let mut new_combos: Vec<String> = vec![];
    if line[index] == '#' || line[index] == '.' { 
        for i in 0..combos.len() { new_combos.push(String::from(line[index]) + &combos[i]); }
    }

    else {
        for i in 0..combos.len() {
            new_combos.push(String::from("#") + &combos[i]);
            new_combos.push(String::from(".") + &combos[i]);
        }
    }

    return new_combos;
}

fn check_combinations(combos: Vec<String>, nums: Vec<&str>) -> usize {
    let mut result = 0;
    for i in 0..combos.len() {
        let line: Vec<&str> = combos[i].split(".").collect();
        let mut num_index = 0;
        let mut is_valid = 1;
        for i in 0..line.len() {
            if line[i] == "" { continue; }
            if num_index >= nums.len() || line[i].len() != nums[num_index].parse::<usize>().unwrap() { is_valid = 0; break; }
            num_index += 1;
        }

        if num_index < nums.len() { is_valid = 0; }
        result += is_valid;
    }
    return result;
}
