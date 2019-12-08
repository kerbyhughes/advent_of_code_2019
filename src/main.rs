use std::env;

mod d3q2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    
    let result = d3q2::process(input_file);

    println!("--------------");
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e)
    }
}
