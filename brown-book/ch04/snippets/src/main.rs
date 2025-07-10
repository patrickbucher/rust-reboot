fn take_mutable_ownership(mut s: String) {
    s.push_str(".");
    println!("{s}");
}

fn take_ownership(s: String) {
    println!("{s}.");
}

fn take_mutable_reference(s: &mut String) {
    s.push_str(".");
    println!("{s}");
}

fn take_reference(s: &String) {
    println!("{s}.");
}

fn main() {
    let mut a = String::from("a");
    let mut b = String::from("b");
    let mut c = String::from("c");
    let mut d = String::from("d");

    take_mutable_ownership(a);
    take_ownership(b);
    take_mutable_reference(&mut c);
    take_reference(&d);

    // println!("{a}"); // fails
    // println!("{b}"); // fails
    println!("{c}"); // ok
    println!("{d}"); // ok
}
