// Rust-101, Part 02: Generic types, Traits
// ========================================

// ## Generic datatypes
// #[derive(Clone)]
pub enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}
use std::fmt::Display;

// Instead of writing out all the variants, we can also just import them all at once.
pub use self::SomethingOrNothing::*;
// type NumberOrNothing = SomethingOrNothing<i32>;

// ## Generic `impl`, Static functions
// Inside an `impl`, `Self` refers to the type we are implementing things for. Here, it is
// an alias for `SomethingOrNothing<T>`.
impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o {
            Some(t) => Something(t),
            None => Nothing,
        }
    }

    fn to_option(self) -> Option<T> {
        match self {
            Something(t) => Some(t),
            Nothing => None,
        }
    }
}
// You can call static functions, and in particular constructors, as demonstrated in `call_constructor`.
fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
    SomethingOrNothing::new(Some(x))
}

// ## Traits

pub trait Minimum: Copy {
    fn min(self, b: Self) -> Self;
}

pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            // Here, we can now call the `min` function of the trait.
            Something(n) => e.min(n),
        });
    }
    min
}

// ## Trait implementations
// To make `vec_min` usable with a `Vec<i32>`, we implement the `Minimum` trait for `i32`.
impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b {
            self
        } else {
            b
        }
    }
}

// We again provide a `print` function.
impl SomethingOrNothing<i32> {
    pub fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

impl<T: Display> Display for SomethingOrNothing<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Something(s) => write!(f, "Something({})", s),
            Nothing => write!(f, "Nothing"),
        }
    }
}

// Now we are ready to run our new code. Remember to change `main.rs` appropriately.
fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 3, 9, 27]
}

fn read_vec_f() -> Vec<f32> {
    vec![18., 5., 7., 3.1, 9., 27.]
}

pub fn main() {
    let vec = read_vec_f();
    let min = vec_min(vec);
    println!("{min}");
}

// **Exercise 02.1**: Change your program such that it computes the minimum of a `Vec<f32>` (where
// `f32` is the type of 32-bit floating-point numbers). You should not change `vec_min` in any
// way, obviously!

type FloatOrNothing = SomethingOrNothing<f32>;

impl Minimum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b {
            self
        } else {
            b
        }
    }
}
