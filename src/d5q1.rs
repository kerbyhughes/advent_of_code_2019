use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn process(input_file: &str) -> Result<i32, &str> {
    let mut file = File::open(input_file)
        .expect("Couldn't open file");
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let mut vals = vec!{};
    for num in content.trim().split(",") {
        let int = num.parse::<i32>().unwrap();
        vals.push(int);
    }

    // test
    // vals = vec!{1101, 2003, 11111, 1002};

    compute(&mut vals);

    Ok(*vals.get(0).unwrap())
}

#[derive(PartialEq)]
enum Mode {
    POSITION = 0,
    IMMEDIATE = 1
}

fn compute(arr: &mut Vec<i32>) {
    let mut ip: usize = 0; // instruction pointer, address of current instruction

    let mut param_1_mode = Mode::POSITION;
    let mut param_2_mode = Mode::POSITION;
    let mut param_3_mode = Mode::POSITION;

    while ip < arr.len() {
        println!("{}", arr[ip]);

        let op1_ip: usize;
        if param_1_mode == Mode::POSITION{
            op1_ip = arr[ip + 1] as usize;
        } else {
            op1_ip = ip + 1;
        }

        let op2_ip: usize;
        if param_2_mode == Mode::POSITION{
            op2_ip = arr[ip + 2] as usize;
        } else {
            op2_ip = ip + 2;
        }

        let target_ip: usize;
        if param_3_mode == Mode::POSITION{
            target_ip = arr[ip + 3] as usize;
        } else {
            target_ip = ip + 3;
        }

        match arr[ip] {
            // add
            1 => {
                arr[target_ip] = arr[op1_ip] + arr[op2_ip];

                ip += 4;

                param_1_mode = Mode::POSITION;
                param_2_mode = Mode::POSITION;
                param_3_mode = Mode::POSITION;
            },
           
            // multiply
            2 => {
                arr[target_ip] = arr[op1_ip] * arr[op2_ip];

                ip += 4;

                param_1_mode = Mode::POSITION;
                param_2_mode = Mode::POSITION;
                param_3_mode = Mode::POSITION;
            },

            // input
            3 => {
                let stdin = io::stdin();
                let mut lines = stdin.lock().lines();
                let input = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

                arr[target_ip] = input;

                ip += 2;

                param_1_mode = Mode::POSITION;
                param_2_mode = Mode::POSITION;
                param_3_mode = Mode::POSITION;
            },

            // output
            4 => {
                let output = arr[op1_ip];
                println!("Output: {}", output);

                ip += 2;

                param_1_mode = Mode::POSITION;
                param_2_mode = Mode::POSITION;
                param_3_mode = Mode::POSITION;
            },
            
            // halt 
            99 => return,

            // decode 4 or 5 digit op code,
            // don't move ip
            _  => {
                let de = arr[ip] % 100;
                let abc = arr[ip] / 100;
                let c = abc % 10;
                let ab = abc / 10;
                let b = ab % 10;
                let a = ab / 10;
                
                //println!("a {} b {} c {} de {}", a, b, c, de);
                
                arr[ip] = de;

                match c {
                    c if c == Mode::IMMEDIATE as i32 => param_1_mode = Mode::IMMEDIATE,
                    _ => param_1_mode = Mode::POSITION
                }
                match b {
                    b if b == Mode::IMMEDIATE as i32 => param_2_mode = Mode::IMMEDIATE,
                    _ => param_2_mode = Mode::POSITION
                }
                match a {
                    a if a == Mode::IMMEDIATE as i32 => param_3_mode = Mode::IMMEDIATE,
                    _ => param_3_mode = Mode::POSITION
                }
            }
        }
    }
}



