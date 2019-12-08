//use std::env;

mod d4q1;

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    */

    let result = d4q1::process();

    println!("--------------");
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e)
    }
}
