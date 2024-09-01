/*
    Appellation: default <bench>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![feature(test)]
extern crate test;

use test::Bencher;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: usize = 20;

// function to benchmark must be annotated with `#[bench]`
#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    b.iter(|| (0..BENCH_SIZE).map(fib::fibonacci).collect::<Vec<_>>())
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| fib::Fibonacci::seq().take(BENCH_SIZE).collect::<Vec<_>>())
}

pub mod fib {

    /// [fibonacci] calculates the nth term of the fibonacci sequence using recursion; this
    /// implementation is not recommended for large values of n and is simply used as a
    /// benchmark.
    pub fn fibonacci(n: usize) -> u32 {
        if n < 2 {
            1
        } else {
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }

    /// [Fibonacci] implements the fibonacci sequence as an [Iterator]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Fibonacci {
        pub curr: u32,
        next: u32,
    }

    impl Fibonacci {
        pub fn new(curr: u32, next: u32) -> Self {
            Self { curr, next }
        }

        pub fn seq() -> Self {
            Self::new(1, 1)
        }

        pub fn get(&self) -> u32 {
            self.curr
        }

        pub fn get_next(&self) -> u32 {
            self.next
        }
    }

    impl Default for Fibonacci {
        fn default() -> Self {
            Self { curr: 1, next: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            use core::mem::replace;
            let next = self.get() + self.next;
            let curr = replace(&mut self.next, next);

            Some(replace(&mut self.curr, curr))
        }
    }
}
