// Rust-101, Part 15: Mutex, Interior Mutability (cont.), RwLock, Sync
// ===================================================================

use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// The derived `Clone` implementation will clone the `Arc`, so all clones will actually talk about
// the same counter.
#[derive(Clone)]
struct ConcurrentCounter(Arc<RwLock<usize>>);

impl ConcurrentCounter {
    // The constructor just wraps the constructors of `Arc` and `RwLock`.
    pub fn new(val: usize) -> Self {
        ConcurrentCounter(Arc::new(RwLock::new(val)))
    }

    // The core operation is, of course, `increment`.
    pub fn increment(&self, by: usize) {
        // `lock` on a mutex returns a guard, very much like `RefCell`. The guard gives access to
        // the data contained in the mutex.
        let mut counter = self.0.write().unwrap_or_else(|err| err.into_inner());
        *counter = *counter + by;
    }

    // The function `get` returns the current value of the counter.
    pub fn get(&self) -> usize {
        let counter = self.0.read().unwrap();
        *counter
    }

    pub fn compare_and_inc(&self, test: usize, by: usize) {
        let mut counter = self.0.write().unwrap();
        if *counter == test {
            *counter += by;
            println!("Compare success: after inc: {}", *counter);
        }
    }
}

// Now our counter is ready for action.
pub fn main() {
    let counter = ConcurrentCounter::new(0);

    // We clone the counter for the first thread, which increments it by 2 every 15ms.
    let counter1 = counter.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(15));
            counter1.increment(2);
        }
    });

    // The second thread increments the counter by 3 every 20ms.
    let counter2 = counter.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(20));
            counter2.increment(3);
        }
    });

    let counter3 = counter.clone();
    let handle3 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(8));
            counter3.compare_and_inc(10, 5);
        }
    });

    // Now we watch the threads working on the counter.
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(10));
        println!("Current value: {}", counter.get());
    }

    // Finally, we wait for all the threads to finish to be sure we can catch the counter's final
    // value.
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    println!("Final value: {}", counter.get());

    let a = RefCell::new(1);
    a.borrow_mut();
}

// **Exercise 15.1**: Add an operation `compare_and_inc(&self, test: usize, by: usize)` that
// increments the counter by `by` *only if* the current value is `test`.
//
// **Exercise 15.2**: Rather than panicking in case the lock is poisoned, we can use `into_inner`
// on the error to recover the data inside the lock. Change the code above to do that. Try using
// `unwrap_or_else` for this job.

// **Exercise 15.3**:  Change the code above to use `RwLock`, such that multiple calls to `get` can
// be executed at the same time.
