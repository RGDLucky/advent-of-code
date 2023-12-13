use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut galaxies: Vec<Vec<usize>> = vec![];
    let mut field: Vec<Vec<char>> = vec![];
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for line in reader.lines() {
        let new_line = line.unwrap();
        field.push(new_line.chars().collect());
    }

    for _i in 0..field.len() { empty_rows.push(2); }
    for _i in 0..field[0].len() { empty_cols.push(2); }

    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] == '#' {
                empty_rows[row] = 1;
                empty_cols[col] = 1;
                galaxies.push(vec![row, col]);
            }
        }
    }

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let point1: &Vec<usize> = &galaxies[i];
            let point2: &Vec<usize> = &galaxies[j];
            let mut start;
            let mut end;

            if point1[0] > point2[0] { start = point2[0]; end = point1[0]; }
            else { start = point1[0]; end = point2[0]; }
            for k in start..end { result += empty_rows[k]; }

            if point1[1] > point2[1] { start = point2[1]; end = point1[1]; }
            else { start = point1[1]; end = point2[1]; }
            for k in start..end { result += empty_cols[k]; }
        }
    }

    println!("{}", result);

    Ok(())
}    
