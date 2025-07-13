# Day 9 (2025-07-13)

Little time today, so I just worked through the 4th Brain Teaser about integer overflows.

# Day 8 (2025-07-12)

I worked through chapter 5 today, which is about structs. It was rather easy; I also managed to get all the quizzes right for once.

# Day 7 (2025-07-11)

Another unproductive day: I just managed to work through puzzle 3 in Rust Brain Teasers. It's about type conversions.

# Day 6 (2025-07-10)

I finished chapter 4 today. I got some quiz questions wrong, but mostly the hypothecical ones ("Would there be any undefined behaviour if the Rust compiler accepted the following program?"). I'm happy with my current understanding of the ownership principles and rules. First, a few definitions:

- _References_: non-owning pointers.
- _Aliasing_: accessing the same data through different variables.
- _Pointer Safety Principle_: data should never be aliased and mutated at the same time.

At compile-time, there are the following variable permissions (with respective allowed operations):

- `r`: read (copy)
- `w`: write (mutate)
- `o`: own (move & drop)

Bindings have the following permissions:

- `let`: `ro`
- `let mut`: `rwo`

References modify the permissions as follows:

| Reference | Original   | Reference        |
|-----------|------------|------------------|
| immutable | `-w`, `-o` | `+r`             |
| mutable   | `-w`, `-o` | `+r`, `+w`, `+o` |

Borrowing from a variable temporarely drops its `o` permission.

Functions input/output references are treated differently in terms of permissions than variables within a function body: they make use of the _flow permission_ `f` (which, unfortunately, is not further explained in this chapter).

The syntax is best explained by this code snippets demonstrating function invocations using both references and passing of ownership; both mutably and immutabely:

```rust
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

    println!("{a}"); // FIXME: fails
    println!("{b}"); // FIXME: fails
    println!("{c}"); // ok
    println!("{d}"); // ok
}
```

Now I'm looking forward to the next chapter on structs.

# Day 5 (2025-07-09)

I got a little further into chapter 4 of the Brown Book, but I am still not finished. I think this chapter is just belaboring the point. Maybe I should just review my old code instead, but let's stick to the plan for a while.

# Day 4 (2025-07-08)

No progress on the Brown Book today; I only had time to work on the first two puzzles of _Rust Brain Teasers_, which I bought a while ago but never bothered to read.

# Day 3 (2025-07-07)

Today, I'm working through chapter 4 of the Brown Book, which is about _ownership_. A few notes:

- Assigning a heap variable moves its ownership to a new destination. E.g. `let a = Box::new(3); let b = a;` moves the ownership of the `Box` containing the value `3` from `a` to `b`. At this point, `b` is the only valid reference to said value; `a` must no longer be used. Thanks to this single-ownerhip principle, boxes can be deallocated automatically at the end of their pointer's scope without the risk of a _double free_ memory operation.

# Day 2 (2025-07-06)

Working through chapter 3 of the Brown Book showed me the value of the added quizzes: Going through the introductory material is mostly about re-activating the knowledge I already acquired a year ago; but once I get quiz answers wrong, I have to dig deeper.

I worked through chapter 3 and did all the exercises, which are rather helpful at this stage for warming up again.

# Day 1 (2025-07-05)

I worked through chapter 1 of the Brown University's version of the Rust Book, which henceforward I'll be calling the _Brown Book_.

I also replaced my native Rust setup from Arch Linux with a `rustup`-based setup, which `rustup` coming in as an Arch Linux package. This way, I don't have to execute shell code coming unvetted from the internet, but still can use `rustup` and its components, e.g. the local Rust documentation.

In the afternoon, I also worked through chapter 2.
