// Rust-101, Part 08: Associated Types, Modules
// ============================================

use part05::BigInt;
use std::{cmp, ops, result};

// So, let us write a function to "add with carry", and give it the appropriate type. Notice Rust's
// native support for pairs.
fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let sum = a.wrapping_add(b);
    // If an overflow happened, then the sum will be smaller than *both* summands. Without an
    // overflow, of course, it will be at least as large as both of them. So, let's just pick one
    // and check.
    if sum >= a {
        // The addition did not overflow. <br/>
        // **Exercise 08.1**: Write the code to handle adding the carry in this case.
        if carry {
            let sum2 = sum.wrapping_add(1);
            if sum2 >= sum {
                (sum2, false)
            } else {
                (sum2, true)
            }
        } else {
            (sum, false)
        }
    } else {
        // Otherwise, the addition *did* overflow. It is impossible for the addition of the carry
        // to overflow again, as we are just adding 0 or 1.
        (sum + if carry { 1 } else { 0 }, true)
    }
}

// `overflow_add` is a sufficiently intricate function that a test case is justified.
// This should also help you to check your solution of the exercise.
#[test]
fn test_overflowing_add() {
    assert_eq!(overflowing_add(10, 100, false), (110, false));
    assert_eq!(overflowing_add(10, 100, true), (111, false));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
    assert_eq!(overflowing_add(1 << 63, (1 << 63) - 1, true), (0, true));
}

// ## Associated Types
impl ops::Add for BigInt {
    // Here, we choose the result type to be again `BigInt`.
    type Output = BigInt;

    // Now we can write the actual function performing the addition.
    fn add(self, rhs: Self) -> Self::Output {
        // We know that the result will be *at least* as long as the longer of the two operands,
        // so we can create a vector with sufficient capacity to avoid expensive reallocations.
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut result_vec: Vec<u64> = Vec::with_capacity(max_len);
        let mut carry = false; /* the current carry bit */
        for i in 0..max_len {
            let lhs_val = if i < self.data.len() { self.data[i] } else { 0 };
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            // Compute next digit and carry. Then, store the digit for the result, and the carry
            // for later.
            let (sum, new_carry) = overflowing_add(lhs_val, rhs_val, carry);
            result_vec.push(sum);
            carry = new_carry;
        }
        // **Exercise 08.2**: Handle the final `carry`, and return the sum.
        if carry {
            result_vec.push(1);
        }
        BigInt::from_vec(result_vec)
    }
}

// ## Traits and reference types

// Writing this out becomes a bit tedious, because trait implementations (unlike functions) require
// full explicit annotation of lifetimes. Make sure you understand exactly what the following
// definition says. Notice that we can implement a trait for a reference type!
impl ops::Add<&BigInt> for &BigInt {
    type Output = BigInt;
    fn add(self, rhs: &BigInt) -> Self::Output {
        // **Exercise 08.3**: Implement this function.
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut result_vec = Vec::with_capacity(max_len);
        let mut carry = false;
        for i in 0..max_len {
            let lhs_val = if i < self.data.len() { self.data[i] } else { 0 };
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            let (sum, new_carry) = overflowing_add(lhs_val, rhs_val, carry);
            result_vec.push(sum);
            carry = new_carry;
        }
        if carry {
            result_vec.push(1);
        }
        BigInt::from_vec(result_vec)
    }
}

// **Exercise 08.4**: Implement the two missing combinations of arguments for `Add`. You should not
// have to duplicate the implementation.
impl ops::Add<&Self> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl ops::Add<BigInt> for &BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        self + &rhs
    }
}

// ## Modules

// Rust calls a bunch of definitions that are grouped together a *module*. You can put the tests in
// a submodule as follows.
#[cfg(test)]
mod tests {
    use super::*;
    use part05::BigInt;

