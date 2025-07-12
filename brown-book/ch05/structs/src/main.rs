#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u16,
    published: i16,
}

impl Book {
    fn describe(&self) -> String {
        format!(
            "{} by {}, {}p. ({})",
            self.title, self.author, self.pages, self.published
        )
    }

    fn is_longer_than(&self, other: &Book) -> bool {
        self.pages > other.pages
    }
}

#[derive(Debug)]
struct Position(f32, f32);

fn main() {
    let crime_and_punishment = Book {
        title: String::from("Crime and Punishment"),
        author: String::from("Fyodor M. Dostoevsky"),
        pages: 732,
        published: 1866,
    };
    let brothers_karamazov = Book {
        title: String::from("The Brothers Karamazov"),
        author: String::from("Fyodor M. Dostoevsky"),
        pages: 1234,
        published: 1880,
    };
    println!("{}", crime_and_punishment.describe());
    println!("{}", brothers_karamazov.describe());
    println!(
        "Is {} longer than {}? {}",
        brothers_karamazov.title,
        crime_and_punishment.title,
        brothers_karamazov.is_longer_than(&crime_and_punishment)
    );

    let menznau = Position(47.08333, 8.039441);
    let kuleshovka = Position(47.079167, 39.5375);
    println!("{menznau:?}");
    println!("{kuleshovka:#?}");

    dbg!(menznau);
    dbg!(&kuleshovka);
}
