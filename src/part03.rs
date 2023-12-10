// Rust-101, Part 03: Input
// ========================

// I/O is provided by the module `std::io`, so we first have to import that with `use`.
// We also import the I/O *prelude*, which makes a bunch of commonly used I/O stuff
// directly available.
use std::io::prelude::*;
use std::{io, str::FromStr};

fn read_vec<T: FromStr>() -> Vec<T> {
    let mut vec: Vec<T> = Vec::<T>::new();
    // The central handle to the standard input is made available by the function `io::stdin`.
    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");
    for line in stdin.lock().lines() {
        // Rust's type for (dynamic, growable) strings is `String`. However, our variable `line`
        // here is not yet of that type: It has type `io::Result<String>`.

        // I chose the same name (`line`) for the new variable to ensure that I will never,
        // accidentally, access the "old" `line` again.
        let line = line.unwrap();
        // Now that we have our `String`, we want to make it an `i32`.

        match line.trim().parse::<T>() {
            Ok(num) => {
                vec.push(num);
            }
            // We don't care about the particular error, so we ignore it with a `_`.
            Err(_) => {
                println!("Input numbers");
            }
        }
    }

    vec
}

// For the rest of the code, we just re-use part 02 by importing it with `use`.
use part02::{vec_min, Nothing, Something, SomethingOrNothing};

// If you update your `main.rs` to use part 03, `cargo run` should now ask you for some numbers,
// and tell you the minimum. Neat, isn't it?
pub fn main() {
    let vec = read_vec::<i32>();
    let min = vec_min(vec);
    // min.print2();
    println!("{min}");
}

// **Exercise 03.1**: The goal is to write a generic version of `SomethingOrNothing::print`.
// To this end, define a trait `Print` that provides (simple) generic printing, and implement
// that trait for `i32`. Then define `SomethingOrNothing::print2` to use that trait, and change
// `main` above to use the new generic `print2` function.
// I will again provide a skeleton for this solution. It also shows how to attach bounds to generic
// implementations (just compare it to the `impl` block from the previous exercise).
// You can read this as "For all types `T` satisfying the `Print` trait, I provide an implementation
// for `SomethingOrNothing<T>`".
//
// Notice that I called the function on `SomethingOrNothing` `print2` to disambiguate from the
// `print` defined previously.
//
// *Hint*: There is a macro `print!` for printing without appending a newline.
pub trait Print {
    fn print(self);
}

impl Print for i32 {
    /* Add things here */
    fn print(self) {
        print!("The i32 is: {}", self);
    }
}

impl<T: Print> SomethingOrNothing<T> {
    fn print2(self) {
        match self {
            Something(s) => {
                s.print();
                println!();
            }
            Nothing => println!("There is nothing."),
        }
    }
}

// **Exercise 03.2**: Building on exercise 02.2, implement all the things you need on `f32` to make
// your program work with floating-point numbers.

impl Print for f32 {
    fn print(self) {
        print!("The f32 is: {}", self);
    }
}
