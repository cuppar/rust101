// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers

#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            Self { data: vec![] }
        } else {
            Self { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        // if self.data.len() == 0 {
        //     true
        // } else {
        //     self.data[self.data.len() - 1] != 0
        // }
        self.data.iter().last().map(|n| *n != 0).unwrap_or(true)
    }

    // We can convert any little-endian vector of digits (i.e., least-significant digit first) into
    // a number, by removing trailing zeros. The `mut` declaration for `v` here is just like the
    // one in `let mut ...`: We completely own `v`, but Rust still asks us to make our intention of
    // modifying it explicit. This `mut` is *not* part of the type of `from_vec` - the caller has
    // to give up ownership of `v` anyway, so they don't care anymore what you do to it.
    //
    // **Exercise 05.1**: Implement this function.
    //
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        if v.is_empty() {
            return Self { data: vec![] };
        }
        while !v.is_empty() && v[v.len() - 1] == 0 {
            v.pop();
        }
        Self { data: v }
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0, 1 << 16];
    let b1 = BigInt::from_vec(v.clone());
    let b2 = BigInt::from_vec(v);
}

// impl Clone for BigInt {
//     fn clone(&self) -> Self {
//         Self {
//             data: self.data.clone(),
//         }
//     }
// }

// We can also make the type `SomethingOrNothing<T>` implement `Clone`.
use part02::{Nothing, Something, SomethingOrNothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match self {
            Something(v) => Something(v.clone()),
            Nothing => Nothing,
        }
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the
// number of digits? The number of non-zero digits? The smallest/largest digit? Of course, these
// should all take `self` as a shared reference (i.e., in borrowed form).

impl BigInt {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn non_zero_digits(&self) -> usize {
        let mut count = 0;
        for e in self.data.iter() {
            if *e != 0 {
                count += 1;
            }
        }
        count
    }

    pub fn smallest(&self) -> Option<u64> {
        if self.data.is_empty() {
            return None;
        }
        let mut min = self.data[0];
        for e in self.data.iter() {
            if *e < min {
                min = *e;
            }
        }
        Some(min)
    }

    pub fn largest(&self) -> Option<u64> {
        if self.data.is_empty() {
            return None;
        }
        let mut max = self.data[0];
        for e in self.data.iter() {
            if *e > max {
                max = *e;
            }
        }
        Some(max)
    }
}

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}
fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */
    /* BAD! */
    *ptr = 1337;
}
