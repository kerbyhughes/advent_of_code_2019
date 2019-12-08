use std::fs::File;
use std::io::prelude::*;

pub fn count_up_fuel(input_file: &str) {
    let mut file = File::open(input_file)
        .expect("Couldn't open file");
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut fuel: i64 = 0;
    let mut total: i64 = 0;

    let lines = content.split("\n");
    for line in lines {
        if line == "" { continue };

        let mass = line.parse::<f64>().unwrap();
        fuel = (mass / 3.0).floor() as i64 - 2; 

        while (fuel > 0) {
            total += fuel;
            fuel = (fuel as f64 / 3.0).floor() as i64 - 2;
        }
    }

    println!("Total fuel required: {}", total);
}
