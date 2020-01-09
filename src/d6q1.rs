use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn process(input_file: &str) -> Result<(), &str> {
    let mut file = File::open(input_file)
        .expect("Couldn't open file");

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut vals = vec!{};
    for pair in content.trim().split("\n") {
        vals.push(pair)
    }

    compute(&mut vals);

    Ok(())
}

fn compute(arr: &mut Vec<&str>) {
    let mut orbits = HashMap::new();

    for connection in arr {
        let parts: Vec<&str> = connection.split(")").collect();
        let (center, orbiter) = (parts[0], parts[1]);
        orbits.insert(orbiter, center);
    }

    let mut num_orbits = 0;
    for (k, _) in &orbits {
        num_orbits += count(k, &orbits);
    }

    println!("Orbits: {}", num_orbits);
}

fn count(planet: &str, orbits: &HashMap<&str, &str>) -> i32 {
    if planet == "COM" {
        return 0;
    }

    return 1 + count(orbits[planet], orbits);
}