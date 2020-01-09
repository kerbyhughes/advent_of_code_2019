use std::env;

mod d6q2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let result = d6q2::process(input_file);

    match result {
        Ok(()) => println!(""),
        Err(e) => println!("{}", e)
    }
}