    #[test]
    fn test_add() {
        let b1 = BigInt::new(1 << 32);
        let b2 = BigInt::from_vec(vec![0, 1]);

        assert_eq!(&b1 + &b2, BigInt::from_vec(vec![1 << 32, 1]));
        assert_eq!(b1.clone() + &b2, BigInt::from_vec(vec![1 << 32, 1]));
        assert_eq!(&b1 + b2.clone(), BigInt::from_vec(vec![1 << 32, 1]));
        assert_eq!(b1 + b2, BigInt::from_vec(vec![1 << 32, 1]));
        // **Exercise 08.5**: Add some more cases to this test.

        let b3 = BigInt::new(1 << 63);
        let b4 = BigInt::from_vec(vec![1 << 63, 1]);
        let b5 = BigInt::from_vec(vec![(1 << 63) - 1]);
        let b6 = BigInt::from_vec(vec![(1 << 63) + 1]);
        assert_eq!(&b3 + &b4, BigInt::from_vec(vec![0, 2]));
        assert_eq!(&b3 + &b5, BigInt::from_vec(vec![(1 << 63) - 1 + (1 << 63)]));
        assert_eq!(&b3 + &b6, BigInt::from_vec(vec![1, 1]));

        // test mod
        assert_eq!(part00::ttt(), 1);
    }
}

// test mod
mod part00;

// **Exercise 08.6**: Write a subtraction function, and testcases for it. Decide for yourself how
// you want to handle negative results. For example, you may want to return an `Option`, to panic,
// or to return `0`.

impl ops::Sub for BigInt {
    type Output = Option<BigInt>;
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}
impl ops::Sub<BigInt> for &BigInt {
    type Output = Option<BigInt>;
    fn sub(self, rhs: BigInt) -> Self::Output {
        self - &rhs
    }
}
impl ops::Sub<&BigInt> for BigInt {
    type Output = Option<BigInt>;
    fn sub(self, rhs: &BigInt) -> Self::Output {
        &self - rhs
    }
}

impl ops::Sub<&BigInt> for &BigInt {
    type Output = Option<BigInt>;
    fn sub(self, rhs: &BigInt) -> Self::Output {
        if rhs.data.len() > self.data.len() {
            return None;
        }
        let mut carry = false;
        let mut result_vec = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            let lhs_val = self.data[i];
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };

            let (diff, new_carry) = overflow_sub(lhs_val, rhs_val, carry);
            result_vec.push(diff);
            carry = new_carry;
        }
        if carry {
            return None;
        }
        Some(BigInt::from_vec(result_vec))
    }
}

fn overflow_sub(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let diff = a.wrapping_sub(b);
    if diff <= a {
        // no overflow
        if carry {
            let diff2 = diff.wrapping_sub(1);
            if diff2 <= diff {
                // no overflow
                (diff2, false)
            } else {
                // overflow
                (diff2, true)
            }
        } else {
            (diff, false)
        }
    } else {
        // overflow
        (diff - if carry { 1 } else { 0 }, true)
    }
}

#[test]
fn test_overflowing_sub() {
    assert_eq!(overflow_sub(3, 2, false), (1, false));
    assert_eq!(overflow_sub(3, 2, true), (0, false));
    assert_eq!(overflow_sub(3, 3, false), (0, false));
    assert_eq!(overflow_sub(3, 3, true), ((1 << 63) - 1 + (1 << 63), true));
}

#[test]
fn test_sub() {
    let max = (1 << 63) - 1 + (1 << 63);
    let b1 = BigInt::from_vec(vec![0, 0, 1]);
    let b2 = BigInt::from_vec(vec![1]);
    assert_eq!(
        b1.clone() - b2.clone(),
        Some(BigInt::from_vec(vec![max, max]))
    );
    assert_eq!(b1.clone() - &b2, Some(BigInt::from_vec(vec![max, max])));
    assert_eq!(&b1 - b2.clone(), Some(BigInt::from_vec(vec![max, max])));
    assert_eq!(b1 - b2, Some(BigInt::from_vec(vec![max, max])));

    let b3 = BigInt::from_vec(vec![1, 1]);
    let b4 = BigInt::from_vec(vec![1]);
    let b5 = BigInt::from_vec(vec![0]);
    let b6 = BigInt::from_vec(vec![1,2]);
    assert_eq!(&b3 - &b4, Some(BigInt::from_vec(vec![0, 1])));
    assert_eq!(&b3 - &b5, Some(BigInt::from_vec(vec![1, 1])));
    assert_eq!(&b3 - &b6, None);
}
