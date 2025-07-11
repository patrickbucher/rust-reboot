use std::io::stdin;

fn main() {
    println!("What is 3+2? Type your answer and press enter.");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Unable to read standard input");

    if input == "5" /* won't match; newline is part of the string! */ {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}
