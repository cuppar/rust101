// Rust-101, Part 09: Iterators
// ============================

use part05::BigInt;

pub struct Iter<'a> {
    num: &'a BigInt,
    idx: usize, // the index of the last number that was returned
}

// Now we are equipped to implement `Iterator` for `Iter`.
impl<'a> Iterator for Iter<'a> {
    // We choose the type of things that we iterate over to be the type of digits, i.e., `u64`.
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // First, check whether there's any more digits to return.
        if self.idx == 0 {
            // We already returned all the digits, nothing to do.
            None
        } else {
            // Otherwise: Decrement, and return next digit.
            self.idx -= 1;
            Some(self.num.data[self.idx])
        }
    }
}

// All we need now is a function that creates such an iterator for a given `BigInt`.
impl BigInt {
    fn iter(&self) -> Iter {
        Iter {
            num: self,
            idx: self.data.len(),
        }
    }
}

// We are finally ready to iterate! Remember to edit `main.rs` to run this function.
pub fn main() {
    let b = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    for digit in &b {
        println!("{}", digit);
    }
}

// Of course, we don't have to use `for` to apply the iterator. We can also explicitly call `next`.
fn print_digits_v1(b: &BigInt) {
    let mut iter = b.iter();
    loop {
        // Each time we go through the loop, we analyze the next element presented by the iterator
        // - until it stops.
        unimplemented!()
    }
}

fn print_digits_v2(b: &BigInt) {
    let mut iter = b.iter();
    while let Some(digit) = iter.next() {
        println!("{}", digit)
    }
}

// **Exercise 09.1**: Write a testcase for the iterator, making sure it yields the corrects numbers.
//
#[test]
fn test_iter() {
    let b = BigInt::from_vec(vec![1, 2, 3]);
    let mut iter = b.iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}
// **Exercise 09.2**: Write a function `iter_ldf` that iterates over the digits with the
// least-significant digits coming first. Write a testcase for it.

pub struct IterLdf<'a> {
    num: &'a BigInt,
    idx: usize,
}

impl<'a> Iterator for IterLdf<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.num.data.len() {
            None
        } else {
            let idx = self.idx;
            self.idx += 1;
            Some(self.num.data[idx])
        }
    }
}

impl BigInt {
    fn iter_ldf(&self) -> IterLdf {
        IterLdf { num: self, idx: 0 }
    }
}

#[test]
fn test_iter_ldf() {
    let b = BigInt::from_vec(vec![1, 2, 3]);
    let mut iter = b.iter_ldf();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

// ## Iterator invalidation and lifetimes

fn iter_invalidation_demo() {
    let mut b = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    for digit in b.iter() {
        println!("{}", digit);
        /*b = b + BigInt::new(1);*/ /* BAD! */
    }
}

// ## Iterator conversion trait

impl<'a> IntoIterator for &'a BigInt {
    type Item = u64;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}
// With this in place, you can now replace `b.iter()` in `main` by `&b`. Go ahead and try it! <br/>
