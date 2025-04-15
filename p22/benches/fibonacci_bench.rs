#![feature(test)] // Enable nightly feature for bencher

extern crate test;
use p22::calc;
use test::Bencher;

#[bench]
fn bench_fibonacci_loop(b: &mut Bencher) {
    b.iter(|| {
        // Test with n=20 as a reasonable input
        calc::fibonacci_loop(20)
    });
}

#[bench]
fn bench_fibonacci_rec(b: &mut Bencher) {
    b.iter(|| {
        // Same input for fair comparison
        calc::fibonacci_rec(20)
    });
}
