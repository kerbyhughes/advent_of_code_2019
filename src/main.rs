use std::env;

mod d5q1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let result = d5q1::process(input_file);

    println!("--------------");
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e)
    }
}
