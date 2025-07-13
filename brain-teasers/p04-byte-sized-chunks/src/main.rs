fn main() {
    let mut counter: i8 = 0;
    loop {
        println!("{}", counter);
        counter += 1;
    }
}

// guess: output from 0..127, -128..127, etc.
// wrong: 0..127, overflow (in debug mode)
