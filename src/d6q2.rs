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

    let mut min_transfers = -1;
    let mut start = orbits["YOU"];
    let mut end = orbits["SAN"];
    let mut transfers_from_start = 0;
    let mut transfers_from_end = 0;

    while start != "COM" {
        end = orbits["SAN"];
        transfers_from_end = 0;

        while end != "COM" {
            if start == end {
                let transfers = transfers_from_start + transfers_from_end;
                if min_transfers < 0 || transfers < min_transfers {
                    min_transfers = transfers;
                }
            }

            transfers_from_end += 1;
            end = orbits[end];
        }

        transfers_from_start += 1;
        start = orbits[start];
    }

    println!("Min transfers: {}", min_transfers);
}