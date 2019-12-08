use std::fs::File;
use std::io::prelude::*;

pub fn process(input_file: &str) -> Result<i32, &str> {
    let mut file = File::open(input_file)
        .expect("Couldn't open file");
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let mut vals = Vec::new();
    for num in content.trim().split(",") {
        vals.push(num.parse::<i32>().unwrap());
    }

    compute(&mut vals); 

    println!("{:?}", vals);

    Ok(*vals.get(0).unwrap())
}

fn compute(arr: &mut Vec<i32>) {
    let mut pos = 0; 
    
    while pos < arr.len() {
        match arr[pos] {
            // add
            1 => {
                let op1_pos = arr[pos + 1];
                let op2_pos = arr[pos + 2];
                let target_pos = arr[pos + 3];
                arr[target_pos as usize] = arr[op1_pos as usize] + arr[op2_pos as usize];
            },
           
            // multiply
            2 => {
                let op1_pos = arr[pos + 1];
                let op2_pos = arr[pos + 2];
                let target_pos = arr[pos + 3];
                arr[target_pos as usize] = arr[op1_pos as usize] * arr[op2_pos as usize];
            },
            
            // halt 
            99 => return,

            _  => panic!("{} is not a valid op code!", arr[pos])

        }
        
        pos += 4;
    }
}



