use std::fs::File;
use std::io::prelude::*;

pub fn process(input_file: &str) -> Result<i32, &str> {
    let mut file = File::open(input_file)
        .expect("Couldn't open file");
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let mut vals = vec!{0};

    //= 19690720 {
    for noun in {0..99} {
        for verb in {0..99} {

            vals.clear();
            for num in content.trim().split(",") {
                vals.push(num.parse::<i32>().unwrap());
            }

            vals[1] = noun;
            vals[2] = verb;

            compute(&mut vals);
            
            if *vals.get(0).unwrap() == 19690720 {
                println!("{}", 100 * noun + verb);
                ()
            }
        }
    }

    Ok(*vals.get(0).unwrap())
}

fn compute(arr: &mut Vec<i32>) {
    let mut ip = 0; // instruction pointer, address of current instruction 
    
    while ip < arr.len() {
        match arr[ip] {
            // add
            1 => {
                let op1_ip = arr[ip + 1];
                let op2_ip = arr[ip + 2];
                let target_ip = arr[ip + 3];
                arr[target_ip as usize] = arr[op1_ip as usize] + arr[op2_ip as usize];
            },
           
            // multiply
            2 => {
                let op1_ip = arr[ip + 1];
                let op2_ip = arr[ip + 2];
                let target_ip = arr[ip + 3];
                arr[target_ip as usize] = arr[op1_ip as usize] * arr[op2_ip as usize];
            },
            
            // halt 
            99 => return,

            _  => panic!("{} is not a valid op code!", arr[ip])

        }
        
        ip += 4;
    }
}



