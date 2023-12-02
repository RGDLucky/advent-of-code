use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    for line in reader.lines() {
        let new_line = line.unwrap();
        let game: Vec<&str> = new_line.split(": ").collect();
        let game_contents = game[1].to_string();
        if check_game(game_contents) {
            let game_name: Vec<&str> = game[0].split(" ").collect();
            println!("{}", game_name[1]);
            result += game_name[1].parse::<i32>().unwrap();
        }
    }

    println!("{}", result);

    Ok(())
}

// split game game up and figure if it fits in a certain bounds
fn check_game(game: String) -> bool {
    for set in game.split("; ") {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for block in set.split(", ") {
            let block_split: Vec<&str> = block.split(" ").collect();
            match block_split[1] {
                "blue" => blue += block_split[0].parse::<i32>().unwrap(),
                "green" => green += block_split[0].parse::<i32>().unwrap(),
                "red" => red += block_split[0].parse::<i32>().unwrap(),
                _ => panic!(),
            }
        }

        if red > 12 || green > 13 || blue > 14 {
            return false;
        }
    }
    true
}
