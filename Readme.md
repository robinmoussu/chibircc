# Chibircc: A Small C Compiler written in Rust

This project takes [chibicc](https://github.com/rui314/chibicc) and re-write it in Rust. `chibicc` is a small C compiler that implements most C11 features.  `chibicc` is developed as the reference implementation for a book its author is currently writing about the C compiler low-level programming. Each chapter implements one feature at a time, until the language that the compiler accepts matches the C11 spec specifies. Each commit in this project correspond to one commit on the `chibicc` repository, and themselves correspond to a chapter.

If you like this project, I encourage you to purchase a copy of the book written by the author of `chibicc` once it becomes available!

"chibi" means mini or "small" in Japaness, an "cc" stands for the C compiler, according to the author of `chibicc`. `chibircc` is just a Rust translation of that tini C compiler.

Just like the original project, please assume that I will probably rebase and force push changes, at minimum to be able to match any change on the history of `chibbi`.

## Attribution

This project is based on Rui Ueyama work on [chibicc](https://github.com/rui314/chibicc) under the same license.

