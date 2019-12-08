mod d3q3;

fn main() {
    println!("Hello, world!");
    let result = d3q1::process("day3_input.txt");

    println!("--------------");
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e)
    }
}
