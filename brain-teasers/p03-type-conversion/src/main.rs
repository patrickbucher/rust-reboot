fn main() {
    let x: u64 = 4_294_967_296; // NOTE: 2^32
    let y = x as u32; // NOTE: max u32 is 2^32-1, truncated!
    if x == y as u64 {
        println!("x equals y.");
    } else {
        println!("x does not equal y.");
    }
}
