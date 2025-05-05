// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::{sync::Arc, thread};

pub fn sum(v: Vec<i32>) -> i32 {
    let l = Arc::new(v.len()/2);
    let v = Arc::new(v);

    let l2 = l.clone();
    let v2 = v.clone();
    let th1: thread::JoinHandle<i32> = thread::spawn(move || {
        let first_half: &[i32] = &v[0..*l];
        first_half.iter().sum()
    });

    let th2: thread::JoinHandle<i32> = thread::spawn(move || {
        let second_half: &[i32] = &v2[*l2..];
        second_half.iter().sum()
    });

    let r1 = th1.join().unwrap();
    let r2 = th2.join().unwrap();
    r1 + r2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
