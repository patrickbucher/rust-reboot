fn main() {
    // Exercise 1
    let fahrenheit: f64 = 100.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}°F = {celsius}°C");
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius}°C = {fahrenheit}°F");

    // Exercise 2
    let n = 40;
    let n_th_fibonacci_number = fib(40);
    println!("the {n}th Fibonacci number is {n_th_fibonacci_number}");

    // Exercise 3
    twelve_days_of_christmas();
}

fn twelve_days_of_christmas() {
    let gifts = [
        ("first", "A partridge in a pear tree"),
        ("second", "Two turtle doves"),
        ("third", "Three French hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five gold rings"),
        ("sixth", "Six geese a-laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eighth", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelvth", "Twelve drummers drumming"),
    ];
    let mut i = 0;
    while i < gifts.len() {
        let day = gifts[i].0;
        println!("On the {day} day of Christmas my true love sent to me");
        for j in (0..=i).rev() {
            let gift = gifts[j].1;
            if j == 0 && i > 0 {
                println!("And {}", gift.to_lowercase());
            } else {
                println!("{gift}");
            }
        }
        println!("");
        i += 1;
    }
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    ((fahrenheit - 32.0) * 5.0) / 9.0
}
