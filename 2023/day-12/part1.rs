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

// recursive function that first raverse until the end of the line then constructs a vec that is
// added to with each possible combination to each element in the vec
fn get_combinations(index: usize, Vec<char> line) -> Vec<String> {
    if index >= line.len() { return vec![String::from("")]; }
    let combos: Vec<String> = get_combinations(index + 1, line);
    if line[index] == '#' || line[index] == '.' { return combos: }

    let new_combos: Vec<String> = vec![];
    for i in 0..combos.len() {
        new_combos.push("#" + combos[i]);
        new_combos.push("." + combos[i]);
    }

    return new_combos;
}

fn check_combinations(combos: Vec<String>, nums: Vec<&str>) -> usize {
    let mut result = 0;
    for i in 0..combos.len() {
        let line: Vec<&str> = combos[i].split(".");
        let mut num_index = 0;
        let mut is_valid = 1;
        for i in 0..line.len() {
            if line[i] == "" { continue; }
            if line[i].len() != nums[num_index].parse::<i32>().unwrap() { is_valid = 0; break; }
            num_index += 1;
        }
        result += is_valid;
    }
}
