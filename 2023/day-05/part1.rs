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
    let mut seeds: Vec<&str> = seed_line[1].split(" ").collect();
    let seed_line2: Vec<&str> = map[1].split(" ").collect();
    for i in 0..seed_line2.len() { seeds.push(seed_line2[i]); }
    let mut index = 3;

    // use hash map to get value seed to soil
    let mut seed_to_soil = HashMap::new();
    while index < map.len() {
        println!("{}", index);

        if map[index] == "" { break; }
        let convertion: Vec<&str> = map[index].split(" ").collect();
        let mut seed = convertion[1].parse::<i64>().unwrap();
        let mut soil = convertion[0].parse::<i64>().unwrap();

        for _i in 0..convertion[2].parse::<i64>().unwrap() {
            seed_to_soil.insert(seed, soil);
            seed += 1;
            soil += 1;
        }
        index += 1;
    }

    let mut soils: Vec<i64> = vec![];

    for i in 0..seeds.len() {
        let seed = seeds[i].parse::<i64>().unwrap(); 
        if seed_to_soil.contains_key(&seed) {
            soils.push(seed_to_soil[&seed]);
        } else {
            soils.push(seed);
        }
    }
    
    println!("Part 1 done");



    index += 2;
    // for i in 0..soils.len() { println!("{} {}", seeds[i], soils[i]); }

    // use hash map to get value seed to soil
    let mut soil_to_fertilizer = HashMap::new();
    while index < map.len() {
        if map[index] == "" { break; }
        let convertion: Vec<&str> = map[index].split(" ").collect();
        let mut soil = convertion[1].parse::<i64>().unwrap();
        let mut fertilizer = convertion[0].parse::<i64>().unwrap();

        for _i in 0..convertion[2].parse::<i64>().unwrap() {
            soil_to_fertilizer.insert(soil, fertilizer);
            fertilizer += 1;
            soil += 1;
        }
        index += 1;
    }

    let mut fertilizers: Vec<i64> = vec![];

    for i in 0..soils.len() {
        let soil = soils[i];
        if soil_to_fertilizer.contains_key(&soil) {
            fertilizers.push(soil_to_fertilizer[&soil]);
        } else {
            fertilizers.push(soil);
        }
    }

    println!("Part 2 done");

    index += 2;

    // use hash map to get value seed to soil
    let mut fertilizer_to_water = HashMap::new();
    while index < map.len() {
        if map[index] == "" { break; }
        let convertion: Vec<&str> = map[index].split(" ").collect();
        let mut fertilizer = convertion[1].parse::<i64>().unwrap();
        let mut water = convertion[0].parse::<i64>().unwrap();

        for _i in 0..convertion[2].parse::<i64>().unwrap() {
            fertilizer_to_water.insert(fertilizer, water);
            fertilizer += 1;
            water += 1;
        }
        index += 1;
    }

    let mut waters: Vec<i64> = vec![];

    for i in 0..fertilizers.len() {
        let fertilizer = fertilizers[i];
        if fertilizer_to_water.contains_key(&fertilizer) {
            waters.push(fertilizer_to_water[&fertilizer]);
        } else {
            waters.push(fertilizer);
        }
    }

    println!("Part 3 done");

    index += 2;

    // use hash map to get value seed to soil
    let mut water_to_light = HashMap::new();
    while index < map.len() {
        if map[index] == "" { break; }
        let convertion: Vec<&str> = map[index].split(" ").collect();
        let mut water = convertion[1].parse::<i64>().unwrap();
        let mut light = convertion[0].parse::<i64>().unwrap();

        for _i in 0..convertion[2].parse::<i64>().unwrap() {
            water_to_light.insert(water, light);
            water += 1;
            light += 1;
        }
        index += 1;
    }

    let mut lights: Vec<i64> = vec![];

    for i in 0..waters.len() {
        let water = waters[i];
        if water_to_light.contains_key(&water) {
            lights.push(water_to_light[&water]);
        } else {
            lights.push(water);
        }
    }

    println!("Part 4 done");

    index += 2;

    // use hash map to get value seed to soil
    let mut light_to_temperature = HashMap::new();
    while index < map.len() {
        if map[index] == "" { break; }
        let convertion: Vec<&str> = map[index].split(" ").collect();
        let mut light = convertion[1].parse::<i64>().unwrap();
        let mut temperature = convertion[0].parse::<i64>().unwrap();

        for _i in 0..convertion[2].parse::<i64>().unwrap() {
            light_to_temperature.insert(light, temperature);
            light += 1;
            temperature += 1;
        }
        index += 1;
    }

    let mut temperatures: Vec<i64> = vec![];

    for i in 0..lights.len() {
        let light = lights[i];
        if light_to_temperature.contains_key(&light) {
            temperatures.push(light_to_temperature[&light]);
        } else {
            temperatures.push(light);
        }
    }

    println!("Part 5 done");

    index += 2;

    // use hash map to get value seed to soil
    let mut temperature_to_humidity = HashMap::new();
    while index < map.len() {
        if map[index] == "" { break; }
        let convertion: Vec<&str> = map[index].split(" ").collect();
        let mut temperature = convertion[1].parse::<i64>().unwrap();
        let mut humidity = convertion[0].parse::<i64>().unwrap();

        for _i in 0..convertion[2].parse::<i64>().unwrap() {
            temperature_to_humidity.insert(temperature, humidity);
            temperature += 1;
            humidity += 1;
        }
        index += 1;
    }

    let mut humidities: Vec<i64> = vec![];

    for i in 0..temperatures.len() {
        let temperature = temperatures[i];
        if temperature_to_humidity.contains_key(&temperature) {
            humidities.push(temperature_to_humidity[&temperature]);
        } else {
            humidities.push(temperature);
        }
    }

    println!("Part 6 done");

    index += 2;

    // use hash map to get value seed to soil
    let mut humidity_to_location = HashMap::new();
    while index < map.len() {
        if map[index] == "" { break; }
        let convertion: Vec<&str> = map[index].split(" ").collect();
        let mut humidity = convertion[1].parse::<i64>().unwrap();
        let mut location = convertion[0].parse::<i64>().unwrap();

        for _i in 0..convertion[2].parse::<i64>().unwrap() {
            humidity_to_location.insert(humidity, location);
            humidity += 1;
            location += 1;
        }
        index += 1;
    }

    let mut locations: Vec<i64> = vec![];

    for i in 0..humidities.len() {
        let humidity = humidities[i];
        if humidity_to_location.contains_key(&humidity) {
            locations.push(humidity_to_location[&humidity]);
        } else {
            locations.push(humidity);
        }
    }

    let mut result = locations[0];
    for i in 1..locations.len() {
        if locations[i] < result { result = locations[i]; }
    }

    println!("{}", result);

    Ok(())
}    
