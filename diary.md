# Day 22 (2025-07-26)

I've been working through the section on Hash Maps today.

The reason I stopped working on Elixir was that I was unable to focus on the introductory book. You cannot step into the same stream twice: working through a tutorial you're already familiar with is hard because you subconciously know that you're faking it.

Now I realized that I have the same issue with Rust and the Brown Book. It's just not motivating, and I'd rather read some of my old code than work on this tutorial. So I have to pick a new challenge for August; this one isn't working out.

# Day 21 (2025-07-25)

Finally found some time to start chapter 8 on collections. I worked through the section about vectors in the morning, and the section about strings in the evening.

# Day 20 (2025-07-24)

Same excuse, different day.

# Day 19 (2025-07-23)

On vacation now, no time for Rust today.

# Day 18 (2025-07-22)

Before heading off to the airport, I started working on chapter 7 of the Brown Book, which is about managing Rust projects. A few notes:

- _Crate_: the smallest amount of code the Rust compiler considers at a time. Crates…
    - can be as small as a single file.
    - can contain modules.
    - have a _root_, i.e. a source file from which compilation is started.
    - can either be a binary (root: `src/main.rs`) or a library (root: `src/lib.rs`).
- _Package_: a bundle of one or more crates. Packages…
    - contain a `Cargo.toml` file containing the package's build instructions.
    - must contain at least one crate.
    - can contain multiple binary crates but only up to one library crate.
- _Module_: a means to organize the code within a crate and define access rules to its elements. Modules…
    - are declared using the `mod` keyword and a name, i.e. `mod foobar`.
    - are looked up under `src/[name].rs` (new style) and then under `src/[name]/mod.rs` (old style).
    - can contain submodules, which can be declared within the same file, or in `src/[module]/[submodule].rs` and `src/[module]/[submodule]/mod.rs`.
    - can be referred to as `crate::[module]::[submodule]::[item]`.
    - have access rules:
        - By default, declared using `mod`, a module is private, and its elements cannot be seen by its parent.
        - By declaring the module using `pub mod`, or individual elements within the module using `pub`, the module or its element, respectively, are visible to its parent.
        - Modules being declared as public do not automatically expose their elements as public.
    - Absolute paths begin with `crate`, relative paths with `self`, `super`, or an identifier within the current module.
    - A enum declared as public exposes all of its variants as public without further declarations.
    - The fields of a struct must be declared as public explicitly for access from the outside, even if the struct itself is already declared public.
    - Names can be brought into scope by the `use` keyword, which requires a path to the symbol to be brought into scope. The `as` keyword allows for alias names.
        - e.g. `use std::io::Result as IoResult;`
    - Names can be re-exported using the `pub` and `use` keywords in conjunction.
        - e.g. `pub use crates::utils::strip;`
        - e.g. `pub use crates::utils::strip as trim;`

That's it for chapter 7.

# Day 17 (2025-07-21)

Finally some Rust again after feeling better, and after a day full of Ruby programming. I solved puzzle 7 from the brain teasers book.

Travelling day tomorrow, so I expect nothing more than a puzzle. But the day after tomorrow, I'll probably be sitting around in my hotel room with some more time for Rust.

# Day 16 (2025-07-20)

Still sick; but getting better.

# Day 15 (2025-07-19)

Still sick; still no Rust.

# Day 14 (2025-07-18)

No Rust today; I'm simply too busy, tired, and sick, unfortunately.

# Day 13 (2025-07-17)

Little time again; but I managed to finish chapter 6 in the Brown Book, but without spending too much time with the "Ownership Inventory" quizzes. However, I got to know the `let`/`else` construct, which I wasn't familiar with before.

# Day 12 (2025-07-16)

Still suffering from the slight cold, but I started with chapter 6 (on enums) nonetheless. I'd like to finish it tomorrow.

# Day 11 (2025-07-15)

Little time again, and a slight cold. So I just worked through the 6th brain teaser, on floating point arithmetic, that is.

# Day 10 (2025-07-14)

 Almost no time for Rust today; I just worked through the 5th brain teaser on unicode strings.

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
