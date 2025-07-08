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
