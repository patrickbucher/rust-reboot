const HELLO_WORLD: &'static str = "Helló heimur";
fn main() {
    println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.len())
}

// NOTE: 13 bytes are not 13 characters! 'ó' is a two-byte character. len() measures bytes.
